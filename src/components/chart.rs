use super::motion::next_id;
use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChartKind {
    #[default]
    Bar,
    Line,
    Area,
    Donut,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChartIndicator {
    #[default]
    Dot,
    Line,
    Dashed,
}
#[derive(Clone, PartialEq)]
pub struct ChartSeries {
    pub key: String,
    pub label: String,
    pub color: String,
    pub dashed: bool,
}
impl ChartSeries {
    pub fn new(key: impl Into<String>, label: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            color: color.into(),
            dashed: false,
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct ChartPoint {
    pub label: String,
    pub values: Vec<Option<f64>>,
}
impl ChartPoint {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            label: label.into(),
            values: values
                .into_iter()
                .map(|v| v.is_finite().then_some(v))
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct ChartTooltipItem {
    pub label: String,
    pub value: String,
    pub color: String,
}
#[component]
pub fn ChartTooltipContent(
    label: String,
    items: Vec<ChartTooltipItem>,
    #[props(default)] indicator: ChartIndicator,
    #[props(default)] hide_label: bool,
    #[props(default)] hide_indicator: bool,
) -> Element {
    let kind = match indicator {
        ChartIndicator::Dot => "dot",
        ChartIndicator::Line => "line",
        ChartIndicator::Dashed => "dashed",
    };
    rsx! {div{class:"m3-chart-tooltip",role:"tooltip",if !hide_label{strong{class:"m3-chart-tooltip__label","{label}"}}div{class:"m3-chart-tooltip__items",for item in items{div{class:"m3-chart-tooltip__row",if !hide_indicator{span{class:"m3-chart-tooltip__indicator",aria_hidden:"true","data-indicator":kind,style:"--chart-color:{item.color};"}}span{class:"m3-chart-tooltip__name","{item.label}"}span{class:"m3-chart-tooltip__value","{item.value}"}}}}}}
}
#[component]
pub fn ChartLegend(series: Vec<ChartSeries>) -> Element {
    rsx! {div{class:"m3-chart-legend",for s in series{span{class:"m3-chart-legend__entry",span{class:"m3-chart-legend__swatch",aria_hidden:"true",style:"background:{s.color};"},"{s.label}"}}}}
}
#[component]
pub fn ChartContainer(
    label: String,
    #[props(default)] description: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {figure{class:"m3-chart {class}",aria_label:label.clone(),figcaption{class:"m3-chart__sr","{label}" if let Some(text)=description{". {text}"}},{children}}}
}
fn axis_value(v: f64) -> String {
    if v.abs() >= 1_000_000_000.0 {
        format!("{v:.1e}")
    } else if v.abs() >= 1_000_000.0 {
        format!("{:.1}M", v / 1_000_000.0)
    } else if v.abs() >= 1000.0 {
        format!("{:.1}k", v / 1000.0)
    } else {
        format!("{v:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
fn finite_value(point: &ChartPoint, index: usize) -> Option<f64> {
    point
        .values
        .get(index)
        .copied()
        .flatten()
        .filter(|v| v.is_finite())
}

/// Native SVG chart composition. Values follow series order. Donut takes one
/// data point containing non-negative category values. Missing/nonfinite values
/// stay missing (line gaps), never become zero. Raw values remain in the table.
/// Legend actions read current visibility and preserve one visible series.
#[component]
pub fn Chart(
    label: String,
    series: Vec<ChartSeries>,
    data: Vec<ChartPoint>,
    #[props(default)] kind: ChartKind,
    #[props(default)] description: Option<String>,
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    #[props(default)] onselect: EventHandler<usize>,
) -> Element {
    let id = use_hook(|| next_id("m3-chart"));
    let mut dimensions = use_signal(|| (320.0, 200.0));
    let mut hidden = use_signal(Vec::<String>::new);
    let mut hover = use_signal(|| None::<usize>);
    let mut focus = use_signal(|| None::<usize>);
    let mut dismissed = use_signal(|| false);
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("chart.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(value) = eval.recv::<(f64, f64)>().await {
                    dimensions.set(value);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Charts?.[{id:?}]?.();")).await;
            });
        }
    });
    let visible = series
        .iter()
        .enumerate()
        .filter(|(_, s)| !hidden().contains(&s.key))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let donut = kind == ChartKind::Donut;
    let points = if donut {
        data.iter().take(1).cloned().collect::<Vec<_>>()
    } else {
        data.clone()
    };
    let invalid = donut
        && points.first().is_some_and(|p| {
            visible
                .iter()
                .any(|i| finite_value(p, *i).is_some_and(|v| v < 0.0))
        });
    let valid = points
        .iter()
        .any(|p| visible.iter().any(|i| finite_value(p, *i).is_some()));
    let area_overload = kind == ChartKind::Area && visible.len() > 2;
    let active = if dismissed() || disabled || invalid || area_overload {
        None
    } else {
        focus().or(hover()).filter(|i| *i < points.len())
    };
    let (w, h) = dimensions();
    let (w, h) = (w.max(160.0), h.max(120.0));
    let left = 44.0;
    let top = 16.0;
    let right = w - 12.0;
    let bottom = h - 28.0;
    let plotw = (right - left).max(1.0);
    let ploth = (bottom - top).max(1.0);
    let band = plotw / points.len().max(1) as f64;
    let all = points
        .iter()
        .flat_map(|p| visible.iter().filter_map(|i| finite_value(p, *i)))
        .collect::<Vec<_>>();
    let lo = all.iter().copied().fold(0.0, f64::min);
    let mut hi = all.iter().copied().fold(0.0, f64::max);
    if hi == lo {
        hi = lo + 1.0;
    }
    let scale = hi.abs().max(lo.abs()).max(1.0);
    let y = |v: f64| top + (hi / scale - v / scale) / (hi / scale - lo / scale) * ploth;
    let zero = y(0.0);
    let count = points.len();
    let tooltip_id = format!("{id}-tooltip");
    let total = points
        .first()
        .map(|p| {
            visible
                .iter()
                .filter_map(|i| finite_value(p, *i))
                .sum::<f64>()
        })
        .unwrap_or(0.0);
    let stride = ((count as f64 / ((plotw / 55.0).floor().max(1.0))).ceil() as usize).max(1);
    rsx! {ChartContainer{label:label.clone(),description,class,
        div{id:"{id}",class:"m3-chart__interactive",onkeydown:move|e|{if e.key()==Key::Escape{dismissed.set(true);}},
            div{class:"m3-chart__plot",onpointerleave:move |_|hover.set(None),
                if !valid||series.is_empty()||points.is_empty(){p{class:"m3-chart__empty",role:"status","No chart data."}}
                else if invalid{p{class:"m3-chart__empty",role:"status","Donut values must be non-negative."}}
                else if area_overload{p{class:"m3-chart__empty",role:"status","Use at most two overlaid area series. Use a stacked chart for more series."}}
                else if donut&&!total.is_finite(){p{class:"m3-chart__empty",role:"status","Donut total is outside the supported numeric range."}}
                else{svg{class:"m3-chart__svg",width:"100%",height:"100%",view_box:"0 0 {w} {h}","aria-hidden":"true",
                    if donut{
                        {let cx=w/2.0;let cy=h/2.0;let radius=(h.min(w)/2.0-24.0).max(10.0);let circumference=std::f64::consts::TAU*radius;let stroke=radius*0.4;let mut offset=0.0;
                        rsx!{
                            circle{cx,cy,r:radius,fill:"none",class:"m3-chart__donut-track",stroke_width:stroke}
                            if total>0.0{for i in visible.iter().copied(){
                                {let value=finite_value(&points[0],i).unwrap_or(0.0);let length=value/total*circumference;let begin=offset;offset+=length;let color=series[i].color.clone();rsx!{circle{key:"{series[i].key}",cx,cy,r:radius,fill:"none",stroke:color,stroke_width:stroke,stroke_dasharray:"{length} {circumference}",stroke_dashoffset:-begin,transform:"rotate(-90 {cx} {cy})",onpointerenter:move |_|{hover.set(Some(0));dismissed.set(false);}}}}
                            }}
                            text{x:cx,y:cy,text_anchor:"middle",dominant_baseline:"middle",class:"m3-chart__total","{axis_value(total)}"}
                            text{x:cx,y:cy+24.0,text_anchor:"middle",class:"m3-chart__axis-label","Total"}
                        }}
                    }else{
                        for tick in 0..=4{ {let fraction=tick as f64/4.0;let value=lo*(1.0-fraction)+hi*fraction;let pos=y(value);rsx!{g{line{x1:left,x2:right,y1:pos,y2:pos,class:"m3-chart__grid"}text{x:left-8.0,y:pos,text_anchor:"end",dominant_baseline:"middle",class:"m3-chart__axis-label","{axis_value(value)}"}}}}}
                        line{x1:left,x2:right,y1:zero,y2:zero,class:"m3-chart__baseline"}
                        if lo<0.0&&hi>0.0&&!(0..=4).any(|tick|{let f=tick as f64/4.0;(lo*(1.0-f)+hi*f).abs()<scale*1e-12}){text{x:left-8.0,y:zero,text_anchor:"end",dominant_baseline:"middle",class:"m3-chart__axis-label","0"}}
                        for (i,point) in points.iter().enumerate(){if i%stride==0||i==count-1 {text{x:left+band*(i as f64+0.5),y:h-8.0,text_anchor:"middle",class:"m3-chart__axis-label","{point.label}"}}}
                        for (sindex,index) in visible.iter().copied().enumerate(){
                            {let color=series[index].color.clone();let dash=if series[index].dashed{"6 4"}else{"none"};
                             let mut segments=Vec::<Vec<(f64,f64)>>::new();let mut segment=Vec::new();
                             for (i,point) in points.iter().enumerate(){if let Some(v)=finite_value(point,index){segment.push((left+band*(i as f64+0.5),y(v)));}else if !segment.is_empty(){segments.push(std::mem::take(&mut segment));}}
                             if !segment.is_empty(){segments.push(segment);}
                             rsx!{g{style:"color:{color};",
                                if kind==ChartKind::Bar {for (i,point) in points.iter().enumerate(){if let Some(value)=finite_value(point,index){
                                    {let barw=(band*0.75/visible.len().max(1) as f64).max(1.0);let x=left+band*i as f64+band*0.125+barw*sindex as f64;let py=y(value);rsx!{rect{x,y:py.min(zero),width:barw*0.9,height:(py-zero).abs(),rx:2,fill:"currentColor",onpointerenter:move |_|{hover.set(Some(i));dismissed.set(false);}}}}
                                }}}
                                else{for segment in segments{
                                    {let path=segment.iter().enumerate().map(|(i,(x,y))|format!("{} {x} {y}",if i==0{"M"}else{"L"})).collect::<Vec<_>>().join(" ");let first=segment[0].0;let last=segment.last().unwrap().0;
                                    rsx!{
                                        if kind==ChartKind::Area{path{d:"{path} L {last} {zero} L {first} {zero} Z",fill:"currentColor",fill_opacity:"0.18"}}
                                        path{d:path.clone(),fill:"none",stroke:"currentColor",stroke_width:2,stroke_dasharray:dash}
                                    }}
                                }
                                for (i,point) in points.iter().enumerate(){if let Some(value)=finite_value(point,index){
                                    {let x=left+band*(i as f64+0.5);let py=y(value);rsx!{g{onpointerenter:move |_|{hover.set(Some(i));dismissed.set(false);},
                                        if sindex%2==0{circle{cx:x,cy:py,r:4,fill:"currentColor"}}else{rect{x:x-4.0,y:py-4.0,width:8,height:8,fill:"currentColor"}}
                                    }}}
                                }}
                                }
                             }}
                            }
                        }
                        if let Some(i)=active {line{x1:left+band*(i as f64+0.5),x2:left+band*(i as f64+0.5),y1:top,y2:bottom,class:"m3-chart__cursor"}}
                    }
                }}
                if let Some(i)=active{div{id:tooltip_id.clone(),class:"m3-chart__tooltip-anchor",style:"left:{(left+band*(i as f64+0.5)-64.0).clamp(4.0,(w-164.0).max(4.0))}px;",ChartTooltipContent{label:points[i].label.clone(),items:visible.iter().map(|j|ChartTooltipItem{label:series[*j].label.clone(),value:finite_value(&points[i],*j).map(|v|v.to_string()).unwrap_or("No data".into()),color:series[*j].color.clone()}).collect::<Vec<_>>()}}}
            }
            div{class:"m3-chart-legend m3-chart-legend--controls",aria_label:"{label} series",for (i,s) in series.iter().enumerate(){
                {let key=s.key.clone();let keys=series.iter().map(|s|s.key.clone()).collect::<Vec<_>>();let shown=!hidden().contains(&key);rsx!{button{key:"{key}",r#type:"button",class:"m3-chart-legend__button",aria_label:"Show {s.label}",aria_pressed:shown,disabled:disabled||(shown&&visible.len()==1),onclick:move |_|{let mut next=hidden();if next.contains(&key){next.retain(|k|k!=&key);}else if keys.iter().filter(|k|!next.contains(k)).count()>1{next.push(key.clone());}hidden.set(next);},span{class:"m3-chart-legend__swatch",aria_hidden:"true",style:"background:{s.color};", "data-shape":if i%2==0{"circle"}else{"square"}},"{s.label}"}}}
            }}
            if valid&&!invalid&&!area_overload&&(!donut||total.is_finite()){div{class:"m3-chart__point-controls",aria_label:"Inspect chart data",for (i,point) in points.iter().enumerate(){button{key:"{i}-{point.label}",r#type:"button",class:"m3-chart__point",aria_label:"Inspect {point.label}","data-active":active==Some(i),aria_describedby:if active==Some(i){Some(tooltip_id.clone())}else{None},disabled,onfocus:move |_|{focus.set(Some(i));dismissed.set(false);},onblur:move |_|focus.set(None),onclick:move |_|{focus.set(Some(i));dismissed.set(false);onselect.call(i);},"{point.label}"}}}}
            details{class:"m3-chart__data",summary{"View {label} data"}div{class:"m3-chart__table-scroll",tabindex:"0",role:"region",aria_label:"{label} raw data",table{caption{"{label} — underlying values"}thead{tr{th{scope:"col","Category"}for s in &series{th{scope:"col","{s.label}"}}}}tbody{for point in &data{tr{th{scope:"row","{point.label}"}for i in 0..series.len(){td{ {finite_value(point,i).map(|v|v.to_string()).unwrap_or("No data".into())} }}}}}}}}
        }
    }}
}

use dioxus::prelude::*;
/// Compact upstream circular loading status. Use decorative=true when the
/// containing control/status already names the operation to avoid duplicates.
#[component]
pub fn Spinner(
    #[props(default="Loading".into())] label: String,
    #[props(default = 20)] size: u16,
    #[props(default)] decorative: bool,
    #[props(default)] inherit_color: bool,
    #[props(default)] class: String,
) -> Element {
    let size = size.max(1);
    let inherit_class = if inherit_color {
        "m3-spinner--inherit"
    } else {
        ""
    };
    rsx! {svg{class:"m3-spinner {inherit_class} {class}",style:"--m3-spinner-size:{size}px",view_box:"0 0 48 48",role:if decorative{None}else{Some("progressbar")},"aria-label":if decorative{None}else{Some(label)},"aria-hidden":if decorative{Some("true")}else{None},
    circle{class:"m3-spinner__arc",cx:"24",cy:"24",r:"20",fill:"none",stroke:"currentColor",stroke_width:"5",stroke_linecap:"round",path_length:"100"}
    }}
}

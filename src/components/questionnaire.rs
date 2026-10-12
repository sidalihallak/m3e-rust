use super::motion::next_id;
use super::{Button, ButtonVariant, TextField};
use dioxus::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum QuestionKind {
    #[default]
    Single,
    Multiple,
    Text,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuestionChoice {
    pub value: String,
    pub label: String,
    pub description: String,
    pub disabled: bool,
}
impl QuestionChoice {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: String::new(),
            disabled: false,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuestionDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub kind: QuestionKind,
    pub required: bool,
    pub disabled: bool,
    pub choices: Vec<QuestionChoice>,
}
impl QuestionDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<String>, kind: QuestionKind) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            kind,
            required: false,
            disabled: false,
            choices: vec![],
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuestionAnswer {
    pub question_id: String,
    pub values: Vec<String>,
    pub skipped: bool,
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum QuestionShortcuts {
    #[default]
    None,
    Letters,
    Numbers,
}
#[derive(Clone)]
struct Answers {
    values: Vec<Vec<String>>,
    skipped: Vec<bool>,
    invalid: Vec<bool>,
    step: usize,
}
#[derive(Clone, Copy)]
enum Navigation {
    Previous,
    Next,
    Skip,
    Submit,
}
fn navigate(
    questions: &[QuestionDefinition],
    mut state: Signal<Answers>,
    action: Navigation,
    onsubmit: EventHandler<Vec<QuestionAnswer>>,
    disabled: bool,
) {
    if disabled {
        return;
    }
    let enabled: Vec<usize> = questions
        .iter()
        .enumerate()
        .filter(|(_, q)| !q.disabled)
        .map(|(i, _)| i)
        .collect();
    let Some(&index) = enabled.get(state.read().step) else {
        return;
    };
    if matches!(action, Navigation::Previous) {
        let previous = state.read().step.saturating_sub(1);
        state.write().step = previous;
        return;
    }
    if matches!(action, Navigation::Skip) {
        if questions[index].required {
            return;
        }
        let mut data = state.write();
        data.values[index].clear();
        data.skipped[index] = true;
        data.invalid[index] = false;
    } else if matches!(action, Navigation::Next) {
        let empty = state.read().values[index].is_empty();
        if empty && !state.read().skipped[index] {
            state.write().invalid[index] = true;
            return;
        }
    }
    let last = state.read().step + 1 >= enabled.len();
    if !matches!(action, Navigation::Submit) && !last {
        state.write().step += 1;
        return;
    }
    let missing = enabled.iter().position(|&i| {
        state.read().values[i].is_empty() && (questions[i].required || !state.read().skipped[i])
    });
    if let Some(step) = missing {
        let mut data = state.write();
        data.step = step;
        data.invalid[enabled[step]] = true;
        return;
    }
    let data = state.read();
    let answers = enabled
        .iter()
        .map(|&i| QuestionAnswer {
            question_id: questions[i].id.clone(),
            values: data.values[i].clone(),
            skipped: data.skipped[i],
        })
        .collect();
    drop(data);
    onsubmit.call(answers);
}
/// Local multi-step form with native radio/checkbox editing, retained answers,
/// validation/focus, optional skip and a typed submit result. IDs must be unique
/// and definitions stable while mounted; remount to replace the questionnaire.
#[component]
pub fn Questionnaire(
    questions: Vec<QuestionDefinition>,
    #[props(default="Questionnaire".to_string())] label: String,
    #[props(default)] shortcuts: QuestionShortcuts,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onsubmit: EventHandler<Vec<QuestionAnswer>>,
) -> Element {
    let mut state = use_signal(|| Answers {
        values: vec![vec![]; questions.len()],
        skipped: vec![false; questions.len()],
        invalid: vec![false; questions.len()],
        step: 0,
    });
    let id = use_hook(|| next_id("m3-questionnaire"));
    use_effect({
        let id = id.clone();
        move || {
            let script =
                include_str!("questionnaire.js").replace("__ROOT_ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Questionnaires?.[{id:?}]?.();")).await;
            });
        }
    });
    let enabled: Vec<usize> = questions
        .iter()
        .enumerate()
        .filter(|(_, q)| !q.disabled)
        .map(|(i, _)| i)
        .collect();
    let current = enabled.get(state.read().step).copied();
    let step = state.read().step;
    let count = enabled.len();
    let invalid = current
        .and_then(|i| state.read().invalid.get(i).copied())
        .unwrap_or(false);
    rsx! {form{id:id.clone(),class:"m3-questionnaire {class}",aria_label:label,novalidate:true,"data-step":step,"data-invalid":invalid,"data-shortcuts":match shortcuts{QuestionShortcuts::None=>"none",QuestionShortcuts::Letters=>"letters",QuestionShortcuts::Numbers=>"numbers"},
     onsubmit:{let questions=questions.clone();move |event|{event.prevent_default();navigate(&questions,state,Navigation::Submit,onsubmit,disabled);}},
     div{class:"m3-questionnaire-progress",role:"progressbar",aria_label:"Questionnaire progress",aria_valuemin:if count>0{Some(1)}else{None},aria_valuemax:count,aria_valuenow:if count>0{Some(step+1)}else{None},aria_valuetext:format!("Question {} of {}",if count>0{step+1}else{0},count),aria_live:"polite",{format!("Question {} of {}",if count>0{step+1}else{0},count)}}
     if count==0 {p{role:"status","No available questions."}}
     for (index,question) in questions.iter().enumerate(){
      fieldset{key:"{question.id}",class:"m3-questionnaire-item",hidden:current!=Some(index),inert:if current!=Some(index){Some("")}else{None},disabled:disabled||question.disabled,tabindex:-1,aria_invalid:state.read().invalid.get(index).copied().unwrap_or(false),aria_describedby:format!("{id}-{index}-description {id}-{index}-error"),
       legend{class:"m3-questionnaire-title","{question.title}",if question.required{span{class:"m3-questionnaire-required"," (required)"}}}
       p{id:"{id}-{index}-description",class:"m3-questionnaire-description","{question.description}"}
       if question.kind==QuestionKind::Text {
        TextField{label:"Your answer",value:state.read().values.get(index).and_then(|v|v.first()).cloned().unwrap_or_default(),disabled:disabled||question.disabled,error:state.read().invalid.get(index).copied().unwrap_or(false),oninput:move |value:String|{let mut data=state.write();if let Some(answer)=data.values.get_mut(index){*answer=if value.trim().is_empty(){vec![]}else{vec![value]};data.skipped[index]=false;data.invalid[index]=false;}}}
       }else {
        div{class:"m3-questionnaire-choices",for (choice_index,choice) in question.choices.iter().enumerate(){
         label{class:"m3-questionnaire-choice","data-checked":state.read().values.get(index).map(|v|v.contains(&choice.value)).unwrap_or(false),"data-kind":if question.kind==QuestionKind::Multiple{"checkbox"}else{"radio"},"data-disabled":disabled||question.disabled||choice.disabled,
          input{r#type:if question.kind==QuestionKind::Multiple{"checkbox"}else{"radio"},name:format!("{id}-{}",question.id),value:choice.value.clone(),checked:state.read().values.get(index).map(|v|v.contains(&choice.value)).unwrap_or(false),disabled:disabled||question.disabled||choice.disabled,aria_invalid:state.read().invalid.get(index).copied().unwrap_or(false),aria_describedby:format!("{id}-{index}-error"),
           onchange:{let value=choice.value.clone();let multiple=question.kind==QuestionKind::Multiple;move |event:FormEvent|{let mut data=state.write();if let Some(answer)=data.values.get_mut(index){if multiple{if event.checked(){if !answer.contains(&value){answer.push(value.clone());}}else{answer.retain(|v|v!=&value);}}else{*answer=vec![value.clone()];}data.invalid[index]=false;data.skipped[index]=false;}}}
          }
          span{class:"m3-questionnaire-choice-indicator",aria_hidden:"true",span{super::Icon{icon:crate::icons::CHECK}}}
          span{class:"m3-questionnaire-choice-label",span{"{choice.label}"},if !choice.description.is_empty(){span{class:"m3-questionnaire-description","{choice.description}"}}}
          if shortcuts!=QuestionShortcuts::None&&choice_index<if shortcuts==QuestionShortcuts::Letters{26}else{9}{span{class:"m3-questionnaire-shortcut",aria_hidden:"true",{if shortcuts==QuestionShortcuts::Letters{((b'A'+choice_index as u8)as char).to_string()}else{(choice_index+1).to_string()}}}}
         }
        }}
       }
       p{id:"{id}-{index}-error",class:"m3-questionnaire-error",role:if state.read().invalid.get(index).copied().unwrap_or(false){Some("alert")}else{None},hidden:!state.read().invalid.get(index).copied().unwrap_or(false),if question.required{"Choose an answer to continue."}else{"Choose an answer or skip this question."}}
      }
     }
     div{class:"m3-questionnaire-actions",
      if step>0 {Button{variant:ButtonVariant::Outlined,disabled,onclick:{let questions=questions.clone();move |_|navigate(&questions,state,Navigation::Previous,onsubmit,disabled)},"Previous"}}
      if current.is_some_and(|i|!questions[i].required){Button{variant:ButtonVariant::Outlined,class:"m3-questionnaire-skip",disabled,onclick:{let questions=questions.clone();move |_|navigate(&questions,state,Navigation::Skip,onsubmit,disabled)},"Skip"}}
      if count>0 {Button{class:"m3-questionnaire-next",disabled,onclick:{let questions=questions.clone();move |_|navigate(&questions,state,Navigation::Next,onsubmit,disabled)},if step+1>=count{"Submit"}else{"Next"}}}
     }
    }}
}

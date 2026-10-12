use super::motion::next_id;
use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum OTPPattern {
    #[default]
    Digits,
    Alphanumeric,
}
impl OTPPattern {
    fn clean(self, text: &str, length: usize) -> String {
        text.chars()
            .filter(|c| match self {
                Self::Digits => c.is_ascii_digit(),
                Self::Alphanumeric => c.is_ascii_alphanumeric(),
            })
            .take(length)
            .collect()
    }
}
/// One named native text input; slots are decorative. No focus hopping or numeric spinners.
/// `oncomplete` reports a newly completed value; the application owns verification.
#[component]
pub fn InputOTP(
    label: String,
    #[props(default)] value: String,
    #[props(default = 6)] length: usize,
    #[props(default=Some(3))] separator_after: Option<usize>,
    #[props(default)] pattern: OTPPattern,
    #[props(default)] supporting: Option<String>,
    #[props(default)] error: bool,
    #[props(default)] disabled: bool,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    #[props(default)] name: String,
    #[props(default)] class: String,
    #[props(default)] oninput: EventHandler<String>,
    #[props(default)] oncomplete: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-otp"));
    let length = length.clamp(1, 12);
    let display = pattern.clean(&value, length);
    let initial = display.clone();
    let mut completed = use_signal(|| initial);
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("input_otp.js").replace("__ROOT_ID__", &id);
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    rsx! {div{id:id.clone(),class:"m3-otp-field {class}","data-error":error,
        label{class:"m3-input-group__label",r#for:"{id}-input","{label}"}
        div{class:"m3-otp","data-pattern":if pattern==OTPPattern::Digits{"digits"}else{"alphanumeric"},
            input{id:"{id}-input",class:"m3-otp__input",r#type:"text",value:display.clone(),name,autocomplete:"one-time-code",inputmode:if pattern==OTPPattern::Digits{"numeric"}else{"text"},spellcheck:"false",pattern:if pattern==OTPPattern::Digits{format!("[0-9]{{{length}}}")}else{format!("[a-zA-Z0-9]{{{length}}}")},maxlength:length.to_string(),disabled,readonly:read_only,required,"aria-invalid":error,"aria-describedby":"{id}-support",
                oninput:move |e| {let next=pattern.clean(&e.value(),length);let became_complete=completed().len()<length&&next.len()==length;completed.set(next.clone());oninput.call(next.clone());if became_complete{oncomplete.call(next);}}

            }
            div{class:"m3-otp__slots",aria_hidden:"true",
                for index in 0..length {
                    if separator_after==Some(index)&&index>0 {span{class:"m3-otp__separator","−"}}
                    div{class:"m3-otp__slot","data-index":index.to_string(),span{class:"m3-otp__char","{display.chars().nth(index).unwrap_or(' ')}"}span{class:"m3-otp__caret"}}
                }
            }
        }
        p{id:"{id}-support",class:"m3-input-group__support",role:if error{"alert"}else{"none"},if let Some(text)=supporting{"{text}"}}
    }}
}

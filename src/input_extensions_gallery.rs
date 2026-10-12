use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const INPUT_GROUP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{InputGroup, InputGroupInput, InputGroupAddon, InputGroupAlign, InputGroupText};
#[component]
fn PriceInput() -> Element {
    let mut price = use_signal(String::new);
    rsx! { InputGroup { label: "Price in US dollars", supporting: "Enter an amount, for example 24.50",
        InputGroupAddon { InputGroupText { "$" } }
        InputGroupInput { value: price(), input_mode: "decimal", oninput: move |value| price.set(value) }
        InputGroupAddon { align: InputGroupAlign::InlineEnd, InputGroupText { "USD" } }
    } }
}"#;
const TEXTAREA_GROUP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{InputGroup, InputGroupTextarea, InputGroupAddon, InputGroupAlign, InputGroupButton, ButtonVariant};
#[component]
fn MessageComposer() -> Element {
    let mut message = use_signal(String::new);
    rsx! { InputGroup { label: "Message",
        InputGroupTextarea { value: message(), oninput: move |value| message.set(value) }
        InputGroupAddon { align: InputGroupAlign::BlockEnd,
            InputGroupButton { variant: ButtonVariant::Text, disabled: message().is_empty(), onclick: move |_| message.set(String::new()), "Send message" }
        }
    } }
}"#;
const OTP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::InputOTP;
#[component]
fn VerificationCode() -> Element {
    let mut code = use_signal(String::new);
    let mut complete = use_signal(|| false);
    rsx! { InputOTP { label: "Verification code", value: code(), length: 6,
        supporting: "Enter the six-digit code",
        oninput: move |value: String| { complete.set(value.len() == 6); code.set(value); },
        oncomplete: move |_| complete.set(true),
    }
    p { role: "status", if complete() { "Code ready to verify" } } }
}"#;
const ALPHA_OTP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{InputOTP, OTPPattern};
#[component]
fn InvitationCode() -> Element {
    let mut code = use_signal(String::new);
    rsx! { InputOTP { label: "Invitation code", value: code(), length: 4,
        separator_after: None, pattern: OTPPattern::Alphanumeric,
        supporting: "Use four letters or digits",
        oninput: move |value| code.set(value),
    } }
}"#;
#[component]
pub fn InputExtensionsGallery(active: String) -> Element {
    let mut price = use_signal(String::new);
    let mut query = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut sent = use_signal(|| 0usize);
    let mut code = use_signal(String::new);
    let mut complete = use_signal(|| false);
    let mut alpha = use_signal(String::new);
    rsx! {
        section{id:"input-groups",class:"roles-section wrap",hidden:active!="input-groups",
            div{class:"section-header",div{p{class:"eyebrow","SELECTION / INPUT GROUPS"}h2{"Room for context"}}p{class:"section-description","Persistent labels, native editing and composable prefixes, suffixes and actions."}}
            div{class:"m3-input-extensions-demo",
                InputGroup{label:"Price in US dollars",supporting:"Enter an amount, for example 24.50",InputGroupAddon{InputGroupText{"$"}}InputGroupInput{value:price(),input_mode:"decimal",oninput:move |v|price.set(v)}InputGroupAddon{align:InputGroupAlign::InlineEnd,InputGroupText{"USD"}}}
                InputGroup{label:"Find a project",InputGroupAddon{Icon{icon:icons::SEARCH}}InputGroupInput{value:query(),placeholder:"Project name",oninput:move |v|query.set(v)}InputGroupAddon{align:InputGroupAlign::InlineEnd,InputGroupButton{variant:ButtonVariant::Text,disabled:query().is_empty(),onclick:move |_|query.set(String::new()),"Clear"}}}
                InputGroup{label:"Message",InputGroupAddon{align:InputGroupAlign::BlockStart,InputGroupText{"To your workspace"}}InputGroupTextarea{value:message(),oninput:move |v|message.set(v)}InputGroupAddon{align:InputGroupAlign::BlockEnd,InputGroupText{"{message().chars().count()} characters"}InputGroupButton{variant:ButtonVariant::Text,disabled:message().is_empty(),onclick:move |_|{sent+=1;message.set(String::new());},"Send message"}}}
                p{role:"status","Messages sent: {sent}"}
                InputGroup{label:"Unavailable field",disabled:true,supporting:"This field is disabled",InputGroupInput{value:"Unavailable"}InputGroupAddon{align:InputGroupAlign::InlineEnd,InputGroupButton{"Action"}}}
                InputGroup{label:"Workspace ID (read-only)",InputGroupInput{value:"atlas-2026",read_only:true}}
                InputGroup{label:"Postal code",error:true,supporting:"Error: enter a valid postal code",InputGroupInput{value:"ABC",error:true}InputGroupAddon{align:InputGroupAlign::InlineEnd,Icon{icon:icons::CANCEL}}}
            }
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Input group usage",code:INPUT_GROUP_USAGE}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Textarea group usage",code:TEXTAREA_GROUP_USAGE}
        }
        section{id:"otp-input",class:"roles-section wrap",hidden:active!="otp-input",
            div{class:"section-header",div{p{class:"eyebrow","SELECTION / ONE-TIME CODES"}h2{"One code, one field"}}p{class:"section-description","A single native field keeps keyboard selection, paste and code autofill together."}}
            div{class:"m3-input-extensions-demo",
                InputOTP{label:"Verification code",value:code(),supporting:"Enter the six-digit code",oninput:move |v:String|{complete.set(v.len()==6);code.set(v);},oncomplete:move |_|complete.set(true)}
                p{role:"status",if complete(){"Code ready to verify"}else{"Waiting for six digits"}}
                InputOTP{label:"Invitation code",value:alpha(),length:4,separator_after:None,pattern:OTPPattern::Alphanumeric,supporting:"Use four letters or digits",oninput:move |v|alpha.set(v)}
                InputOTP{label:"Expired code",value:"123456",error:true,supporting:"Error: this code expired. Request a new code."}
                InputOTP{label:"Disabled code",value:"123456",disabled:true}
                InputOTP{label:"Saved code (read-only)",value:"654321",read_only:true}
                Button{variant:ButtonVariant::Text,"Focus after code fields"}
            }
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"OTP input usage",code:OTP_USAGE}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Alphanumeric OTP usage",code:ALPHA_OTP_USAGE}
        }
    }
}

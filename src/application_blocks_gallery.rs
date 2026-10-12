use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const ATTACHMENT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn AttachedDocument()->Element {
    let mut status=use_signal(|| "Ready".to_string());
    rsx! { Attachment {
        AttachmentMedia { Icon {icon:icons::INFO} }
        AttachmentContent {
            AttachmentTitle { "Brief.pdf" }
            AttachmentDescription { "PDF · 2.4 MB" }
        }
        AttachmentTrigger { label:"Open Brief.pdf", onclick:move |_|status.set("Opened".into()) }
        AttachmentActions { AttachmentAction { label:"Remove Brief.pdf", onclick:move |_|status.set("Removed".into()) } }
    } p {role:"status", "{status}"} }
}"#;
const BUBBLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn MessageBubbles()->Element {
    let mut liked=use_signal(||false);
    rsx! { BubbleGroup {
        Bubble {variant:BubbleVariant::Secondary, BubbleContent { "Can you review the draft?" } }
        Bubble {align:MessageAlign::End,
            BubbleContent {"Yes, I will share feedback today."}
            BubbleReactions { button {r#type:"button",aria_label:"Like reply",aria_pressed:liked(),onclick:move |_|liked.toggle(),"👍"} }
        }
    } }
}"#;
const MESSAGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ConversationMessage()->Element {
    rsx! { Message {label:"Alex, 10:24",
        MessageAvatar { "A" }
        MessageContent {
            MessageHeader {"Alex"}
            Bubble {variant:BubbleVariant::Secondary, BubbleContent {"The new design is ready."} }
            MessageFooter {time {datetime:"2026-10-12T10:24:00", "10:24"}}
        }
    } }
}"#;
const SCROLLER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ConversationHistory()->Element {
    let mut count=use_signal(||12);
    rsx! { MessageScroller {label:"Conversation history",height:"320px",auto_scroll:true,
        for index in 0..count() {
            MessageScrollerItem {key:"{index}",message_id:format!("reply-{index}"),
                Message { Bubble {variant:BubbleVariant::Secondary, BubbleContent {"Reply {index}"} } }
            }
        }
    }
    Button {onclick:move |_|count+=1,"Append reply"} }
}"#;
const QUESTIONNAIRE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ProjectQuestionnaire()->Element {
    let mut result=use_signal(||String::new());
    let mut kind=QuestionDefinition::new("project","What are you building?",QuestionKind::Single);
    kind.required=true;
    kind.choices=vec![QuestionChoice::new("web","Web app"),QuestionChoice::new("mobile","Mobile app")];
    let details=QuestionDefinition::new("details","Anything else?",QuestionKind::Text);
    rsx! {Questionnaire {questions:vec![kind,details],shortcuts:QuestionShortcuts::Letters,
        onsubmit:move |answers:Vec<QuestionAnswer>|result.set(format!("{answers:?}"))
    } p {role:"status","{result}"} }
}"#;
fn questions() -> Vec<QuestionDefinition> {
    let mut platform = QuestionDefinition::new(
        "platform",
        "Where will your project run?",
        QuestionKind::Single,
    );
    platform.required = true;
    platform.description = "Choose one platform. Use A or B to select a choice.".into();
    platform.choices = vec![
        QuestionChoice::new("web", "Web application"),
        QuestionChoice::new("mobile", "Mobile application"),
        QuestionChoice::new("unavailable", "Desktop preview unavailable"),
    ];
    platform.choices[0].description = "A browser experience for your users.".into();
    platform.choices[2].disabled = true;
    let mut features = QuestionDefinition::new(
        "features",
        "Which features matter to you?",
        QuestionKind::Multiple,
    );
    features.description = "Choose more than one, or skip this optional question.".into();
    features.choices = vec![
        QuestionChoice::new("offline", "Offline access"),
        QuestionChoice::new("collaboration", "Team collaboration"),
        QuestionChoice::new("themes", "Personal themes"),
    ];
    let mut notes = QuestionDefinition::new("notes", "Describe your project", QuestionKind::Text);
    notes.required = true;
    notes.description = "A short description helps your team understand the plan.".into();
    let mut unavailable =
        QuestionDefinition::new("disabled", "Disabled question", QuestionKind::Text);
    unavailable.disabled = true;
    vec![platform, unavailable, features, notes]
}
#[component]
pub fn ApplicationBlocksGallery(active: String) -> Element {
    let image = asset!("/assets/card-image.svg");
    let mut attachment_status = use_signal(|| "Ready".to_string());
    let mut upload = use_signal(|| AttachmentState::Uploading);
    let mut like = use_signal(|| false);
    let mut bubble_actions = use_signal(|| 0);
    let mut messages = use_signal(|| (0..12).collect::<Vec<i32>>());
    let mut submitted = use_signal(|| String::new());
    let mut submitted_count = use_signal(|| 0);
    let mut questionnaire_disabled = use_signal(|| false);
    rsx! {
    section{id:"attachments",class:"roles-section wrap",hidden:active!="attachments",
     div{class:"section-header",div{p{class:"eyebrow","APPLICATION BLOCKS / ATTACHMENTS"}h2{"Content carried along"}}p{class:"section-description","Document and media attachments with separate open and remove actions. Upload state belongs to your application."}}
     article{class:"m3-application-demo",
      Attachment{class:"m3-attachment-live",AttachmentMedia{Icon{icon:icons::INFO}}AttachmentContent{AttachmentTitle{"Project brief.pdf"}AttachmentDescription{"PDF · 2.4 MB"}}AttachmentTrigger{label:"Open Project brief.pdf",onclick:move |_|attachment_status.set("Opened Project brief.pdf".into())}AttachmentActions{AttachmentAction{label:"Remove Project brief.pdf",onclick:move |_|attachment_status.set("Removed Project brief.pdf".into())}}}
      div{class:"m3-attachment-demos",
       Attachment{state:AttachmentState::Idle,size:AttachmentSize::Small,AttachmentMedia{Icon{icon:icons::ADD}}AttachmentContent{AttachmentTitle{"Choose a file"}AttachmentDescription{"Waiting for selection"}}AttachmentTrigger{label:"Choose example attachment",onclick:move |_|attachment_status.set("Attachment selection requested".into())}}
       Attachment{state:upload(),AttachmentMedia{Icon{icon:icons::INFO}}AttachmentContent{AttachmentTitle{"Assets.zip"}AttachmentDescription{if upload()==AttachmentState::Done{"Complete"}else{"Uploading · 60%"}}}AttachmentActions{AttachmentAction{label:"Disabled remove Assets.zip",disabled:true}}div{class:"m3-attachment-progress",LinearProgress{value:0.6,aria_label:"Example upload progress"}}}
       Attachment{state:AttachmentState::Processing,size:AttachmentSize::ExtraSmall,AttachmentContent{AttachmentTitle{"Indexing document"}AttachmentDescription{"Processing"}}}
       Attachment{state:AttachmentState::Error,AttachmentMedia{Icon{icon:icons::INFO}}AttachmentContent{AttachmentTitle{"Failed upload.txt"}AttachmentDescription{"Upload failed. Retry when ready."}}AttachmentActions{AttachmentAction{label:"Retry Failed upload.txt",icon:icons::ADD,onclick:move |_|attachment_status.set("Retry requested".into())}}}
      }
      AttachmentGroup{label:"Media attachments",for index in 1..=4{Attachment{orientation:AttachmentOrientation::Vertical,AttachmentMedia{image:true,img{src:image,alt:format!("Landscape attachment {index}")}}AttachmentContent{AttachmentTitle{"Landscape {index}"}AttachmentDescription{"PNG · 640 KB"}}AttachmentTrigger{label:format!("Open landscape {index}"),onclick:move |_|attachment_status.set(format!("Opened landscape {index}"))}}}}
      div{class:"m3-application-actions",Button{variant:ButtonVariant::Text,onclick:move |_|upload.set(AttachmentState::Done),"Complete example upload"}}
      p{role:"status","{attachment_status}"}
     }
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Attachment usage",code:ATTACHMENT_USAGE}
    }
    section{id:"messages",class:"roles-section wrap",hidden:active!="messages",
     div{class:"section-header",div{p{class:"eyebrow","APPLICATION BLOCKS / BUBBLES & MESSAGES"}h2{"A conversation in context"}}p{class:"section-description","Bubbles for content and native actions, paired with sender information, timestamps and reactions."}}
     article{class:"m3-application-demo",
      MessageGroup{
       Message{label:"Alex, 10:24",MessageAvatar{span{class:"m3-demo-message-avatar",aria_hidden:"true","A"}}MessageContent{MessageHeader{"Alex"}Bubble{variant:BubbleVariant::Secondary,BubbleContent{"Can you review the latest design?"}}MessageFooter{time{datetime:"2026-10-12T10:24:00","10:24"}}}}
       Message{align:MessageAlign::End,label:"You, 10:25",MessageContent{MessageHeader{"You"}Bubble{BubbleContent{"Yes — I'll send feedback this afternoon."}BubbleReactions{button{r#type:"button",aria_label:"Like reply",aria_pressed:like(),onclick:move |_|like.toggle(),"👍",span{{if like(){"1"}else{"0"}}}}}MessageFooter{time{datetime:"2026-10-12T10:25:00","10:25 · Delivered"}}}}}
      }
      BubbleGroup{for variant in [BubbleVariant::Muted,BubbleVariant::Tinted,BubbleVariant::Outline,BubbleVariant::Ghost,BubbleVariant::Error]{Bubble{variant,BubbleContent{"{variant:?} content with its paired Material color roles."}}}}
      Bubble{variant:BubbleVariant::Tinted,BubbleContent{action:true,onclick:move |_|bubble_actions+=1,"Open message details"}}
      Bubble{variant:BubbleVariant::Outline,BubbleContent{action:true,disabled:true,"Unavailable message action"}}
      Bubble{variant:BubbleVariant::Outline,BubbleContent{href:"#attachments","View attachments"}}
      Bubble{variant:BubbleVariant::Muted,BubbleContent{"Long content remains readable: verylongunbrokentextthatmustwrapwithinthemessagebubbleatphonewidthwithoutoverflowingthepage."}}
      p{role:"status","Message action activated {bubble_actions} times"}
     }
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Bubble usage",code:BUBBLE_USAGE}
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Message usage",code:MESSAGE_USAGE}
    }
    section{id:"message-scroller",class:"roles-section wrap",hidden:active!="message-scroller",
     div{class:"section-header",div{p{class:"eyebrow","APPLICATION BLOCKS / MESSAGE SCROLLER"}h2{"Keep your place"}}p{class:"section-description","Follow new replies at the end, pause while reading history, and keep the same message in view when older replies load."}}
     article{class:"m3-application-demo",
      MessageScroller{label:"Demo conversation history",auto_scroll:true,height:"360px",class:"m3-scroller-live",for index in messages(){MessageScrollerItem{key:"{index}",message_id:format!("demo-{index}"),scroll_anchor:index==8,Message{align:if index%2==0{MessageAlign::Start}else{MessageAlign::End},label:format!("Reply {index}"),MessageContent{MessageHeader{if index%2==0{"Alex"}else{"You"}}Bubble{variant:if index%2==0{BubbleVariant::Secondary}else{BubbleVariant::Primary},BubbleContent{"Reply {index}: A useful piece of conversation history to read at your own pace."}}MessageFooter{{format!("10:{:02}",index.abs())}}}}}}}
      div{class:"m3-application-actions",Button{onclick:move |_|{let next=messages.read().last().copied().unwrap_or(0)+1;messages.write().push(next);},"Append reply"}Button{variant:ButtonVariant::Outlined,onclick:move |_|{let first=messages.read().first().copied().unwrap_or(0);let mut older=(first-3..first).collect::<Vec<_>>();older.extend(messages());messages.set(older);},"Load older replies"}}
      p{role:"status","Conversation contains {messages.read().len()} replies"}
     }
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Message scroller usage",code:SCROLLER_USAGE}
    }
    section{id:"questionnaire",class:"roles-section wrap",hidden:active!="questionnaire",
     div{class:"section-header",div{p{class:"eyebrow","APPLICATION BLOCKS / QUESTIONNAIRE"}h2{"One question at a time"}}p{class:"section-description","A local multi-step form with single and multiple choices, text answers, optional skipping, validation and retained answers. Nothing is sent to a server."}}
     article{class:"m3-application-demo",Questionnaire{questions:questions(),disabled:questionnaire_disabled(),shortcuts:QuestionShortcuts::Letters,onsubmit:move |answers:Vec<QuestionAnswer>|{submitted_count+=1;submitted.set(format!("{answers:?}"));}}Button{variant:ButtonVariant::Text,toggle:true,selected:questionnaire_disabled(),onclick:move |_|questionnaire_disabled.toggle(),"Disable questionnaire"}p{role:"status","Submitted {submitted_count} times"}if !submitted().is_empty(){pre{class:"m3-questionnaire-result","{submitted}"}}}
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Questionnaire usage",code:QUESTIONNAIRE_USAGE}
    }
    }
}

use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
use std::{
    cell::RefCell,
    rc::Rc,
    task::{Poll, Waker},
};
const HOST_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn NotificationApp()->Element {
 rsx!{ToastProvider {NotificationActions {} Toaster {close_button:true}}}
}
#[component]
fn NotificationActions()->Element {
 let manager=use_toast_manager();
 let mut feedback=use_signal(String::new);
 rsx!{Button {onclick:move|_|{feedback.set("Changes saved".into());manager.message("Changes saved");},"Save changes"}
 p {"{feedback}"}}

}"#;
const QUEUE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn QueuedActions()->Element {
 let manager=use_toast_manager();
 let mut current=use_signal(||None::<ToastId>);
 let mut status=use_signal(||"Ready".to_string());
 rsx!{Button {onclick:move|_|{
  let id=manager.show("Preparing export",ToastOptions {kind:ToastKind::Loading,..Default::default()});
  status.set("Preparing export".into());current.set(Some(id));manager.message("Another update waits in the queue");
 },"Queue notifications"}
 Button {onclick:move|_|{if let Some(id)=current(){status.set("Export complete".into());manager.show("Export complete",ToastOptions {id:Some(id),kind:ToastKind::Success,..Default::default()});}},"Update export"}
 Button {onclick:move|_|manager.dismiss_all(),"Dismiss all"}p {"{status}"}}
}"#;
const FUTURE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn AsyncNotification()->Element {
 let manager=use_toast_manager();
 let mut feedback=use_signal(String::new);
 rsx!{Button {onclick:move|_|{spawn(async move {
  feedback.set("Saving…".into());
  let task=async {Ok::<_,String>("saved")}; // Replace with your application future.
  let result=manager.track(task,ToastPromiseMessages::new("Saving…","Saved","Save failed")).await;
  feedback.set(if result.is_ok(){"Saved"}else{"Save failed"}.into());
 });},"Save asynchronously"}p {"{feedback}"}}
}"#;
const ACTION_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn UndoNotification()->Element {
 let manager=use_toast_manager();
 let mut undone=use_signal(||false);
 rsx!{Button {onclick:move|_|{
  manager.show("Document archived",ToastOptions {
   action:Some(ToastAction {label:"Undo".into(),onclick:EventHandler::new(move|_|undone.set(true))}),
   ..Default::default()
  });
 },"Archive document"}p {"Undone: {undone}"}}
}"#;
#[derive(Default)]
struct TaskState {
    result: Option<Result<(), String>>,
    waker: Option<Waker>,
}
#[component]
fn NotificationControls() -> Element {
    let manager = use_toast_manager();
    let mut feedback = use_signal(String::new);
    use_effect(move || {
        if let Some((_, message)) = manager.snapshot().current {
            feedback.set(message);
        }
    });
    let mut saved_id = use_signal(|| None::<ToastId>);
    let mut actions = use_signal(|| 0);
    let mut reason = use_signal(String::new);
    let mut pending = use_signal(|| false);
    let mut outcomes = use_signal(|| 0);
    let mut position = use_signal(|| ToastPosition::BottomCenter);
    let mut offset = use_signal(|| 0u32);
    let task = use_hook(|| Rc::new(RefCell::new(TaskState::default())));
    let snapshot = manager.snapshot();
    let dismiss = EventHandler::new(move |event: ToastDismissal| {
        reason.set(format!("{:?} ({})", event.reason, event.id.0))
    });
    rsx! {article{class:"notification-demo",
        div{class:"notification-demo-actions",
         Button{onclick:move|_|{let id=manager.show("Changes saved",ToastOptions{ondismiss:Some(dismiss),..Default::default()});saved_id.set(Some(id));},"Show notification"}
         Button{variant:ButtonVariant::Tonal,onclick:move|_|{for title in ["First queued message","Second queued message","Third queued message"]{manager.show(title,ToastOptions{duration_ms:Some(0),ondismiss:Some(dismiss),..Default::default()});}},"Queue three messages"}
         Button{variant:ButtonVariant::Outlined,onclick:move|_|{if let Some(id)=saved_id(){manager.show("Updated message",ToastOptions{id:Some(id),description:Some("Updated in place.".into()),duration_ms:Some(0),ondismiss:Some(dismiss),..Default::default()});}},"Update saved notification"}
         Button{variant:ButtonVariant::Outlined,onclick:move|_|{manager.show("Document archived",ToastOptions{action:Some(ToastAction{label:"Undo".into(),onclick:EventHandler::new(move|_|actions+=1)}),ondismiss:Some(dismiss),..Default::default()});},"Show action notification"}
         Button{variant:ButtonVariant::Outlined,onclick:move|_|{saved_id.set(Some(manager.show("Queued update target",ToastOptions{duration_ms:Some(0),ondismiss:Some(dismiss),..Default::default()})));},"Queue update target"}
      Button{variant:ButtonVariant::Text,onclick:move|_|{if let Some(id)=saved_id(){manager.dismiss(id);}},"Dismiss saved notification"}
      Button{variant:ButtonVariant::Outlined,onclick:move|_|{manager.show("Your changes are available",ToastOptions{action_new_line:true,action:Some(ToastAction{label:"Review saved changes".into(),onclick:EventHandler::new(move|_|actions+=1)}),..Default::default()});},"Show long action"}
      Button{variant:ButtonVariant::Text,onclick:move|_|manager.dismiss_all(),"Dismiss all notifications"}
        }
        div{class:"notification-demo-actions",
         Button{disabled:pending(),onclick:{let task=task.clone();move|_|{pending.set(true);*task.borrow_mut()=TaskState::default();let task=task.clone();spawn(async move{let future=std::future::poll_fn(move|context|{let mut state=task.borrow_mut();if let Some(result)=state.result.take(){Poll::Ready(result)}else{state.waker=Some(context.waker().clone());Poll::Pending}});let result=manager.track(future,ToastPromiseMessages::new("Saving project…","Project saved","Could not save project")).await;feedback.set(if result.is_ok(){"Project saved"}else{"Could not save project"}.into());outcomes+=1;pending.set(false);});}},"Start loading task"}
         Button{disabled:!pending(),variant:ButtonVariant::Tonal,onclick:{let task=task.clone();move|_|{let waker={let mut s=task.borrow_mut();s.result=Some(Ok(()));s.waker.take()};if let Some(w)=waker{w.wake();}}},"Complete task"}
         Button{disabled:!pending(),variant:ButtonVariant::Outlined,onclick:{let task=task.clone();move|_|{let waker={let mut s=task.borrow_mut();s.result=Some(Err("Example failure".into()));s.waker.take()};if let Some(w)=waker{w.wake();}}},"Fail task"}
        }
        p{class:"notification-inline-feedback","Last update: {feedback}"}
    p{role:"status","Completed tasks: {outcomes} · Actions: {actions}"}
        p{"Current: {snapshot.current:?}"}p{"Queued: {snapshot.queued:?}"}p{"Last dismissal: {reason}"}
        div{class:"notification-demo-actions",
         Button{variant:ButtonVariant::Text,onclick:move|_|position.set(ToastPosition::BottomStart),"Place at start"}
         Button{variant:ButtonVariant::Text,onclick:move|_|position.set(ToastPosition::BottomCenter),"Place at center"}
         Button{variant:ButtonVariant::Text,onclick:move|_|position.set(ToastPosition::BottomEnd),"Place at end"}
         Button{variant:ButtonVariant::Text,onclick:move|_|offset.set(if offset()==0{80}else{0}),"Toggle navigation offset"}
        }
       }
       Toaster{position:position(),offset:offset(),close_button:true}
       }
}
#[component]
pub fn NotificationGallery(active: String) -> Element {
    rsx! {section{id:"notification-host",class:"roles-section wrap",hidden:active!="notification-host",
       div{class:"section-header",div{p{class:"eyebrow","FEEDBACK / NOTIFICATION HOST"}h2{"One message at a time"}}p{class:"section-description","Messages queue until the current snackbar has left. Updates reuse an ID; loading tasks resolve in place. Use Alt+G to focus notifications and Escape to dismiss."}}
       if active=="notification-host"{ToastProvider{NotificationControls{}}}
       CodeCard{eyebrow:"Copy into your Dioxus app",title:"Toaster provider usage",code:HOST_USAGE}
       CodeCard{eyebrow:"Inside a ToastProvider",title:"Notification queue usage",code:QUEUE_USAGE}
       CodeCard{eyebrow:"Inside a ToastProvider",title:"Future notification usage",code:FUTURE_USAGE}
    CodeCard{eyebrow:"Persistent until acted on",title:"Action notification usage",code:ACTION_USAGE}
      }}
}

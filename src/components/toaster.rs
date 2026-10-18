use super::{Icon, motion::next_id};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastKind {
    #[default]
    Message,
    Success,
    Info,
    Warning,
    Error,
    Loading,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastDismissReason {
    Action,
    Close,
    Timeout,
    Api,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastDismissal {
    pub id: ToastId,
    pub reason: ToastDismissReason,
}
#[derive(Clone, PartialEq)]
pub struct ToastAction {
    pub label: String,
    pub onclick: EventHandler<()>,
}
#[derive(Clone, Default, PartialEq)]
pub struct ToastOptions {
    pub id: Option<ToastId>,
    pub kind: ToastKind,
    pub description: Option<String>,
    /// None uses the host default; zero is persistent. Actions/loading persist.
    pub duration_ms: Option<u32>,
    pub action: Option<ToastAction>,
    pub close_button: Option<bool>,
    pub action_new_line: bool,
    pub ondismiss: Option<EventHandler<ToastDismissal>>,
}
#[derive(Clone, PartialEq)]
struct Notification {
    id: ToastId,
    uid: u64,
    revision: u64,
    message: String,
    options: ToastOptions,
    closing: bool,
}
#[derive(Clone, Default)]
struct ToastStore {
    current: Option<Notification>,
    queue: Vec<Notification>,
    counter: u64,
}
#[derive(Clone, Copy)]
pub struct ToastManager {
    store: Signal<ToastStore>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastQueueSnapshot {
    pub current: Option<(ToastId, String)>,
    pub queued: Vec<(ToastId, String)>,
    pub closing: bool,
}
#[derive(Clone, PartialEq)]
pub struct ToastPromiseMessages {
    pub loading: String,
    pub success: String,
    pub error: String,
}
impl ToastPromiseMessages {
    pub fn new(
        loading: impl Into<String>,
        success: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            loading: loading.into(),
            success: success.into(),
            error: error.into(),
        }
    }
}
impl ToastManager {
    pub fn show(mut self, message: impl Into<String>, options: ToastOptions) -> ToastId {
        let mut store = self.store.write();
        store.counter += 1;
        let id = if let Some(id) = options.id {
            id
        } else {
            while store
                .current
                .as_ref()
                .is_some_and(|n| n.id == ToastId(store.counter))
                || store.queue.iter().any(|n| n.id == ToastId(store.counter))
            {
                store.counter += 1;
            }
            ToastId(store.counter)
        };
        let uid = store.counter;
        let message = message.into();
        if let Some(current) = store.current.as_mut().filter(|n| n.id == id && !n.closing) {
            current.message = message;
            current.options = options;
            current.revision += 1;
            return id;
        }
        if let Some(queued) = store.queue.iter_mut().find(|n| n.id == id) {
            queued.message = message;
            queued.options = options;
            queued.revision += 1;
            return id;
        }
        let entry = Notification {
            id,
            uid,
            revision: 0,
            message,
            options,
            closing: false,
        };
        if store.current.is_none() {
            store.current = Some(entry)
        } else {
            store.queue.push(entry)
        }
        id
    }
    pub fn message(self, message: impl Into<String>) -> ToastId {
        self.show(message, ToastOptions::default())
    }
    pub fn success(self, message: impl Into<String>) -> ToastId {
        self.show(
            message,
            ToastOptions {
                kind: ToastKind::Success,
                ..Default::default()
            },
        )
    }
    pub fn error(self, message: impl Into<String>) -> ToastId {
        self.show(
            message,
            ToastOptions {
                kind: ToastKind::Error,
                ..Default::default()
            },
        )
    }
    pub fn info(self, message: impl Into<String>) -> ToastId {
        self.show(
            message,
            ToastOptions {
                kind: ToastKind::Info,
                ..Default::default()
            },
        )
    }
    pub fn warning(self, message: impl Into<String>) -> ToastId {
        self.show(
            message,
            ToastOptions {
                kind: ToastKind::Warning,
                ..Default::default()
            },
        )
    }
    pub fn loading(self, message: impl Into<String>) -> ToastId {
        self.show(
            message,
            ToastOptions {
                kind: ToastKind::Loading,
                ..Default::default()
            },
        )
    }
    pub fn snapshot(self) -> ToastQueueSnapshot {
        let store = self.store.read();
        ToastQueueSnapshot {
            current: store.current.as_ref().map(|n| (n.id, n.message.clone())),
            queued: store
                .queue
                .iter()
                .map(|n| (n.id, n.message.clone()))
                .collect(),
            closing: store.current.as_ref().is_some_and(|n| n.closing),
        }
    }
    pub fn dismiss(self, id: ToastId) {
        self.close(id, ToastDismissReason::Api)
    }
    pub fn dismiss_all(mut self) {
        let callbacks = {
            let mut store = self.store.write();
            let mut entries = std::mem::take(&mut store.queue);
            if let Some(current) = store.current.as_mut().filter(|n| !n.closing) {
                current.closing = true;
                entries.push(current.clone());
            }
            entries
        };
        for n in callbacks {
            if let Some(handler) = n.options.ondismiss {
                handler.call(ToastDismissal {
                    id: n.id,
                    reason: ToastDismissReason::Api,
                });
            }
        }
    }
    fn close(mut self, id: ToastId, reason: ToastDismissReason) {
        let callback = {
            let mut store = self.store.write();
            if let Some(n) = store.current.as_mut().filter(|n| n.id == id) {
                if n.closing {
                    return;
                }
                n.closing = true;
                n.options.ondismiss
            } else if let Some(index) = store.queue.iter().position(|n| n.id == id) {
                store.queue.remove(index).options.ondismiss
            } else {
                return;
            }
        };
        if let Some(callback) = callback {
            callback.call(ToastDismissal { id, reason });
        }
    }
    fn finish(mut self, uid: u64) {
        let mut store = self.store.write();
        if !store
            .current
            .as_ref()
            .is_some_and(|n| n.uid == uid && n.closing)
        {
            return;
        }
        store.current = if store.queue.is_empty() {
            None
        } else {
            Some(store.queue.remove(0))
        };
    }
    fn timeout(self, uid: u64, revision: u64) {
        let id = self
            .store
            .read()
            .current
            .as_ref()
            .filter(|n| n.uid == uid && n.revision == revision && !n.closing)
            .map(|n| n.id);
        if let Some(id) = id {
            self.close(id, ToastDismissReason::Timeout);
        }
    }
    /// Runs the caller's future and updates its loading message in place. Closing
    /// or superseding that notification prevents a late result resurrecting it.
    pub async fn track<F, T, E>(mut self, future: F, messages: ToastPromiseMessages) -> Result<T, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        let id = self.loading(messages.loading);
        let (uid, revision) = {
            let s = self.store.read();
            let n = s
                .current
                .as_ref()
                .filter(|n| n.id == id)
                .or_else(|| s.queue.iter().find(|n| n.id == id))
                .unwrap();
            (n.uid, n.revision)
        };
        let result = future.await;
        let mut store = self.store.write();
        let n = if store.current.as_ref().is_some_and(|n| n.id == id) {
            store.current.as_mut()
        } else {
            store.queue.iter_mut().find(|n| n.id == id)
        };
        if let Some(n) = n.filter(|n| n.uid == uid && n.revision == revision && !n.closing) {
            n.message = if result.is_ok() {
                messages.success
            } else {
                messages.error
            };
            n.options.kind = if result.is_ok() {
                ToastKind::Success
            } else {
                ToastKind::Error
            };
            n.revision += 1;
        }
        result
    }
}
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let store = use_signal(ToastStore::default);
    use_context_provider(|| ToastManager { store });
    rsx! {{children}}
}
pub fn use_toast_manager() -> ToastManager {
    use_context::<ToastManager>()
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastPosition {
    #[default]
    BottomCenter,
    BottomStart,
    BottomEnd,
}
/// Render exactly once per provider near the application root, outside transformed
/// ancestors. One Material snackbar is visible; the others wait for its exit.
#[component]
pub fn Toaster(
    #[props(default)] position: ToastPosition,
    #[props(default)] offset: u32,
    #[props(default = 4000)] duration_ms: u32,
    #[props(default)] close_button: bool,
    #[props(default="Notifications".into())] label: String,
    #[props(default)] class: String,
) -> Element {
    let manager = use_toast_manager();
    let id = use_hook(|| next_id("m3-toaster"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("toaster.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok((kind, uid, revision)) = eval.recv::<(String, u64, u64)>().await {
                    if kind == "closed" {
                        manager.finish(uid)
                    } else if kind == "timeout" {
                        manager.timeout(uid, revision)
                    } else if kind == "close" {
                        let current = manager
                            .store
                            .read()
                            .current
                            .as_ref()
                            .filter(|n| n.uid == uid)
                            .map(|n| n.id);
                        if let Some(id) = current {
                            manager.close(id, ToastDismissReason::Close);
                        }
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Toasters?.[{id:?}]?.();"));
        }
    });
    let current = manager.store.read().current.clone();
    let position = match position {
        ToastPosition::BottomCenter => "center",
        ToastPosition::BottomStart => "start",
        ToastPosition::BottomEnd => "end",
    };
    rsx! {div{id,class:"m3-toaster {class}",role:"region",aria_label:label,aria_keyshortcuts:"Alt+G",tabindex:"-1","data-position":position,style:"--m3-toast-offset:{offset}px",
     span{class:"m3-feedback__sr",role:"status",aria_live:"polite",aria_atomic:"true","data-toast-announcement":""}
     if let Some(n)=current {
      {let options=n.options.clone();let duration=if options.action.is_some()||options.kind==ToastKind::Loading{0}else{let raw=options.duration_ms.unwrap_or(duration_ms);if raw==0{0}else{raw.clamp(4000,10000)}};let toast_id=n.id;let uid=n.uid;let revision=n.revision;let kind=match options.kind{ToastKind::Message=>"message",ToastKind::Success=>"success",ToastKind::Info=>"info",ToastKind::Warning=>"warning",ToastKind::Error=>"error",ToastKind::Loading=>"loading"};let announcement=options.description.as_ref().map(|d|format!("{}: {}",n.message,d)).unwrap_or(n.message.clone());
       rsx!{div{key:"{uid}",class:"m3-toast-card","data-uid":uid,"data-revision":n.revision,"data-open":!n.closing,"data-duration":duration,"data-type":kind,"data-message":announcement,"data-new-line":options.action_new_line,inert:if n.closing{Some("")}else{None},aria_hidden:if n.closing{Some("true")}else{None},
        div{class:"m3-toast-text",p{class:"m3-toast-title","{n.message}"}if let Some(description)=options.description{p{class:"m3-toast-description","{description}"}}}
        div{class:"m3-toast-actions",
         if let Some(action)=options.action{button{r#type:"button",class:"m3-snackbar__action",onclick:move|_|{action.onclick.call(());let same=manager.store.read().current.as_ref().is_some_and(|n|n.uid==uid&&n.revision==revision&&!n.closing);if same{manager.close(toast_id,ToastDismissReason::Action);}},super::ripple::Ripple{},"{action.label}"}}
         if options.close_button.unwrap_or(close_button){button{r#type:"button",class:"m3-feedback__icon",aria_label:"Dismiss notification",onclick:move|_|manager.close(toast_id,ToastDismissReason::Close),super::ripple::Ripple{},Icon{icon:icons::CLOSE}}}
        }
       }}
      }
     }
    }}
}

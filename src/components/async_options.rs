use super::SelectOption;
use dioxus::prelude::*;
#[derive(Clone, Default, PartialEq)]
pub struct AsyncOptionsSnapshot {
    pub query: String,
    pub options: Vec<SelectOption>,
    pub loading: bool,
    pub error: Option<String>,
    generation: u64,
}
#[derive(Clone, Copy)]
pub struct AsyncOptions {
    state: Signal<AsyncOptionsSnapshot>,
}
pub fn use_async_options() -> AsyncOptions {
    AsyncOptions {
        state: use_signal(AsyncOptionsSnapshot::default),
    }
}
impl AsyncOptions {
    pub fn snapshot(self) -> AsyncOptionsSnapshot {
        (self.state)()
    }
    /// Start synchronously, before spawning the future, so request order owns the
    /// generation. Results from obsolete requests are ignored, including errors.
    pub fn begin(mut self, query: impl Into<String>) -> AsyncOptionsRequest {
        let mut state = self.state.write();
        state.generation += 1;
        state.query = query.into();
        state.loading = true;
        state.error = None;
        state.options.clear();
        let generation = state.generation;
        drop(state);
        AsyncOptionsRequest {
            state: self.state,
            generation,
        }
    }
    pub fn cancel(mut self) {
        let mut state = self.state.write();
        state.generation += 1;
        state.loading = false;
    }
}
#[derive(Clone, Copy)]
pub struct AsyncOptionsRequest {
    state: Signal<AsyncOptionsSnapshot>,
    generation: u64,
}
impl AsyncOptionsRequest {
    pub fn complete(mut self, result: Result<Vec<SelectOption>, String>) {
        let mut state = self.state.write();
        if state.generation != self.generation {
            return;
        }
        state.loading = false;
        match result {
            Ok(options) => {
                state.options = options;
                state.error = None;
            }
            Err(message) => {
                state.options.clear();
                state.error = Some(message);
            }
        }
    }
    pub async fn resolve<F>(self, future: F)
    where
        F: std::future::Future<Output = Result<Vec<SelectOption>, String>>,
    {
        self.complete(future.await);
    }
}

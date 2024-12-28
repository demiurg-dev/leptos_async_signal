use leptos::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

const _: () = assert!(
    cfg!(not(all(feature = "hydrate", feature = "ssr"))),
    "cannot enable hydrate and ssr features at the same time"
);
const _: () = assert!(
    cfg!(any(feature = "hydrate", feature = "ssr")),
    "need to enable hydrate or ssr feature"
);

#[cfg(feature = "ssr")]
mod async_state_ssr;
#[cfg(feature = "ssr")]
use async_state_ssr::AsyncState;

#[cfg(feature = "hydrate")]
mod async_state_hydrate;
#[cfg(feature = "hydrate")]
use async_state_hydrate::AsyncState;

#[derive(Clone)]
pub struct AsyncWriteSignal<T>
where
    T: 'static,
{
    inner: WriteSignal<T>,
    state: AsyncState,
}

pub fn async_signal<T>(default: T) -> (Resource<T>, AsyncWriteSignal<T>)
where
    T: Clone + Send + Sync + PartialEq + Serialize + DeserializeOwned,
{
    let (signal_read, signal_write) = signal(default);
    let state = AsyncState::default();
    let signal_write = AsyncWriteSignal {
        inner: signal_write,
        state: state.clone(),
    };
    let resource = Resource::new(
        move || signal_read.get(),
        move |_| {
            let state = state.clone();
            async move {
                state.wait().await;
                signal_read.get_untracked()
            }
        },
    );
    (resource, signal_write)
}

impl<T> Set for AsyncWriteSignal<T>
where
    T: Send + Sync + 'static,
{
    type Value = T;

    fn set(&self, value: Self::Value) {
        self.inner.set(value);
        #[cfg(not(feature = "hydrate"))]
        self.state.mark_ready();
    }

    fn try_set(&self, value: Self::Value) -> Option<Self::Value> {
        let res = self.inner.try_set(value);
        #[cfg(not(feature = "hydrate"))]
        self.state.mark_ready();
        res
    }
}

impl<T> Drop for AsyncWriteSignal<T> {
    fn drop(&mut self) {
        self.state.mark_ready();
    }
}

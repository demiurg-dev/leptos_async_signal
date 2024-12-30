//! # About async signal
//! 
//! `leptos_async_signal` is a library built on top of
//! [Leptos](https://github.com/leptos-rs/leptos) that extends the functionality of Leptos signals
//!  to provide a mechanism for generating values  asynchronously. This library is particularly
//! useful in server-side rendering (SSR) contexts where  certain application elements need to be
//! generated asynchronously before the associated signal is set.
//! 
//! # Use case
//! 
//! A typical example is generating breadcrumbs for a page. Breadcrumbs, which appear at the top
//! of the page, often depend on deeper page elements or server-side data. With
//! `leptos_async_signal`, you can generate these breadcrumbs asynchronously in SSR mode and still
//! allow them to react to changes dynamically in other modes.
//! 
//! This pattern mimics the behavior of `leptos_meta` for managing HTML meta elements but extends
//! the functionality to any application element.
//! 
//! # Example
//! 
//! Check the
//! [breadcrumbs example](https://github.com/demiurg-dev/leptos_async_signal/tree/main/sample-crumbs)
//! in the repository.

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

/// An async write signal. This is almost the same as the regular Leptos write signal, but under
///  the hood also takes care of notifying the resource about the new value.
#[derive(Clone)]
pub struct AsyncWriteSignal<T>
where
    T: 'static,
{
    inner: WriteSignal<T>,
    state: AsyncState,
}

/// Creates a new async signal, that is a pair of a resource and an async write signal. The
/// default provided value is used only as a placeholder value in the case that write signal
/// is never written to (detected by the dropped value before write/set).
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

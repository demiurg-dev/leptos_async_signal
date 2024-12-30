[![CI Status](https://github.com/demiurg-dev/leptos_async_signal/actions/workflows/rust.yml/badge.svg)](https://github.com/demiurg-dev/leptos_async_signal/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/leptos_async_signal)](https://crates.io/crates/leptos_async_signal)
[![docs.rs](https://img.shields.io/docsrs/leptos_async_signal)](https://docs.rs/leptos_async_signal/latest/leptos_async_signal/)

# `leptos_async_signal`

`leptos_async_signal` is a library built on top of [Leptos](https://github.com/leptos-rs/leptos) that 
extends the functionality of Leptos signals to provide a mechanism for generating values 
asynchronously. This library is particularly useful in server-side rendering (SSR) contexts where 
certain application elements need to be generated asynchronously before the associated signal is set.

## Features

The core feature of `leptos_async_signal` is the **asynchronous signal**. An async signal is a 
combination of:

1. **A writable signal**: A Leptos-signal-like object that allows updates of its value.
2. **A resource**: A Leptos resource that produces a value asynchronously whenever the signal is 
    updated.

This allows the developer to prepare application elements in advance (e.g., for SSR) while maintaining 
a seamless interface for reactive updates.

## Use Case

A typical example is generating breadcrumbs for a page. Breadcrumbs, which appear at the top of the 
page, often depend on deeper page elements or server-side data. With `leptos_async_signal`, you can 
generate these breadcrumbs asynchronously in SSR mode and still allow them to react to changes 
dynamically in other modes.

This pattern mimics the behavior of `leptos_meta` for managing HTML meta elements but extends the 
functionality to any application element.

## Example

Please take a look at the [sample-crumbs](sample-crumbs/) example for detailed usage.

The main idea is as follows:
- At the top of the application, create a write signal and a resource that hold the breadcrumbs state.
- (Optional) Store the writable signal in context so that it does not need to be passed down.
- Pass the resource to a top-level component that generates the breadcrumbs using something like 
    `Suspend`.
- On a particular page, write to the write signal (obtained from parameters or global context) when 
    the required data for the breadcrumbs is obtained (this can include data from a server function).

## Usage

To use `leptos_async_signal`, add it to your `Cargo.toml`, and use `ssr` feature appropriately:

```toml
[dependencies]
leptos_async_signal = "0.1.0"
leptos = "0.7"

[features]
default = []
hydrate = ["leptos/hydrate"]
# In app's ssr feature, alo use leptos_async_signal ssr feature
ssr = ["leptos/ssr", "leptos_async_signal/ssr"]
```

See an example in `sample-crumbs` [Cargo.toml](sample-crumbs/Cargo.toml)

## Leptos versions

The currently supported Leptos version is `0.7.x`.

## Notes

- In SSR mode, `leptos_async_signal` ensures that resources are fully resolved before rendering, 
    providing a smooth and efficient server-side rendering experience.
- In other modes (e.g., client-side rendering), resources behave like regular Leptos resources, 
    updating reactively when their associated signal changes.

## Contributions

Contributions to `leptos_async_signal` are welcome! If you encounter bugs, have feature requests, or 
want to contribute improvements, please open an issue or a pull request on the [GitHub repository]
(https://github.com/demiurg-dev/leptos_async_signal).

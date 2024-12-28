use futures::StreamExt;
use leptos::prelude::*;
use leptos_async_signal::*;

#[component]
fn App() -> impl IntoView {
    view! {
        {
            let (msg_res, msg_tx) = async_signal("default message".to_string());
            view! {
                <Suspense>
                    { move || {
                        let msg = match msg_res.get() {
                            None => "no msg yet".to_owned(),
                            Some(msg) => format!("msg is: {msg}")
                        };
                        view! { <span id="msg">{msg}</span> }
                    }
                }
                </Suspense>
                <Component msg_tx=msg_tx />
            }
        }
    }
}

#[component]
fn Component(msg_tx: AsyncWriteSignal<String>) -> impl IntoView {
    let data = Resource::new(
        || (),
        move |_| {
            let _msg_tx = msg_tx.clone();
            async move {
                let (_msg, num) = tests_ssr::fetch_data().await;
                // NOTE: We forget to set the value:
                //  msg_tx.set(msg);
                num
            }
        },
    );
    view! {
        <Suspense>
            { move || {
                    match data.get() {
                        Some(num) => view! { <span>The number is: {num}</span> }.into_any(),
                        None => view! { <span>No number</span> }.into_any(),
                    }
                }
            }
        </Suspense>
    }
}

#[tokio::test]
async fn render_async() {
    any_spawner::Executor::init_tokio().unwrap();
    let app = view! { <App /> };
    let html = app.to_html_stream_in_order().collect::<String>().await;
    println!("{html}");
    assert!(html.contains("msg is: default message"));
}

use std::time::Duration;

use futures::StreamExt;
use leptos::prelude::*;
use leptos_async_signal::*;
use tests_ssr::init_test;
use tokio::time::timeout;

#[component]
pub fn App() -> impl IntoView {
    view! {
        {
            let (msg_res, _) = async_signal("default message".to_string());
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
            }
        }
    }
}

#[tokio::test]
async fn render_async() {
    init_test();
    let app = view! { <App /> };
    let html = timeout(Duration::from_secs(1), app.to_html_stream_in_order().collect::<String>())
        .await
        .expect("SSR should not have timed out");
    assert!(html.contains("msg is: default message"));
}

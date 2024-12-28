use leptos::prelude::*;
use leptos_async_signal::{async_signal, AsyncWriteSignal};
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes}, hooks::use_params, params::Params, path, SsrMode
};
use serde::{Deserialize, Serialize};

use crate::model::Post;

pub fn shell(options: LeptosOptions) -> impl IntoView {    
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (crumbs_res, crumbs_tx) = async_signal(Crumbs::default());
    provide_context(crumbs_tx);

    view! {
        <Router>
            <main>
                <Crumbs crumbs=crumbs_res />
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") ssr=SsrMode::Async view=HomePage />
                    <Route path=path!("post/:id") ssr=SsrMode::Async view=PostPage />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
enum Crumbs {
    #[default]
    Home,
    Post { title: String },
}

impl Crumbs {
    fn into_view(self) -> impl IntoView {
        match self {
            Crumbs::Home => view! { <span>Home</span> }.into_any(),
            Crumbs::Post { title } => {
                view! { <a href="/">Home</a> | <span>{title}</span> }.into_any()
            },
        }
    }
}

#[component]
fn Crumbs(crumbs: Resource<Crumbs>) -> impl IntoView {
    view! {
        <p>
            <Suspense>
                {move || crumbs.get().unwrap_or_default().into_view() }
            </Suspense>
        </p>
    }
}

#[server]
async fn list_posts() -> Result<Vec<(u64, Post)>, ServerFnError> {
    Ok(crate::db::all_posts().await.collect())
}

#[server]
async fn post_by_id(id: u64) -> Result<Post, ServerFnError<String>> {
    crate::db::post_by_id(id).await.ok_or(ServerFnError::WrappedServerError(format!("Post not found: {id}")))
}

/// Renders the home page with list of posts.
#[component]
fn HomePage() -> impl IntoView {
    let crumbs = use_context::<AsyncWriteSignal<Crumbs>>().unwrap();
    crumbs.set(Crumbs::Home);

    let posts = Resource::new(|| (), |_| list_posts());

    view! {
        <Title text="Welcome to my blog!" />
        <Suspense>
            <ul>
            {move || Suspend::new(async move {
                posts
                    .await
                    .into_iter()
                    .map(|src| {
                        view! {
                            <For
                                each=move || src.clone()
                                key=|(id, _)| *id
                                children=|(id, post)| view!{
                                    <li><a href=format!("/post/{id}")>{post.title}</a></li>
                                }
                            />
                        }
                    })
                    .collect_view()
            })}
            </ul>
        </Suspense>
    }
}

#[derive(Clone, Copy, Params, PartialEq)]
struct PostRequest {
    id: Option<u64>,
}

#[component]
fn PostPage() -> impl IntoView {
    let params = use_params::<PostRequest>();
    let post = Resource::new(move || params.read().as_ref().ok().and_then(|pid| pid.id), |post_id| async move {
        match post_id {
            Some(id) => {
                let post_res = post_by_id(id).await;

                // Set crumbs
                let crumbs = use_context::<AsyncWriteSignal<Crumbs>>().unwrap();
                match &post_res {
                    Ok(post) => crumbs.set(Crumbs::Post { title: post.title.clone() }),
                    Err(_) => crumbs.set(Crumbs::Home),
                }

                post_res.map_err(|err| err.to_string())
            }
            None => Err(format!("Invalid URL"))
        }
    });

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(post) => {
                        let body = post.body.lines().map(|line| view! { <p>{ line.to_string() }</p> }).collect_view();
                        view! {
                            <Title text=post.title.clone() />
                            <h1>{post.title}</h1>
                            {body}
                        }.into_any()
                    }
                    Err(err) => view! {
                        <h1>Error: {err}</h1>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}
#![allow(non_snake_case, unused)]

#[cfg(feature = "server")]
mod db;

use std::{marker::PhantomData, sync::Arc};

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[derive(Clone)]
pub struct AppState<T> {
    client: Arc<T>,
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {

    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus_web::launch::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    {
        use axum::routing::*;
        use axum::response::Html;
        use axum::extract::State;
        use axum::Json;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let prisma_client: db::PrismaClient = db::new_client().await.unwrap();

                #[axum::debug_handler]
                async fn handler(State(state): State<db::PrismaClient>) -> Json<Option<db::system_user::Data>> {
                    let prisma = state.clone();
                    let first = prisma.system_user().find_first(vec![])
                        .exec().await.unwrap();
                    match first {
                        Some(user) => {
                            println!("{:?}", user);
                            Json(Some(user))
                        }
                        None => {
                            println!("No user found");
                            Json(None)
                        }
                    }
                }
                let apis = Router::new()
                    .route("/data", get(handler))
                    ;

                // let app_state: AppState<db::PrismaClient> = AppState {
                //     client: Arc::new(prisma_client),
                // };
                // build our application with some routes
                let app = Router::new()
                    .nest("/apis", apis)
                    .with_state(prisma_client)
                    // Server side render the application, serve static assets, and register server functions
                    .serve_dioxus_application(ServeConfig::builder().build(), || {
                        VirtualDom::new(App)
                    })
                    .await
                    ;
    
                // run it
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 7000));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }

}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        println!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

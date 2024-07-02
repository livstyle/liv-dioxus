#![allow(non_snake_case, unused)]

#[cfg(feature = "server")]
mod db;

#[cfg(feature = "server")]
mod api;

use std::{marker::PhantomData, sync::{Arc}};

#[cfg(feature = "server")]
use tokio::sync::Mutex;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

mod npay;

const _STYLE: &str = manganis::mg!(file("assets/tailwind.css"));

pub struct AppStateE<T> {
    client: Arc<T>,
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<db::PrismaClient>>,
}

fn main() {

    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus_web::launch::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    {
        use axum::routing::*;
        use axum::response::Html;
        use axum::extract::{State, Extension};
        use axum::Json;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let prisma_client: db::PrismaClient = db::new_client().await.unwrap();
                let app_state = AppState {
                    db: Arc::new(Mutex::new(prisma_client)),
                };

                // build our application with some routes
                let app = Router::new()
                    // .with_state(app_state)
                    // Server side render the application, serve static assets, and register server functions
                    .serve_dioxus_application(ServeConfig::builder().build(), || {
                        VirtualDom::new(App)
                    })
                    .await
                    .layer(Extension(app_state))
                    ;
    
                // run it
                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 7000));
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
            class: "text-gray-400 body-font",
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
            npay::view::Main {}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    let headers: axum::http::HeaderMap = extract().await?;
    println!("{:?}", headers);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    let headers: axum::http::HeaderMap = extract().await?;
    println!("{:?}", headers[axum::http::header::USER_AGENT]);
    Ok("Hello from the server!".to_string())
}


#[server(name=PayAgent, endpoint="mobile/pay_agent", encoding="getjson")]
pub async fn pay_agent() -> Result<String, ServerFnError> {
    let headers: axum::http::HeaderMap = extract().await?;
    // println!("{:?}", headers);
    println!("USER_AGENT ===> {:?}", headers[axum::http::header::USER_AGENT]);

    // redirect to the real page
    let url = "https://www.baidu.com";
    let response = axum::response::Redirect::temporary(url);
    // Ok(response.into_response().body().await.unwrap())

    Ok("{\"a\":1}".to_string())
}
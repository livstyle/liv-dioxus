
#[cfg(feature="server")]
pub mod auth;

#[cfg(feature="server")]
use axum::{
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    Json, Router,
    response::Redirect,
    extract::{Request, Path, Extension, Query},
};
#[cfg(feature="server")]
use serde::{Deserialize, Serialize};

#[cfg(feature="server")]
use crate::AppState;

#[cfg(feature="server")]
pub async fn jf_callback(Extension(state): Extension<AppState>, headers: HeaderMap, Query(query): Query<std::collections::HashMap<String, String>>) -> String {
    use std::collections::HashMap;

    let agent = headers.get("User-Agent").unwrap().to_str().unwrap();
    let is_weixin = agent.contains("Weixin");

    println!("jf_callback请求参数 {:?}", query);
    
    // Redirect::to(format!("https://sh.livstyle.cn/").as_str())
    format!("https://sh.livstyle.cn/")
}
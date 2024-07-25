pub mod mobile;

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
pub async fn scan_code(Extension(state): Extension<AppState>, headers: HeaderMap)-> axum::response::Redirect {
    let agent = headers.get("User-Agent").unwrap().to_str().unwrap();
    let is_weixin = agent.contains("Weixin");
    
    if is_weixin {
        // let redirect_uri = "https://sh.livstyle.cn/";
        let redirect_uri = "https%3A%2F%2Fsh.livstyle.cn%2F";
        let uri = format!("https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope=snsapi_base&state=WEIXIN#wechat_redirect", "wxc36eb8c96f92fefe", redirect_uri);
        println!("redirect to {}", uri.clone());
        Redirect::to(uri.as_str())
    } else {
        let is_ali = agent.contains("Alipay");
        if is_ali {
            Redirect::to(format!("https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id={}&scope=auth_user&redirect_uri={}", "", "").as_str())
        } else {
            Redirect::to(format!("https://www.livstyle.cn").as_str())
        } // https://sh.livstyle.cn/jf_callback
    }
}

#[cfg(feature="server")]
pub async fn redirect(Extension(state): Extension<AppState>, headers: HeaderMap)-> axum::response::Redirect {
    let agent = headers.get("User-Agent").unwrap().to_str().unwrap();
    let is_weixin = agent.contains("Weixin");
    
    Redirect::to(format!("https://sh.livstyle.cn/").as_str())
}


use std::{collections::HashMap};

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature= "server")]
use crate::db::payment_record::SetParam;

#[component]
pub fn Main(code: ReadOnlySignal<String>, state: ReadOnlySignal<String>) -> Element {
    let mut amount = use_signal(|| 0);
    rsx! {
        div {
            class: "ml-4 flex flex-col items-center justify-center h-screen",
            h2 {
                class: "text-2xl font-bold mb-4 text-blue-500",
                "code: {code}"
            }
            h2 {
                class: "text-2xl font-bold mb-4 text-blue-500",
                "state: {state}"
            }
            h2 { 
                class: "text-2xl font-bold mb-4 text-red-500",
                "请输入数字" 
            }
            input {
                value: {amount},
                oninput: move |evt| {
                    if let Some(am) = evt.value().parse::<i64>().ok() {
                        if am < 0 {
                            amount.set(0);
                        } else {
                            amount.set(am);
                        }
                    } else {
                        amount.set(0);
                    }
                }
            }
            br {}
            button {
                class: "bg-orange-100 rounded-md w-[60px]",
                onclick: move |_| async move {
                    // 这里应该调用支付接口
                    println!("支付 {} 元", amount);
                    println!("code: {}", code());
                    if let Ok(result) = union_payment(amount(), code(), state()).await {
                        println!("支付结果: {}", result);
                    }
                    else {
                        println!("支付失败");
                    }
                },
                "支付"
            }
        }
    }
}

#[server(name=Payment, endpoint="/payment")]
pub async fn payment(amount: i64) -> Result<String, ServerFnError> {
    let app_state: axum::extract::Extension<crate::AppState> = extract().await?;
    // 这里应该调用支付接口
    let r = app_state.db.lock().await.payment_record().create(vec![SetParam::SetAmount(amount)]).exec().await;
    println!("payment 支付结果: {:?}", r);
    Ok(format!("支付 {} 元成功", amount))
}

#[component]
pub fn LivPay(code: ReadOnlySignal<String>, state: ReadOnlySignal<String>) -> Element {
    let mut amount = use_signal(|| 0);
    let mut pay_res = use_signal(|| "".to_string());
    rsx! {
        div {
            class: "ml-4 flex flex-col items-center justify-center h-screen",
            h2 {
                class: "text-2xl font-bold mb-4 text-red-500",
                "页面"
            }
            p {
                class: "text-gray-500 mb-4",
                "类型: {state}"
            }
            p {
                class: "text-gray-500 mb-4",
                "CODE: {code}"
            }
            h2 {
                class: "text-2xl font-bold mb-4 text-red-500",
                "请输入数字"
            }
            input {
                class: "border border-gray-300 rounded-md p-2 mb-4",
                value: {amount},
                oninput: move |evt| {
                    if let Some(am) = evt.value().parse::<i64>().ok() {
                        if am < 0 {
                            amount.set(0);
                        } else {
                            amount.set(am);
                        }   
                        
                    } else {
                        amount.set(0);
                    }
                }
            }
            br {}
            button {
                class: "bg-orange-100 rounded-md w-[60px]",
                onclick: move |_| async move {
                    // 这里应该调用支付接口
                    // let code_a = code();
                    // let state_a = state();
                    let amount_a = amount();
                    let pr = payment(amount_a).await;
                    println!("支付结果pr : {:?}", pr);
                    // if let Ok(result) = union_payment(amount_a, code_a, state_a).await {
                    //     println!("union_payment支付结果: {}", result);
                    //     // pay_res.set(result);
                    // } else {
                    //     println!("union_payment支付失败");
                    // }
                },
                "支付"
            }
        }
    }
}

#[server(name=UnionPayment, endpoint="/union_payment")]
pub async fn union_payment(amount: i64, pay_type: String, code: String) -> Result<String, ServerFnError> {

    let app_state: axum::extract::Extension<crate::AppState> = extract().await?;

    // 这里应该调用支付接口
    // let r = app_state.db.lock().await.union_payment_record().create(vec![SetParam::SetAmount(amount)]).exec().await;
    println!("进入 union_payment 开始支付 {} 元, code: {}, pay_type: {}", amount, code, pay_type);
    let pay_r = wx_payment(amount, code).await;
    println!("支付结果pay_r : {:?}", pay_r);
    Ok("success".to_string())

}

// {
//     "access_token":"ACCESS_TOKEN",
//     "expires_in":7200,
//     "refresh_token":"REFRESH_TOKEN",
//     "openid":"OPENID",
//     "scope":"SCOPE",
//     "is_snapshotuser": 1,
//     "unionid": "UNIONID"
//   }

#[derive(Debug, Deserialize, Serialize)]
pub struct WxUser {
    access_token: Option<String>,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
    openid: Option<String>,
    scope: Option<String>,
    is_snapshotuser: Option<i64>,
    unionid: Option<String>,
    errcode: Option<i64>,
    errmsg: Option<String>,
}

pub async fn wx_payment(amount: i64, code: String) -> Result<String, anyhow::Error> {
    // 获取身份信息 openid
    let wx_user_url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", "wxc36eb8c96f92fefe", "3fa95616520edbdbffc118aaac0539da", code);
    println!("wx_user_url: {}", wx_user_url);
    let resp = reqwest::get(wx_user_url)
    .await?
    .json::<WxUser>() 
    .await?
    ;
    println!("获取微信OpenId结果: {resp:#?}");
    // 下单
    // 支付
    Ok(format!("支付 {} 元成功", amount))
}
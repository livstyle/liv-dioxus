
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[cfg(feature= "server")]
use crate::db::payment_record::SetParam;

#[component]
pub fn Main() -> Element {
    let mut amount = use_signal(|| 0);
    rsx! {
        div {
            class: "ml-4 flex flex-col items-center justify-center h-screen",
            h2 { 
                class: "text-2xl font-bold mb-4 text-red-500",
                "请输入金额" 
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
                    if let Ok(result) = payment(amount()).await {
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
    println!("支付结果: {:?}", r);
    Ok(format!("支付 {} 元成功", amount))
}

#[component]
pub fn LivPay(code: ReadOnlySignal<String>, state: ReadOnlySignal<String>) -> Element {
    let mut amount = use_signal(|| 0);
    rsx! {
        div {
            class: "ml-4 flex flex-col items-center justify-center h-screen",
            h2 {
                class: "text-2xl font-bold mb-4 text-red-500",
                "支付页面"
            }
            p {
                class: "text-gray-500 mb-4",
                "支付状态: {state}"
            }
            p {
                class: "text-gray-500 mb-4",
                "支付CODE: {code}"
            }
            h2 {
                class: "text-2xl font-bold mb-4 text-red-500",
                "请输入金额"
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
                class: "bg-orange-100 rounded-md",
                onclick: move |_| async move {
                    // 这里应该调用支付接口
                    println!("支付 {} 元", amount);
                    if let Ok(result) = payment(amount()).await {
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

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[cfg(feature= "server")]
use crate::db::payment_record::SetParam;

#[component]
pub fn Main() -> Element {
    let mut amount = use_signal(|| 0);
    rsx! {
        div {
            h2 { "请输入金额" }
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
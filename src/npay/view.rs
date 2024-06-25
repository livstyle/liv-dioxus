
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[component]
pub fn Main() -> Element {
    let mut ammount = use_signal(|| 0);
    rsx! {
        div {
            h2 { "请输入金额" }
            input {
                value: {ammount},
                oninput: move |evt| ammount.set(evt.value().parse::<i64>().unwrap())
            }
        }
    }
}
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
pub fn t () {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}


#[server(name = SomeStructName, endpoint = "mobile/user")]
pub async fn user() -> Result<String, ServerFnError> {
    Ok("wwww".to_string())
}
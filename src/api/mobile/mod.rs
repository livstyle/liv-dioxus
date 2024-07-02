use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
pub fn t () {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}


#[server(name=SomeStructName, endpoint="mobile/user")]
pub async fn user() -> Result<String, ServerFnError> {
    let headers: axum::http::HeaderMap = extract().await?;
    println!("{:?}", headers);
    Ok("wwww".to_string())
}

// #[server(name=PayAgent, endpoint="mobile/pay_agent", encoding="getjson")]
// pub async fn pay_agent() -> Result<String, ServerFnError> {
//     let headers: axum::http::HeaderMap = extract().await?;
//     println!("{:?}", headers);
//     println!("{:?}", headers[axum::http::header::USER_AGENT]);
//     Ok("{\"a\":1}".to_string())
// }


// Mozilla/5.0 (Linux; Android 11; ZTE A2322 Build/RKQ1.210907.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/105.0.5195.148 MYWeb/0.11.0.240621112601 UWS/3.22.2.9999 UCBS/3.22.2.9999_220000000000 Mobile Safari/537.36 NebulaSDK/1.8.100112 Nebula AlipayDefined(nt:WIFI,ws:360|0|3.0,ac:sp) AliApp(AP/10.6.10.8000) AlipayClient/10.6.10.8000 Language/zh-Hans useStatusBar/true isConcaveScreen/true Region/CNAriver/1.0.0 DTN/2.0

// Mozilla/5.0 (Linux; Android 11; ZTE A2322 Build/RKQ1.210907.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/122.0.6261.120 Mobile Safari/537.36 XWEB/1220099 MMWEBSDK/20240404 MMWEBID/8656 MicroMessenger/8.0.49.2600(0x2800315A) WeChat/arm64 Weixin NetType/WIFI Language/zh_CN ABI/arm64
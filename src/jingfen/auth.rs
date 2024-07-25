
// 获取接口数据的统一请求方式，这里封装了token验证方法

pub fn fetch() -> Result<(), anyhow::Error> {
    // TODO
    Ok(())
}


fn get_token() -> Result<String, anyhow::Error> {
    let APP_KEY = "fbb4addc89c396bd0276b3e575b09685";
    let APP_SECRET = "5a902ab1a9e04b6db7b0aa67828ab8161616";
    let url = "https://api.jd.com/routerjson";
    // TODO
    Ok("get token".to_owned())
}


fn get_code() -> Result<String, anyhow::Error> { 

    // https%3A%2F%2Fsh.livstyle.cn%2Fjf_callback

    // https://open-oauth.jd.com/oauth2/to_login?app_key=fbb4addc89c396bd0276b3e575b09685&response_type=code&redirect_uri=https%3A%2F%2Fsh.livstyle.cn%2Fjf_callback&state=JING_FEN&scope=snsapi_base

    let url = "https://open-oauth.jd.com/oauth2/to_login";
    let uu = "https://open-oauth.jd.com/oauth2/to_login?app_key=fbb4addc89c396bd0276b3e575b09685&response_type=code&redirect_uri=http%3A%2F%2Fkepler.jd.com%2Foauth%2Fcode.do&state=20180416&scope=snsapi_base";
    let mut client = reqwest::blocking::Client::builder();
    client = client.redirect(reqwest::redirect::Policy::none());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.82 Safari/537.36"));
    client = client.default_headers(headers);

    let client = client.build().unwrap();

    let res = client.get(uu).send().unwrap();
    let _headers = res.headers();
    // let _text = res.text().unwrap();
    println!("{:?}", _headers);

    // println!("{:?}", _text);
    Ok("get token".to_owned())
}

// TODO: 实现token验证方法
// pub fn auth() -> Result<()> { Ok(()) }
// 缓存token，避免每次请求都去获取
// let token = get_token().unwrap();
// TODO: 使用token进行接口请求
// fetch::<T>().unwrap()

#[cfg(feature = "server")]
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    #[test] 
    fn test_auth() { 
        let d = get_code();
        println!("{:?}", d);
    }
}
// 获取接口数据的统一请求方式，这里封装了token验证方法
pub fn fetch<T>() -> Result<T> {
    // TODO
    Err(Error::new(ErrorKind::Other, "fetch auth"))
}

fn get_token() -> Result<String> {
    let APP_KEY = "fbb4addc89c396bd0276b3e575b09685";
    let APP_SECRET = "5a902ab1a9e04b6db7b0aa67828ab8161616";
    let url = "https://api.jd.com/routerjson";
    // TODO
    Err(Error::new(ErrorKind::Other, "get token"))
}

// TODO: 实现token验证方法
// pub fn auth() -> Result<()> { Ok(()) }
// 缓存token，避免每次请求都去获取
// let token = get_token().unwrap();
// TODO: 使用token进行接口请求
// fetch::<T>().unwrap()

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    
}
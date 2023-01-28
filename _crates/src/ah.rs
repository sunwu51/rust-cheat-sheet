use anyhow::anyhow;

/// 一般通过Error特征对象如下，来接收所有的错误。
pub fn f1() -> Result<String, Box<dyn std::error::Error>> {

    let v = vec!["1", "2"];
    Ok(v.get(3).ok_or(std::fmt::Error)?
        .to_string())

}

/// 有了anyhow，不光可以接收Error特征的，甚至接收任何类型
/// anyhow::Result<T> 就是Result<T, anyhow::Error>的别名
/// pub type Result<T, E = Error> = core::result::Result<T, E>;
/// anyhow!(字符串)可以anyhow::Error类型
pub fn f2() -> anyhow::Result<String> {
    let v = vec!["1", "2"];
    // 返回Ok时和原来逻辑一致即可，?可以被anyhow捕捉
    // Ok(v.get(3).ok_or(std::fmt::Error)?
    //     .to_string())

    // 返回Err时，不能自己new，必须anyhow::Error
    // Err(anyhow!("error in f2"))

    // 需要包装成anyhow::Error
    Err(anyhow::Error::new(std::fmt::Error))
}


use std::num::ParseIntError;
use thiserror::Error;

// f1功能是 当字符串长度<3，那么转为i32
// 否则返回转换错误
fn f1(s: &str) -> Result<i32, MyError> {
    if (s.len() >= 3) {
        return Err(MyError::LengthError(s.to_string(), s.len()))
    }
    s.parse::<i32>().map_err(|e| MyError::ParseError(e))
}

#[derive(Debug, Error, PartialEq)]
enum MyError {
    #[error("转换出错")] // for display
    ParseError(#[from] ParseIntError),

    #[error("字符串 `{0}` 长度 `{1}` 过长")]
    LengthError(String, usize),

    #[error("未知错误")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        let res = f1("12");
        assert_eq!(res, Ok(12));
        println!("{:?}", res);


        let res = f1("123");
        assert_eq!(res.is_err(), true);
        println!("{}", res.err().unwrap()); // 字符串 `123` 长度 `3` 过长

        let res = f1("ab");
        assert_eq!(res.is_err(), true);
        println!("{:?}", res);

    }
}
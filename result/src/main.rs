use std::{io, string::ParseError};

use derive_error::Error;

/// Result与Option类似，也是一个enum，Option是要么包裹一个T，要么为空
/// Result是要么包裹一个T，要么包裹一个Error。core::result::Result是preload的不需要use

fn main() {
    // 1 声明Result的时候一般需要显示声明类型，因为不管是Ok还是Err变体，都只能推断T或E中的一个
    //  或者在方法签名中声明了返回值的类型，也可以直接返回Ok(xx)
    let res: Result<i32, io::Error> = Ok(1);
    let res: Result<i32, ParseError> = Ok(1);

    // 2 在方法签名中Error，如果是有多种类型，例如可能是io错误，也可以能是string::ParseError，可以有如下两方案
    //   - 2.1 用万能的特征对象，只要是实现了std::error::Error的错误都可以
    fn f1()-> Result<String, Box<dyn std::error::Error>> {
        // 显式返回错误需要用Box包裹，如果是?来捕捉，则会自动打包Box
        Err(Box::new(io::Error::new(io::ErrorKind::Other, "xx")))
    };
    //   - 2.2 自定义Error类型，拥有两个变体，需要引入第三方宏derive-error = "0.0.4"
    //      这里使用的是derive_error，但是推荐使用更强大好用的thiserror，请查看thiserror相关章节
    #[derive(Debug, Error)] // Error需要和Debug一起放在派生中，这样就继承自std::error::Error符合rust生态了
    enum MyError {
        IoError(io::Error),
        ParseError(ParseError),
    } 
    fn f2()-> Result<String, MyError> { Ok("1".to_owned())};

    fn f3()-> Result<String, Box<dyn std::error::Error>> { 
        Err(Box::new(MyError::IoError(io::Error::new(io::ErrorKind::Other, "xx"))))
    }
    // 3 ?可以自动捕捉错误，并作为Err返回
    fn f4()-> Result<String, Box<dyn std::error::Error>> { 
        Ok(f2()?)
    }

    ()

}

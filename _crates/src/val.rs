use lazy_static::lazy_static;

// 常量const
//  一般是mod范围声明，这样pub的还可以被外部访问，也可以函数内声明
pub const SIZE: i32= 10;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// 静态量static
//  与const最大的不同在于static是可以改的，可以声明为mut
//  只不过对于static来说是全局的，修改需要在unsafe中，因为可能有多线程修改
pub static mut NEW_SIZE: i32 = 20;

// 第三方包lazy_static提供的宏
//  static只能用常量进行声明，不够灵活，因为初始化是编译期，所以很多变量没法访问
//  lazy_static实现了，可以在第一次被访问的时候进行初始化，并且只初始化一次的限制
lazy_static!(
    pub static ref NEW_VERSION: String = {
        println!("lazy init");
        let mut v = String::new();
        v.push_str(VERSION);
        v
    };
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(SIZE, 10);
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn test2() {
        unsafe {
            assert_eq!(NEW_SIZE, 20);
            NEW_SIZE = 30;
            assert_eq!(NEW_SIZE, 30);
        }
    }

    #[test]
    fn test3() {
        assert_eq!(NEW_VERSION.as_str(), "0.1.0");
    }
}
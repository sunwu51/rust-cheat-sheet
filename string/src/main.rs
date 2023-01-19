fn main() {
    // str实现了eq，所以可以直接用==判断字符串的相等
    // 也实现了PartialOrd，可以进行大小比较的
    let b = "123" == "123";

    // &str 与 String 互转
    let s: &str = "123";
    let s: String = s.to_string();
    let s: &str = s.as_str();

    // &[u8] 与 vec<u8>互转
    let arr: &[u8] = s.as_bytes();
    let vec: Vec<u8> = arr.to_vec();
    let arr: &[u8] = vec.as_slice();

    // 以上是最基础的同类转换，下面函数则是字节码与字符串互转的。
    // 尝试通过as_slice as_bytes as_str to_vec to_string来进行快速转换
    // 如果不好使则考虑用String或者str包下的from_utf8方法

    // string util methods
    // 字符串拼接, String类型的+操作符， 或者mut的push_str
    let mut x = "123".to_string();
    x.push_str("aaa");
    let x = x + "aaa";

    // 字符串截断，通过[s..e]截取得到的类型是str，str不能直接使用在前加&，成为&str字面量使用
    let xx = "123";
    let xx = &xx[0..1];

    let s: &str = "1,2,3,4,5";
    // split 拆分，split返回一个内部的Split类型，一个经验是这些内部类型都是迭代器
        // 可以通过collect方法转为Vec 
    let v: Vec<&str> = s.split(",").collect();
    let b: bool = s.starts_with("1");
    let b: bool = s.ends_with("1");


    // rust字符串utf8编码，因而s.len()是字节数非字符数，字符操作需要转char数组
    let cs: Vec<char> = s.chars().collect();

    // 函数的接收参数类型请使用 &str类型，而返回类型 请使用String
        // 因为String是str的智能指针，当一个函数需要 X 类型的参数，那么我们可以传入
        // X类型或者将其封装的智能指针。
    let fnc1 = |s: &str|{ s.clone().to_owned() };

        // 智能指针String拥有str的所有方法，可以直接使用
    "xx".to_string().split(",");

        // 函数返回值一般不能是引用类型，所以不是&str，最好用String类型。例如str中的转小写方法
    let s: String = s.to_lowercase();

    
}


fn vec_to_string(vec: Vec<u8>) -> String {
    //String结构体内就是一个vec成员
    //pub struct String {
    //     vec: Vec<u8>,
    // }
    
    // 这样写就可以，但是这个写法不去校验utf8的合法性
    // String {vec: vec}

    // 这个封装的方法是校验了utf8合法性的
    String::from_utf8(vec).unwrap()
}

fn arr_to_string(arr: &[u8]) -> String {
    // String::from_utf8(arr.to_vec()).unwrap() 可以先转成vec再转string
    std::str::from_utf8(arr).unwrap().to_string() // 或者用std::str库的from_utf8方法接收的是数组参数
}

fn string_to_vec(s: String) -> Vec<u8> {
    let u8arr : &[u8] = s.as_bytes();
    u8arr.to_vec()
}

fn str_to_vec(s: &str) -> Vec<u8> {
    let u8arr : &[u8] = s.as_bytes();
    u8arr.to_vec()
}


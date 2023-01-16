fn main() {
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
}


fn vec_to_string(vec: Vec<u8>) -> String {
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


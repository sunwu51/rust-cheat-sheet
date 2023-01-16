use std::fs;
use std::path::Path;

fn main() {

    // 1fs
    // 读文件
    let content: Vec<u8> = fs::read("./Cargo.toml").unwrap();
    // 直接读成字符串类型
    let content: String = fs::read_to_string("./Cargo.toml").unwrap();
    
    // 写文件
    fs::write("/tmp/1.txt", content).unwrap();

    // 对于u8 string的转换 可以查看string部分，这里不展开


    // 删除
    fs::remove_file("/tmp/1.txt");
    // fs::remove_dir_all("/tmp/xx"); //删除整个目录

    // 2path
    let path = Path::new(".");

    // 判断文件存在与否
    let exist: bool = path.exists();


    // 列出文件夹下的path列表
    if path.is_dir() {
        for ele in fs::read_dir(path).unwrap() {
            let s_path = ele.unwrap().path();
            println!("path: {}", s_path.file_name().unwrap().to_str().unwrap());
        }
    }

}

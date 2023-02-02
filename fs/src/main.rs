use std::fs::{self, File};
use std::io::Write;
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

    // 2 Path
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

    // 3 File 比较精确地操作文件， 先申请文件的读写追加等权限
    let mut file: File = fs::OpenOptions::new()
        .create(true) // 如果没有就先创建
        .read(true) // 读
        .truncate(true) // 操作文件之前先给清理了所有数据，必须是write模式才行
        .write(true) // 从第0个字节写文件，多次write是往后追加
        // .append(true) // 从最后一个字节往后追加，多次write是最后位置追加
        .open("foo.txt").unwrap();

    
    file.write("123".as_bytes()).unwrap();
    file.write("345".as_bytes()).unwrap();


    // 4 另一种访问目录的方式 read_dir
    let mut dir = fs::read_dir("tmp").unwrap();

    while let Ok(child) = dir.next().unwrap() {
        if child.metadata().unwrap().is_dir() {
            // 文件夹下的文件跳过
        } else {
            // 文件类型的直接打印
            println!("{}", child.path().as_os_str().to_str().unwrap());
        }
    }

}

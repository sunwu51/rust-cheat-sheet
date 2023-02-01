
// tokio::fs文件相关操作
async fn fs_read() {
    let vec = tokio::fs::read("Cargo.toml").await;
    let content: String = String::from_utf8(vec.unwrap()).unwrap();
    println!("{content}");
}

async fn fs_write() {
    tokio::fs::create_dir("tmp").await.unwrap();
    tokio::fs::write("tmp/1.txt", "content").await.unwrap();
    println!("写入完成");

    let mut dir = tokio::fs::read_dir("tmp").await.unwrap();
    let f = dir.next_entry().await.unwrap().unwrap().file_name();
    println!("{}", f.into_string().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fs_read_test() {
        // test不能是async的，要想用tokio就不能用宏语法糖了，得手写runtime的创建
        tokio::runtime::Builder::new_current_thread() // new_multi_thread则是默认的多线程版本
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                fs_read()
            )
    }

    #[test]
    fn fs_write_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                fs_write()
            )
    }
}
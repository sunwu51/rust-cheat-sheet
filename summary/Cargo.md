# Cargo与项目组织
# Cargo
`Cargo`是rust的包管理工具，也是项目管理工具，他相当于`maven`的功能。
## 1 创建、构建、运行相关指令
```bash
# 创建新的项目，会创建文件夹，默认是bin类型
$ cargo new my_project 
# 创建库项目，lib类型
$ cargo new --lib my_project 

# 在空目录中初始化项目
$ cargo init [--lib/bin] 

# 添加依赖
$ cargo add lazy_static 
# @指定版本，-F指定feature，crate/featurename
$ cargo add serde serde_json@1.0 -F serde/derive 

# 仅下载依赖
cargo fetch

# 构建 简写cargo b
$ cargo build [--release] [--target xxx]
# 运行 简写cargo r
$ cargo run
# 运行单元测试
$ cargo test
# 运行指定的测试函数
$ cargo test tests::test1

```
## unit test
对于一般的项目来说，可以在文件后半部分就写测试，也可以在代码同级目录创建测试代码。`#[test]`会在`cargo test`时运行，`#[cfg(test)]`这个不影响，可以没有。
```rs
#[cfg(test)]
mod tests {
    use super::*; // 将上一级都引入，后面进行测试
    #[test]
    fn test1() {
        println!("test1");
    }

    #[test]
    fn test2() {
        println!("test1");
    }
}
```
# 项目组织
`main.rs` / `lib.rs`放在src下，然后其他的功能最好放到文件夹之中，其中每个文件夹都要创建一个`mod.rs`。
```text
src:
    - main.rs
    - util
        - mod.rs
        - http_util.rs
        - encode_util.rs
    - service
        - mod.rs
        - my_service.rs
```
`main.rs`中需要，声明mod，同时起到`use`的作用。
```rs
mod util;
mod service;

fn main() {
    util::http_util::get_data();
}
```
文件夹util + mod.rs 等价于 一个util.rs文件。`mod.rs`一般就只是声明一下这个目录下的文件。
```rs
// util/mod.rs
pub mod http_util;
pub mod encode_util;
```
有了main中的`mod util`，就能找到这个`mod.rs`，这里面的`mod http_util`，最终使得`util::http_util::get_data`能调到对应的函数。



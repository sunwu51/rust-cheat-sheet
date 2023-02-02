use lazy_static::lazy_static;
use reqwest::Client;
use anyhow::Result;
use serde_json::{Value, json};

static URL: &str = "https://vercel-express-sunwu51.vercel.app/test";
lazy_static!{
    pub static ref HTTP_CLIENT: Client = Client::builder().build().unwrap();
}

async fn get() -> Result<String> {
    let txt = HTTP_CLIENT.get(URL)
        .header("a", "b")
        .query(&[("id", "1")])
        .send()
        .await?
        .text()
        .await?;
    Ok(txt)
}

async fn post_form()  -> Result<String> {
    let txt = HTTP_CLIENT.get(URL)
        .form(&[("k1", "v1"), ("k2", "v2")])
        .send()
        .await?
        .text()
        .await?;
    Ok(txt)
}


async fn post_json()  -> Result<Value> {
    // json! 生成Value类型可以作为匿名结构体非常方便
    let json: Value = HTTP_CLIENT.get(URL)
        .json(&json!({
            "k1": "v1"
        }))
        .send()
        .await?
        .json()
        .await?;
    Ok(json)
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                async {
                    println!("{}", get().await.unwrap());
                }
            );
    }

    #[test]
    fn post_form_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                async {
                    println!("{}", post_form().await.unwrap());
                }
            );
    }

    #[test]
    fn post_json_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                async {
                    println!("{}", post_json().await.unwrap());
                }
            );
    }
}
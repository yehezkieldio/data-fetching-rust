use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataItem {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub tweets: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data: Vec<DataItem>
}

pub async fn fetch_data() -> Result<Data, reqwest::Error> {
    let url = "https://raw.githubusercontent.com/zakiego/baku-hantam/main/src/lib/tweet/data.json";
    let response = reqwest::get(url).await?.json::<Data>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => {
            for item in data.data {
                println!("Title: {}", item.title);
                println!("Slug: {}", item.slug);
                println!("Description: {}", item.description);
                for tweet in item.tweets {
                    println!("Tweet: {}", tweet);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

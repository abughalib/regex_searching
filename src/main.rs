use models::*;
use std::{fs::File, io::Read};
use futures::executor::block_on;
use meilisearch_sdk::client::*;

pub mod models;

fn read_from_file() -> Vec<Product>{
    let mut file = File::open("predict_categories_dataset_ocrs.jsonl").ok().unwrap();

    let mut file_str: String = String::new();

    file.read_to_string(&mut file_str).unwrap();

    let mut products: Vec<Product> = Vec::new();

    for line in file_str.lines() {
        let categories: Product = match serde_json::from_str(line) {
            Ok(t) => t,
            Err(e) => panic!("Error: {:?}", e)
        };

        products.push(categories);
    }
    return products;
}

async fn add_data(products: &Vec<Product>) {

    const MEILISEARCH_HOST: &str = "http://127.0.0.1:7700";
    const MEILISEARCH_API_KEY: &str = "masterKey";

    let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);

    match client
    .index("categories")
    .add_documents(&products, Some("code"))
    .await {
        Ok(success) => {
            println!("{:?}", success);
        },
        Err(e)=>{
            println!("Error: {:?}", e);
        }
    }
}

fn main() {

    let products: Vec<Product> = read_from_file();

    block_on(async move {
        add_data(&products).await;


    });

}

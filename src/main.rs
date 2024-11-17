use reqwest::Client;
use serde::de::DeserializeOwned;
use std::error::Error;

use std::collections::HashMap;
use serde_json::json;

struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
}

trait ApiOperation<T: DeserializeOwned> {
    fn get(&self, api_client: &ApiClient, endpoint: &str) -> impl std::future::Future<Output = Result<T, Box<dyn Error>>>;
    fn post(&self, api_client: &ApiClient, endpoint: &str, data: &T) -> impl std::future::Future<Output = Result<T, Box<dyn Error>>>;
    fn put(&self, api_client: &ApiClient, endpoint: &str, data: &T) -> impl std::future::Future<Output = Result<T, Box<dyn Error>>>;
    fn delete(&self, api_client: &ApiClient, endpoint: &str) -> impl std::future::Future<Output = Result<(), Box<dyn Error>>>;
}

struct PostsEndpoint;

impl ApiOperation<HashMap<String, serde_json::Value>> for PostsEndpoint {
    async fn get(&self, api_client: &ApiClient, endpoint: &str) -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
        let response = api_client.client
            .get(format!("{}/{}/1", api_client.base_url, endpoint))
            .send()
            .await?;
        Ok(response.json().await?)
    }

    async fn post(&self, api_client: &ApiClient, endpoint: &str, data: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
        let response = api_client.client
            .post(format!("{}/{}", api_client.base_url, endpoint))
            .json(&data)
            .send()
            .await?;
        Ok(response.json().await?)
    }

    async fn put(&self, api_client: &ApiClient, endpoint: &str, data: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
        let response = api_client.client
            .put(format!("{}/{}/1", api_client.base_url, endpoint))
            .json(data)
            .send()
            .await?;
        Ok(response.json().await?)
    }

    async fn delete(&self, api_client: &ApiClient, endpoint: &str) -> Result<(), Box<dyn Error>> {
        let response = api_client.client
            .delete(format!("{}/{}/1", api_client.base_url, endpoint))
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Delete failed with status: {}", response.status()).into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let url = "https://jsonplaceholder.typicode.com";
    
    let api_client = ApiClient::new(&url);
    let posts_endpoint = PostsEndpoint;

    // GET request
    let res = posts_endpoint.get(&api_client, "posts").await?;
    println!("GET result:\n {:?}\n", res);

    // POST request
    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("New Post !!!"));
    data.insert("body".to_string(), json!("This is a new post."));
    let res = posts_endpoint.post(&api_client, "posts", &data).await?;
    println!("POST result:\n {:?}\n", res);

    // PUT request
    let mut data = HashMap::new();
    data.insert("title".to_string(), json!("Updated Post !!!"));
    data.insert("body".to_string(), json!("This post has been updated recently."));
    let res = posts_endpoint.put(&api_client, "posts", &data).await?;
    println!("PUT result:\n {:?}\n", res);

    // DELETE request
    posts_endpoint.delete(&api_client, "posts").await?;
    println!("DELETE successful\n");

    Ok(())
}
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;

pub struct IpfsStorage {
    client: IpfsClient,
}

impl IpfsStorage {
    pub fn new(api_url: &str) -> Self {
        let client = IpfsClient::new(api_url);
        Self { client }
    }

    pub async fn add_data(&self, data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        let data = Cursor::new(data);
        let res = self.client.add(data).await?;
        Ok(res.hash)
    }

    pub async fn get_data(&self, cid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let data = self.client.cat(cid).await?;
        Ok(data)
    }
}

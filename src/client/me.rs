use super::TogglClient;
use crate::{error::Result, model::me::Me};
use reqwest::Method;

pub struct MeClient {
    client: TogglClient,
}

impl MeClient {
    pub fn new(client: TogglClient) -> MeClient {
        MeClient { client }
    }

    pub fn get(&self, debug: bool) -> Result<Me> {
        self.client.request(debug, Method::GET, "me")
    }
}

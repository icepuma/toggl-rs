use super::TogglClient;
use crate::{
    error::Result,
    model::{self},
};
use reqwest::Method;

pub struct MeClient {
    client: TogglClient,
}

impl MeClient {
    pub fn new(client: TogglClient) -> MeClient {
        MeClient { client }
    }

    pub fn get_me(&self, debug: bool) -> Result<model::me::Me> {
        self.client.request(debug, Method::GET, "me")
    }

    pub fn get_me_clients(&self, debug: bool) -> Result<Vec<model::me::Client>> {
        self.client.request(debug, Method::GET, "me/clients")
    }

    pub fn get_me_features(&self, debug: bool) -> Result<Vec<model::me::Features>> {
        self.client.request(debug, Method::GET, "me/features")
    }

    pub fn get_me_location(&self, debug: bool) -> Result<model::me::Location> {
        self.client.request(debug, Method::GET, "me/location")
    }
}

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

    // https://developers.track.toggl.com/docs/api/me#put-me

    pub fn get_me_clients(&self, debug: bool) -> Result<Vec<model::me::Client>> {
        self.client.request(debug, Method::GET, "me/clients")
    }

    // https://developers.track.toggl.com/docs/api/me#post-closeaccount

    pub fn get_me_features(&self, debug: bool) -> Result<Vec<model::me::Features>> {
        self.client.request(debug, Method::GET, "me/features")
    }

    pub fn get_me_location(&self, debug: bool) -> Result<model::me::Location> {
        self.client.request(debug, Method::GET, "me/location")
    }

    pub fn get_me_logged(&self, debug: bool) -> Result<()> {
        self.client.empty_request(debug, Method::GET, "me/logged")
    }

    // https://developers.track.toggl.com/docs/api/me#get-lostpassword

    // https://developers.track.toggl.com/docs/api/me#post-lostpassword

    // https://developers.track.toggl.com/docs/api/me#post-lostpassword-1

    pub fn get_me_organizations(&self, debug: bool) -> Result<Vec<model::me::Organization>> {
        self.client.request(debug, Method::GET, "me/organizations")
    }

    // https://developers.track.toggl.com/docs/api/me#get-projects

    // https://developers.track.toggl.com/docs/api/me#get-projectspaginated

    // https://developers.track.toggl.com/docs/api/me#get-tags

    // https://developers.track.toggl.com/docs/api/me#get-tasks

    // https://developers.track.toggl.com/docs/api/me#get-trackreminders

    // https://developers.track.toggl.com/docs/api/me#get-webtimer
}

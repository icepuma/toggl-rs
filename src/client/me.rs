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

    /// https://developers.track.toggl.com/docs/api/me#get-me
    pub fn get_me(&self, debug: bool) -> Result<model::me::Me> {
        self.client.request(debug, Method::GET, "me")
    }

    // https://developers.track.toggl.com/docs/api/me#put-me

    /// https://developers.track.toggl.com/docs/api/me#get-clients
    pub fn get_me_clients(&self, debug: bool) -> Result<Vec<model::me::Client>> {
        self.client.request(debug, Method::GET, "me/clients")
    }

    // https://developers.track.toggl.com/docs/api/me#post-closeaccount

    /// https://developers.track.toggl.com/docs/api/me#get-features
    pub fn get_me_features(&self, debug: bool) -> Result<Vec<model::me::Features>> {
        self.client.request(debug, Method::GET, "me/features")
    }

    /// https://developers.track.toggl.com/docs/api/me#get-users-last-known-location
    pub fn get_me_location(&self, debug: bool) -> Result<model::me::Location> {
        self.client.request(debug, Method::GET, "me/location")
    }

    /// https://developers.track.toggl.com/docs/api/me#get-logged
    pub fn get_me_logged(&self, debug: bool) -> Result<()> {
        self.client.empty_request(debug, Method::GET, "me/logged")
    }

    // https://developers.track.toggl.com/docs/api/me#get-lostpassword

    // https://developers.track.toggl.com/docs/api/me#post-lostpassword

    // https://developers.track.toggl.com/docs/api/me#post-lostpassword-1

    /// https://developers.track.toggl.com/docs/api/me#get-organizations-that-a-user-is-part-of
    pub fn get_me_organizations(&self, debug: bool) -> Result<Vec<model::me::Organization>> {
        self.client.request(debug, Method::GET, "me/organizations")
    }

    /// https://developers.track.toggl.com/docs/api/me#get-projects
    pub fn get_me_projects(&self, debug: bool) -> Result<Vec<model::me::Project>> {
        self.client.request(debug, Method::GET, "me/projects")
    }

    /// https://developers.track.toggl.com/docs/api/me#get-projectspaginated
    pub fn get_me_projects_paginated(
        &self,
        debug: bool,
        start_project_id: Option<u32>,
        since: Option<u32>,
    ) -> Result<Vec<model::me::Project>> {
        let mut params = vec![];

        if let Some(start_project_id) = start_project_id {
            params.push(("start_project_id".to_string(), start_project_id.to_string()));
        }

        if let Some(since) = since {
            params.push(("since".to_string(), since.to_string()));
        }

        self.client
            .request_with_params(debug, Method::GET, "me/projects", &params)
    }

    // https://developers.track.toggl.com/docs/api/me#get-tags

    // https://developers.track.toggl.com/docs/api/me#get-tasks

    // https://developers.track.toggl.com/docs/api/me#get-trackreminders

    // https://developers.track.toggl.com/docs/api/me#get-webtimer
}

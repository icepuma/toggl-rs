use std::fmt::Debug;

use crate::error::{Result, TogglError};
use owo_colors::OwoColorize;
use reqwest::{blocking, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

pub const DEFAULT_BASE_URL: &str = "https://api.track.toggl.com/api/v9/";

pub struct TogglClient {
    base_url: Url,
    client: blocking::Client,
    api_token: String,
}

impl TogglClient {
    pub fn new(api_token: String) -> Result<TogglClient> {
        #[cfg(not(test))]
        let base_url = "https://api.track.toggl.com/api/v9/".parse()?;

        #[cfg(test)]
        let base_url = mockito::server_url().parse()?;

        let client = blocking::Client::new();

        Ok(TogglClient {
            base_url,
            client,
            api_token,
        })
    }

    fn base_request(&self, method: Method, uri: &str) -> Result<blocking::RequestBuilder> {
        let url = self.base_url.join(uri)?;

        Ok(self
            .client
            .request(method, url)
            .basic_auth(&self.api_token, Some("api_token")))
    }

    fn request<D: DeserializeOwned + Debug>(
        &self,
        debug: bool,
        method: Method,
        uri: &str,
    ) -> Result<D> {
        let request = self.base_request(method, uri)?;

        if debug {
            println!("{}", "Request:".bold().underline());
            println!("{:?}", request);
            println!();
        }

        let response = request.send()?;

        self.response(debug, response)
    }

    fn empty_request(&self, debug: bool, method: Method, uri: &str) -> Result<()> {
        let request = self.base_request(method, uri)?;

        if debug {
            println!("{}", "Request:".bold().underline());
            println!("{:?}", request);
            println!();
        }

        let response = request.send()?;

        self.empty_response(response)
    }

    fn request_with_body<D: DeserializeOwned + Debug, S: Serialize + Debug>(
        &self,
        debug: bool,
        method: Method,
        uri: &str,
        body: S,
    ) -> Result<D> {
        let request = self.base_request(method, uri)?.json(&body);

        if debug {
            println!("{}", "Request:".bold().underline());
            println!("{:?}", request);
            println!();
            println!("{:?}", &body);
            println!();
        }

        let response = request.send()?;

        self.response(debug, response)
    }

    fn response<D: DeserializeOwned + Debug>(
        &self,
        debug: bool,
        response: blocking::Response,
    ) -> Result<D> {
        if debug {
            println!("{}", "Response:".bold().underline());
            println!("{:?}", response);
            println!();
        }

        match response.status() {
            StatusCode::OK | StatusCode::CREATED if debug => match response.json() {
                Ok(json) => {
                    println!("{}", "Received JSON response:".bold().underline());
                    println!("{:?}", json);
                    println!();

                    Ok(json)
                }
                Err(err) => Err(TogglError::LibraryError(format!(
                    "Failed to deserialize JSON: {}",
                    err
                ))),
            },
            StatusCode::OK | StatusCode::CREATED => Ok(response.json()?),
            status => match response.text() {
                Ok(text) => Err(TogglError::LibraryError(format!("{} - {}", status, text))),
                Err(_) => Err(TogglError::LibraryError(format!("{}", status))),
            },
        }
    }

    fn empty_response(&self, response: blocking::Response) -> Result<()> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => Ok(()),
            status => match response.text() {
                Ok(text) => Err(TogglError::LibraryError(format!("{} - {}", status, text))),
                Err(_) => Err(TogglError::LibraryError(format!("{}", status))),
            },
        }
    }

    // me
    pub fn get_me(&self, debug: bool) -> Result<crate::model::me::Me> {
        self.request(debug, Method::GET, "me")
    }
}

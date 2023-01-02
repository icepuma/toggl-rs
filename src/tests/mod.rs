use crate::{client::TogglClient, error::Result};
use reqwest::Method;

pub mod me;

const API_TOKEN: &str = "cb7bf7efa6d652046abd2f7d84ee18c1";

fn with_mockito(
    method: Method,
    url: &str,
    status: usize,
    response: Option<serde_json::Value>,
    test: impl FnOnce(TogglClient) -> Result<()>,
) -> Result<()> {
    let mock = if let Some(response) = response {
        mockito::mock(method.as_str(), url)
            .with_status(status)
            .with_body(response.to_string())
            .expect(1)
            .create()
    } else {
        mockito::mock(method.as_str(), url)
            .with_status(status)
            .expect(1)
            .create()
    };

    let toggl_client = TogglClient::new(API_TOKEN.to_string())?;
    let result = test(toggl_client);

    mock.assert();

    result
}

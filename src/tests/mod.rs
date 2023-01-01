use reqwest::Method;

use crate::error::Result;

pub mod me;

const API_TOKEN: &str = "cb7bf7efa6d652046abd2f7d84ee18c1";

fn with_mockito(
    method: Method,
    url: &str,
    status: usize,
    response: serde_json::Value,
    test: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let mock = mockito::mock(method.as_str(), url)
        .with_status(status)
        .with_body(response.to_string())
        .expect(1)
        .create();

    let result = test();

    mock.assert();

    result
}

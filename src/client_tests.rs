use crate::{client::TogglClient, error::Result};
use pretty_assertions::assert_eq;
use serde_json::json;

const API_TOKEN: &str = "cb7bf7efa6d652046abd2f7d84ee18c1";

#[test]
fn get_me() -> Result<()> {
    let client = TogglClient::new(API_TOKEN.to_string())?;

    let response = json!({
        "api_token": API_TOKEN,
        "at": "2020-01-01T00:00:00+00:00",
        "beginning_of_week": 1,
        "country_id": 82,
        "created_at": "2020-01-01T00:00:00+00:00",
        "default_workspace_id": 1234567,
        "email": "hans.toggl@fkbr.org",
        "fullname": "Hans Toggl",
        "id": 1234567,
        "image_url": "https://assets.track.toggl.com/images/profile.png",
        "intercom_hash": null,
        "openid_email": null,
        "openid_enabled": false,
        "options": null,
        "timezone": "Europe/Berlin",
        "updated_at": "2020-01-01T00:00:00+00:00"
    });

    let get_me = mockito::mock("GET", "/me")
        .with_header(
            "Authorization",
            "Basic Y2I3YmY3ZWZhNmQ2NTIwNDZhYmQyZjdkODRlZTE4YzE6YXBpX3Rva2Vu",
        )
        .with_status(200)
        .with_body(response.to_string())
        .expect(1)
        .create();

    {
        let me = client.get_me(true)?;

        assert_eq!(Some(API_TOKEN.to_string()), me.api_token);
    }

    get_me.assert();

    Ok(())
}

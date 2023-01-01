use crate::{client::TogglClient, error::Result};
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::{with_mockito, API_TOKEN};

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

    with_mockito(Method::GET, "/me", 200, response, || {
        let me = client.get_me(true)?;

        assert_eq!(Some(API_TOKEN.to_string()), me.api_token);
        assert_eq!(1234567, me.default_workspace_id);

        Ok(())
    })
}

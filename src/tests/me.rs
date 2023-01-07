use crate::error::Result;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::{with_mockito, API_TOKEN};

#[test]
fn get_me() -> Result<()> {
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

    with_mockito(Method::GET, "/me", 200, Some(response), |toggl_client| {
        let me = toggl_client.me().get_me(true)?;

        assert_eq!(Some(API_TOKEN.to_string()), me.api_token);
        assert_eq!(1234567, me.default_workspace_id);

        Ok(())
    })
}

#[test]
fn get_me_clients() -> Result<()> {
    let response = json!([
      {
        "id": 1234567,
        "wid": 1234567,
        "archived": false,
        "name": "aaa",
        "at": "2022-10-03T15:47:31+00:00"
      },
      {
        "id": 7654321,
        "wid": 7654321,
        "archived": false,
        "name": "fsdklfkldskf",
        "at": "2022-10-03T21:21:29+00:00"
      }
    ]);

    with_mockito(
        Method::GET,
        "/me/clients?since=1673134409",
        200,
        Some(response),
        |toggl_client| {
            let me_clients = toggl_client.me().get_me_clients(true, Some(1673134409))?;

            assert_eq!(2, me_clients.len());
            assert_eq!(1234567, me_clients[0].id);
            assert_eq!(7654321, me_clients[1].id);

            Ok(())
        },
    )
}

#[test]
fn get_me_features() -> Result<()> {
    let response = json!([{
        "workspace_id": 1234567,
        "features": [
          {
            "feature_id": 0,
            "name": "free",
            "enabled": true
          },
          {
            "feature_id": 13,
            "name": "pro",
            "enabled": false
          },
          {
            "feature_id": 15,
            "name": "business",
            "enabled": false
          },
          {
            "feature_id": 50,
            "name": "scheduled_reports",
            "enabled": false
          },
          {
            "feature_id": 51,
            "name": "time_audits",
            "enabled": false
          },
          {
            "feature_id": 52,
            "name": "locking_time_entries",
            "enabled": false
          },
          {
            "feature_id": 53,
            "name": "edit_team_member_time_entries",
            "enabled": false
          },
          {
            "feature_id": 54,
            "name": "edit_team_member_profile",
            "enabled": false
          },
          {
            "feature_id": 55,
            "name": "tracking_reminders",
            "enabled": false
          },
          {
            "feature_id": 56,
            "name": "time_entry_constraints",
            "enabled": false
          },
          {
            "feature_id": 57,
            "name": "priority_support",
            "enabled": false
          },
          {
            "feature_id": 58,
            "name": "labour_cost",
            "enabled": false
          },
          {
            "feature_id": 59,
            "name": "report_employee_profitability",
            "enabled": false
          },
          {
            "feature_id": 60,
            "name": "report_project_profitability",
            "enabled": false
          },
          {
            "feature_id": 61,
            "name": "report_comparative",
            "enabled": false
          },
          {
            "feature_id": 62,
            "name": "report_data_trends",
            "enabled": false
          },
          {
            "feature_id": 63,
            "name": "report_export_xlsx",
            "enabled": false
          },
          {
            "feature_id": 64,
            "name": "tasks",
            "enabled": false
          },
          {
            "feature_id": 65,
            "name": "project_dashboard",
            "enabled": false
          },
          {
            "feature_id": 66,
            "name": "outlook_calendar_integration",
            "enabled": false
          },
          {
            "feature_id": 67,
            "name": "favorites",
            "enabled": false
          },
          {
            "feature_id": 68,
            "name": "multi_workspace",
            "enabled": false
          },
          {
            "feature_id": 69,
            "name": "goals",
            "enabled": false
          },
          {
            "feature_id": 70,
            "name": "recurring_projects",
            "enabled": false
          },
          {
            "feature_id": 71,
            "name": "only_admins_may_create_tags",
            "enabled": false
          },
          {
            "feature_id": 72,
            "name": "billable_rates",
            "enabled": false
          },
          {
            "feature_id": 73,
            "name": "historical_billable_rates",
            "enabled": false
          },
          {
            "feature_id": 74,
            "name": "split_time_entry",
            "enabled": false
          },
          {
            "feature_id": 75,
            "name": "focus_mode",
            "enabled": false
          },
          {
            "feature_id": 76,
            "name": "fixed_fee",
            "enabled": false
          },
          {
            "feature_id": 77,
            "name": "summary_audit",
            "enabled": false
          },
          {
            "feature_id": 78,
            "name": "archive_client",
            "enabled": false
          },
          {
            "feature_id": 79,
            "name": "reports_hide_weekends",
            "enabled": false
          },
          {
            "feature_id": 80,
            "name": "multi_calendar",
            "enabled": false
          },
          {
            "feature_id": 81,
            "name": "project_template",
            "enabled": false
          }
        ]
      }
    ]);

    with_mockito(
        Method::GET,
        "/me/features",
        200,
        Some(response),
        |toggl_client| {
            let me_features = toggl_client.me().get_me_features(true)?;

            assert_eq!("free", me_features[0].features[0].name);
            assert_eq!(true, me_features[0].features[0].enabled);

            Ok(())
        },
    )
}

#[test]
fn get_me_locations() -> Result<()> {
    let response = json!({
        "city": "New York",
        "city_lat_long": "40.730610,-73.935242",
        "state": "New York",
        "country_code": "US",
        "country_name": "United States",
    });

    with_mockito(
        Method::GET,
        "/me/location",
        200,
        Some(response),
        |toggl_client| {
            let me_location = toggl_client.me().get_me_location(true)?;

            assert_eq!("New York", me_location.city);
            assert_eq!("40.730610,-73.935242", me_location.city_lat_long);
            assert_eq!("New York", me_location.state);
            assert_eq!("US", me_location.country_code);
            assert_eq!("United States", me_location.country_name);

            Ok(())
        },
    )
}

#[test]
fn get_me_logged() -> Result<()> {
    with_mockito(Method::GET, "/me/logged", 200, None, |toggl_client| {
        let me_logged = toggl_client.me().get_me_logged(true);

        assert!(me_logged.is_ok());

        Ok(())
    })
}

#[test]
fn get_me_organizations() -> Result<()> {
    let response = json!([
        {
            "id": 1234567,
            "name": "My Organization",
            "pricing_plan_id": 0,
            "created_at": "2021-11-12T19:33:19.860863Z",
            "at": "2021-11-12T19:33:19.930603Z",
            "server_deleted_at": null,
            "is_multi_workspace_enabled": false,
            "suspended_at": null,
            "user_count": 1,
            "trial_info": {
              "trial": false,
              "trial_available": false,
              "trial_end_date": "2021-12-12T00:00:00Z",
              "next_payment_date": null,
              "last_pricing_plan_id": null
            },
            "is_chargify": false,
            "is_unified": false,
            "max_workspaces": 20,
            "admin": true,
            "owner": true
          }
    ]);

    with_mockito(
        Method::GET,
        "/me/organizations",
        200,
        Some(response),
        |toggl_client| {
            let me_organizations = toggl_client.me().get_me_organizations(true)?;

            assert_eq!(1234567, me_organizations[0].id);
            assert_eq!("My Organization", me_organizations[0].name);
            assert_eq!(0, me_organizations[0].pricing_plan_id);
            assert_eq!(1, me_organizations[0].user_count);
            assert_eq!(20, me_organizations[0].max_workspaces);
            assert_eq!(true, me_organizations[0].admin);
            assert_eq!(true, me_organizations[0].owner);

            Ok(())
        },
    )
}

#[test]
fn get_me_projects() -> Result<()> {
    let response = json!([
      {
        "id": 123456789,
        "workspace_id": 123456789,
        "client_id": null,
        "name": "aaaa",
        "is_private": true,
        "active": true,
        "at": "2023-01-02T23:02:17+00:00",
        "created_at": "2022-10-03T15:47:08+00:00",
        "server_deleted_at": null,
        "color": "#465bb3",
        "billable": false,
        "template": false,
        "auto_estimates": false,
        "estimated_hours": null,
        "rate": null,
        "rate_last_updated": null,
        "currency": null,
        "recurring": true,
        "recurring_parameters": [
          {
            "estimated_seconds": 0,
            "period": "monthly",
            "custom_period": null,
            "project_start_date": "2022-10-03",
            "parameter_start_date": "2023-01-02",
            "parameter_end_date": null
          }
        ],
        "current_period": {
          "start_date": "2022-12-03",
          "end_date": "2023-01-02"
        },
        "fixed_fee": null,
        "actual_hours": null,
        "start_date": "2022-10-03T00:00:00Z",
        "wid": 123456789,
        "cid": null
      }
    ]);

    with_mockito(
        Method::GET,
        "/me/projects?include_archived=true&since=1673134409",
        200,
        Some(response),
        |toggl_client| {
            let me_projects =
                toggl_client
                    .me()
                    .get_me_projects(true, Some(true), Some(1673134409))?;

            assert_eq!(123456789, me_projects[0].id);
            assert_eq!(123456789, me_projects[0].workspace_id);
            assert_eq!(None, me_projects[0].client_id);
            assert_eq!("aaaa", me_projects[0].name);
            assert_eq!(true, me_projects[0].is_private);
            assert_eq!(true, me_projects[0].active);

            Ok(())
        },
    )
}

#[test]
fn get_me_projects_paginated() -> Result<()> {
    let response = json!([
      {
        "id": 123456789,
        "workspace_id": 123456789,
        "client_id": null,
        "name": "aaaa",
        "is_private": true,
        "active": true,
        "at": "2023-01-02T23:02:17+00:00",
        "created_at": "2022-10-03T15:47:08+00:00",
        "server_deleted_at": null,
        "color": "#465bb3",
        "billable": false,
        "template": false,
        "auto_estimates": false,
        "estimated_hours": null,
        "rate": null,
        "rate_last_updated": null,
        "currency": null,
        "recurring": true,
        "recurring_parameters": [
          {
            "estimated_seconds": 0,
            "period": "monthly",
            "custom_period": null,
            "project_start_date": "2022-10-03",
            "parameter_start_date": "2023-01-02",
            "parameter_end_date": null
          }
        ],
        "current_period": {
          "start_date": "2022-12-03",
          "end_date": "2023-01-02"
        },
        "fixed_fee": null,
        "actual_hours": null,
        "start_date": "2022-10-03T00:00:00Z",
        "wid": 123456789,
        "cid": null
      }
    ]);

    with_mockito(
        Method::GET,
        "/me/projects?start_project_id=123456789&since=1673134409",
        200,
        Some(response),
        |toggl_client| {
            let me_projects = toggl_client.me().get_me_projects_paginated(
                true,
                Some(123456789),
                Some(1673134409),
            )?;

            assert_eq!(123456789, me_projects[0].id);
            assert_eq!(123456789, me_projects[0].workspace_id);
            assert_eq!(None, me_projects[0].client_id);
            assert_eq!("aaaa", me_projects[0].name);
            assert_eq!(true, me_projects[0].is_private);
            assert_eq!(true, me_projects[0].active);

            Ok(())
        },
    )
}

#[test]
fn get_me_tags() -> Result<()> {
    let response = json!([
      {
        "id": 1234,
        "workspace_id": 123456789,
        "name": "aaa",
        "at": "2022-10-03T15:44:20.424008Z"
      },
      {
        "id": 1235,
        "workspace_id": 123456789,
        "name": "aaaa",
        "at": "2022-10-03T15:49:09.73311Z"
      },
      {
        "id": 1236,
        "workspace_id": 123456789,
        "name": "aaaa, fdsfsd",
        "at": "2022-10-03T15:44:41.464544Z"
      },
      {
        "id": 1237,
        "workspace_id": 123456789,
        "name": "aaa,bbb",
        "at": "2022-10-03T15:43:46.015571Z"
      },
      {
        "id": 1238,
        "workspace_id": 123456789,
        "name": "klklk",
        "at": "2022-10-03T15:49:09.73311Z"
      }
    ]);

    with_mockito(
        Method::GET,
        "/me/tags?since=1673134409",
        200,
        Some(response),
        |toggl_client| {
            let me_tags = toggl_client.me().get_me_tags(true, Some(1673134409))?;

            assert_eq!(1234, me_tags[0].id);
            assert_eq!(123456789, me_tags[0].workspace_id);
            assert_eq!("aaa", me_tags[0].name);

            Ok(())
        },
    )
}

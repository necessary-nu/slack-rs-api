//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

//! Get info on members of your Slack team.

pub use crate::mod_types::users_types::*;
use crate::requests::SlackWebRequestSender;

/// Delete the user profile photo
///
/// Wraps https://api.slack.com/methods/users.deletePhoto

pub async fn delete_photo<R>(client: &R) -> Result<DeletePhotoResponse, DeletePhotoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: &[(&str, &str)] = &[];
    let url = crate::get_slack_url_for_method("users.deletePhoto");
    client
        .send(&url, &params[..])
        .await
        .map_err(DeletePhotoError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeletePhotoResponse>(&result)
                .map_err(|e| DeletePhotoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

pub async fn get_presence<R>(
    client: &R,

    request: &GetPresenceRequest<'_>,
) -> Result<GetPresenceResponse, GetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("user", request.user))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("users.getPresence");
    client
        .send(&url, &params[..])
        .await
        .map_err(GetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result)
                .map_err(|e| GetPresenceError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

pub async fn identity<R>(client: &R) -> Result<IdentityResponse, IdentityError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: &[(&str, &str)] = &[];
    let url = crate::get_slack_url_for_method("users.identity");
    client
        .send(&url, &params[..])
        .await
        .map_err(IdentityError::Client)
        .and_then(|result| {
            serde_json::from_str::<IdentityResponse>(&result)
                .map_err(|e| IdentityError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

pub async fn info<R>(
    client: &R,

    request: &InfoRequest<'_>,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("user", request.user))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("users.info");
    client
        .send(&url, &params[..])
        .await
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

pub async fn list<R>(client: &R, request: &ListRequest) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request
        .presence
        .map(|presence| ("presence", if presence { "1" } else { "0" }))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("users.list");
    client
        .send(&url, &params[..])
        .await
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive

pub async fn set_active<R>(client: &R) -> Result<SetActiveResponse, SetActiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params: &[(&str, &str)] = &[];
    let url = crate::get_slack_url_for_method("users.setActive");
    client
        .send(&url, &params[..])
        .await
        .map_err(SetActiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetActiveResponse>(&result)
                .map_err(|e| SetActiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

pub async fn set_presence<R>(
    client: &R,

    request: &SetPresenceRequest<'_>,
) -> Result<SetPresenceResponse, SetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("presence", request.presence))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("users.setPresence");
    client
        .send(&url, &params[..])
        .await
        .map_err(SetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPresenceResponse>(&result)
                .map_err(|e| SetPresenceError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

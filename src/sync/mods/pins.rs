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

pub use crate::mod_types::pins_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Pins an item to a channel.
///
/// Wraps https://api.slack.com/methods/pins.add

pub fn add<R>(client: &R, request: &AddRequest<'_>) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let timestamp = request.timestamp.as_ref().map(|t| t.to_param_value());
    let params = vec![
        Some(("channel", request.channel)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", &timestamp[..])),
    ];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("pins.add");
    client
        .send(&url, &params[..])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists items pinned to a channel.
///
/// Wraps https://api.slack.com/methods/pins.list

pub fn list<R>(client: &R, request: &ListRequest<'_>) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("channel", request.channel))];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("pins.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Un-pins an item from a channel.
///
/// Wraps https://api.slack.com/methods/pins.remove

pub fn remove<R>(
    client: &R,

    request: &RemoveRequest<'_>,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let timestamp = request.timestamp.as_ref().map(|t| t.to_param_value());
    let params = vec![
        Some(("channel", request.channel)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", &timestamp[..])),
    ];
    let params = params.into_iter().flatten().collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("pins.remove");
    client
        .send(&url, &params[..])
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

//! Functionality for sending requests to Slack.
use async_trait::async_trait;

use std::{borrow::Borrow, error};

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
#[async_trait]
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    async fn send<I, K, V, S>(&self, method: S, params: I) -> Result<String, Self::Error>
    where
        I: IntoIterator + Send,
        K: AsRef<str>,
        V: AsRef<str>,
        I::Item: Borrow<(K, V)>,
        S: AsRef<str> + Send;
}

#[cfg(feature = "reqwest")]
mod reqwest_support {
    use async_trait::async_trait;
    pub use reqwest::Error;
    use reqwest::{header::HeaderMap, ClientBuilder};
    use std::borrow::Borrow;

    use super::SlackWebRequestSender;

    type Client = reqwest::Client;

    #[async_trait]
    impl SlackWebRequestSender for Client {
        type Error = reqwest::Error;

        async fn send<I, K, V, S>(&self, method_url: S, params: I) -> Result<String, Self::Error>
        where
            I: IntoIterator + Send,
            K: AsRef<str>,
            V: AsRef<str>,
            I::Item: Borrow<(K, V)>,
            S: AsRef<str> + Send,
        {
            let mut url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");

            url.query_pairs_mut().extend_pairs(params);

            Ok(self.get(url).send().await?.text().await?)
        }
    }

    /// Provides a default `reqwest` client to give to the API functions to send requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # let token = "some_token";
    /// let client = slack_api::requests::default_client().unwrap();
    /// let params = Default::default();
    /// let response = slack_api::channels::list(&client, &token, &params);
    /// ```
    pub fn default_client(token: &str) -> Result<Client, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        Ok(ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap())
    }
}

#[cfg(feature = "reqwest")]
pub use self::reqwest_support::*;

use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TagGetTopTags<'a> {
    lastfm: &'a Lastfm,
    method: LastfmMethod,
}

impl<'a> TagGetTopTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TagGetTopTags {
            lastfm,
            method: LastfmMethod::TagGetTopTags,
        }
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        let builder = ParameterBuilder::new();

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}

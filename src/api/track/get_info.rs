use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TrackGetInfo<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub track: Option<String>,
    pub mbid: Option<String>,
    pub autocorrect: Option<bool>,
    pub username: Option<String>,
    method: LastfmMethod,
}

impl<'a> TrackGetInfo<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TrackGetInfo {
            lastfm,
            artist: None,
            track: None,
            mbid: None,
            autocorrect: Some(false),
            username: None,
            method: LastfmMethod::TrackGetInfo,
        }
    }

    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    pub fn track(mut self, track: &str) -> Self {
        self.track = Some(track.to_string());
        self
    }

    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn autocorrect(mut self, autocorrect: bool) -> Self {
        self.autocorrect = Some(autocorrect);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.mbid.is_none() && (self.artist.is_none()) {
            return Err(Error::Generic(
                "Either 'mbid' or 'artist' must be provided.".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("track", self.track)
            .add_optional("mbid", self.mbid)
            .add_optional("username", self.username)
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}

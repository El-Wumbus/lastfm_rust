use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, AlbumGetTagsResponse, Error, Lastfm, Result,
};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct AlbumGetTags<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub mbid: Option<String>,
    pub user: Option<String>,
    pub autocorrect: Option<bool>,
    method: LastfmMethod,
}

impl<'a> AlbumGetTags<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        AlbumGetTags {
            lastfm,
            artist: None,
            album: None,
            mbid: None,
            user: None,
            autocorrect: Some(false),
            method: LastfmMethod::AlbumGetTags,
        }
    }

    /// Sets the artist for the request.
    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    /// Sets the album name for the request.
    pub fn album(mut self, album: &str) -> Self {
        self.album = Some(album.to_string());
        self
    }

    /// Sets the mbid for the request.
    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    /// Sets the user for the request.
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    /// Sets whether to apply autocorrection.
    pub fn autocorrect(mut self, autocorrect: bool) -> Self {
        self.autocorrect = Some(autocorrect);
        self
    }

    /// Validates the request parameters.
    fn validate(&self) -> Result<()> {
        if self.mbid.is_none() && (self.artist.is_none() || self.album.is_none()) {
            return Err(Error::Generic(
                "Either 'mbid' or both 'artist' and 'album' must be provided.".to_string(),
            ));
        }
        Ok(())
    }

    /// Sends the request and retrieves the tags for the album.
    pub async fn send(self) -> Result<APIResponse<AlbumGetTagsResponse>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("album", self.album)
            .add_optional("mbid", self.mbid)
            .add_optional("user", self.user)
            .add_optional("autocorrect", self.autocorrect.map(|b| b.to_string()));

        let mut params = builder.build();

        let json_response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(json_response)
    }
}

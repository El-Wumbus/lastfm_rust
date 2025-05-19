use std::fmt;

mod album;
mod artist;
mod auth;
mod chart;
mod geo;
mod library;
mod parameter_builder;
mod tag;
mod track;
mod user;

pub use album::*;
pub use artist::*;
pub use auth::*;
pub use chart::*;
pub use geo::*;
pub use library::*;
pub use parameter_builder::*;
pub use tag::*;
pub use track::*;
pub use user::*;

#[derive(Debug, Clone)]
pub enum LastfmMethod {
    // Album Methods
    AlbumAddTags,
    AlbumGetInfo,
    AlbumGetTags,
    AlbumGetTopTags,
    AlbumRemoveTag,
    AlbumSearch,

    // Artist Methods
    ArtistAddTags,
    ArtistGetCorrection,
    ArtistGetInfo,
    ArtistGetSimilar,
    ArtistGetTags,
    ArtistGetTopAlbums,
    ArtistGetTopTags,
    ArtistGetTopTracks,
    ArtistRemoveTag,
    ArtistSearch,

    // Auth Methods
    AuthGetMobileSession,
    AuthGetSession,
    AuthGetToken,

    // Chart Methods
    ChartGetTopArtists,
    ChartGetTopTags,
    ChartGetTopTracks,

    // Geo Methods
    GeoGetTopArtists,
    GeoGetTopTracks,

    // Library Methods
    LibraryGetArtists,

    // Tag Methods
    TagGetInfo,
    TagGetSimilar,
    TagGetTopAlbums,
    TagGetTopArtists,
    TagGetTopTags,
    TagGetTopTracks,
    TagGetWeeklyChartList,

    // Track Methods
    TrackAddTags,
    TrackGetCorrection,
    TrackGetInfo,
    TrackGetSimilar,
    TrackGetTags,
    TrackGetTopTags,
    TrackLove,
    TrackRemoveTag,
    TrackScrobble,
    TrackSearch,
    TrackUnlove,
    TrackUpdateNowPlaying,

    // User Methods
    UserGetFriends,
    UserGetInfo,
    UserGetLovedTracks,
    UserGetPersonalTags,
    UserGetRecentTracks,
    UserGetTopAlbums,
    UserGetTopArtists,
    UserGetTopTags,
    UserGetTopTracks,
    UserGetWeeklyAlbumChart,
    UserGetWeeklyArtistChart,
    UserGetWeeklyChartList,
    UserGetWeeklyTrackChart,
}

impl fmt::Display for LastfmMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            // Album Methods
            LastfmMethod::AlbumAddTags => "album.addTags",
            LastfmMethod::AlbumGetInfo => "album.getInfo",
            LastfmMethod::AlbumGetTags => "album.getTags",
            LastfmMethod::AlbumGetTopTags => "album.getTopTags",
            LastfmMethod::AlbumRemoveTag => "album.removeTag",
            LastfmMethod::AlbumSearch => "album.search",

            // Artist Methods
            LastfmMethod::ArtistAddTags => "artist.addTags",
            LastfmMethod::ArtistGetCorrection => "artist.getCorrection",
            LastfmMethod::ArtistGetInfo => "artist.getInfo",
            LastfmMethod::ArtistGetSimilar => "artist.getSimilar",
            LastfmMethod::ArtistGetTags => "artist.getTags",
            LastfmMethod::ArtistGetTopAlbums => "artist.getTopAlbums",
            LastfmMethod::ArtistGetTopTags => "artist.getTopTags",
            LastfmMethod::ArtistGetTopTracks => "artist.getTopTracks",
            LastfmMethod::ArtistRemoveTag => "artist.removeTag",
            LastfmMethod::ArtistSearch => "artist.search",

            // Auth Methods
            LastfmMethod::AuthGetMobileSession => "auth.getMobileSession",
            LastfmMethod::AuthGetSession => "auth.getSession",
            LastfmMethod::AuthGetToken => "auth.getToken",

            // Chart Methods
            LastfmMethod::ChartGetTopArtists => "chart.getTopArtists",
            LastfmMethod::ChartGetTopTags => "chart.getTopTags",
            LastfmMethod::ChartGetTopTracks => "chart.getTopTracks",

            // Geo Methods
            LastfmMethod::GeoGetTopArtists => "geo.getTopArtists",
            LastfmMethod::GeoGetTopTracks => "geo.getTopTracks",

            // Library Methods
            LastfmMethod::LibraryGetArtists => "library.getArtists",

            // Tag Methods
            LastfmMethod::TagGetInfo => "tag.getInfo",
            LastfmMethod::TagGetSimilar => "tag.getSimilar",
            LastfmMethod::TagGetTopAlbums => "tag.getTopAlbums",
            LastfmMethod::TagGetTopArtists => "tag.getTopArtists",
            LastfmMethod::TagGetTopTags => "tag.getTopTags",
            LastfmMethod::TagGetTopTracks => "tag.getTopTracks",
            LastfmMethod::TagGetWeeklyChartList => "tag.getWeeklyChartList",

            // Track Methods
            LastfmMethod::TrackAddTags => "track.addTags",
            LastfmMethod::TrackGetCorrection => "track.getCorrection",
            LastfmMethod::TrackGetInfo => "track.getInfo",
            LastfmMethod::TrackGetSimilar => "track.getSimilar",
            LastfmMethod::TrackGetTags => "track.getTags",
            LastfmMethod::TrackGetTopTags => "track.getTopTags",
            LastfmMethod::TrackLove => "track.love",
            LastfmMethod::TrackRemoveTag => "track.removeTag",
            LastfmMethod::TrackScrobble => "track.scrobble",
            LastfmMethod::TrackSearch => "track.search",
            LastfmMethod::TrackUnlove => "track.unlove",
            LastfmMethod::TrackUpdateNowPlaying => "track.updateNowPlaying",

            // User Methods
            LastfmMethod::UserGetFriends => "user.getFriends",
            LastfmMethod::UserGetInfo => "user.getInfo",
            LastfmMethod::UserGetLovedTracks => "user.getLovedTracks",
            LastfmMethod::UserGetPersonalTags => "user.getPersonalTags",
            LastfmMethod::UserGetRecentTracks => "user.getRecentTracks",
            LastfmMethod::UserGetTopAlbums => "user.getTopAlbums",
            LastfmMethod::UserGetTopArtists => "user.getTopArtists",
            LastfmMethod::UserGetTopTags => "user.getTopTags",
            LastfmMethod::UserGetTopTracks => "user.getTopTracks",
            LastfmMethod::UserGetWeeklyAlbumChart => "user.getWeeklyAlbumChart",
            LastfmMethod::UserGetWeeklyArtistChart => "user.getWeeklyArtistChart",
            LastfmMethod::UserGetWeeklyChartList => "user.getWeeklyChartList",
            LastfmMethod::UserGetWeeklyTrackChart => "user.getWeeklyTrackChart",
        };
        write!(f, "{}", method_str)
    }
}

impl From<LastfmMethod> for String {
    fn from(method: LastfmMethod) -> Self {
        method.to_string()
    }
}

impl LastfmMethod {
    pub fn requires_auth(&self) -> bool {
        matches!(
            self,
            LastfmMethod::AlbumAddTags
                | LastfmMethod::AlbumRemoveTag
                | LastfmMethod::ArtistAddTags
                | LastfmMethod::ArtistRemoveTag
                | LastfmMethod::TrackAddTags
                | LastfmMethod::TrackLove
                | LastfmMethod::TrackUnlove
                | LastfmMethod::TrackRemoveTag
                | LastfmMethod::TrackUpdateNowPlaying
                | LastfmMethod::TrackScrobble
        )
    }
}

/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetStatus200Response {
    /// The current status of the game server.
    #[serde(rename = "status")]
    pub status: String,
    /// The current version of the API.
    #[serde(rename = "version")]
    pub version: String,
    /// The date when the game server was last reset.
    #[serde(rename = "resetDate")]
    pub reset_date: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "stats")]
    pub stats: models::GetStatus200ResponseStats,
    #[serde(rename = "leaderboards")]
    pub leaderboards: models::GetStatus200ResponseLeaderboards,
    #[serde(rename = "serverResets")]
    pub server_resets: models::GetStatus200ResponseServerResets,
    #[serde(rename = "announcements")]
    pub announcements: Vec<models::GetStatus200ResponseAnnouncementsInner>,
    #[serde(rename = "links")]
    pub links: Vec<models::GetStatus200ResponseLinksInner>,
}

impl GetStatus200Response {
    pub fn new(status: String, version: String, reset_date: String, description: String, stats: models::GetStatus200ResponseStats, leaderboards: models::GetStatus200ResponseLeaderboards, server_resets: models::GetStatus200ResponseServerResets, announcements: Vec<models::GetStatus200ResponseAnnouncementsInner>, links: Vec<models::GetStatus200ResponseLinksInner>) -> GetStatus200Response {
        GetStatus200Response {
            status,
            version,
            reset_date,
            description,
            stats,
            leaderboards,
            server_resets,
            announcements,
            links,
        }
    }
}


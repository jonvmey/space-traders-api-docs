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

/// ScannedShip : The ship that was scanned. Details include information about the ship that could be detected by the scanner.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScannedShip {
    /// The globally unique identifier of the ship.
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "registration")]
    pub registration: models::ShipRegistration,
    #[serde(rename = "nav")]
    pub nav: models::ShipNav,
    #[serde(rename = "frame", skip_serializing_if = "Option::is_none")]
    pub frame: Option<models::ScannedShipFrame>,
    #[serde(rename = "reactor", skip_serializing_if = "Option::is_none")]
    pub reactor: Option<models::ScannedShipReactor>,
    #[serde(rename = "engine")]
    pub engine: models::ScannedShipEngine,
    /// List of mounts installed in the ship.
    #[serde(rename = "mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::ScannedShipMountsInner>>,
}

impl ScannedShip {
    /// The ship that was scanned. Details include information about the ship that could be detected by the scanner.
    pub fn new(symbol: String, registration: models::ShipRegistration, nav: models::ShipNav, engine: models::ScannedShipEngine) -> ScannedShip {
        ScannedShip {
            symbol,
            registration,
            nav,
            frame: None,
            reactor: None,
            engine,
            mounts: None,
        }
    }
}


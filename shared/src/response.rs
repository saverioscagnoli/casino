use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomPrivacy {
    Public,
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomResponse {
    pub id: String,
    pub privacy: RoomPrivacy,
    pub relay_addr: String,
}

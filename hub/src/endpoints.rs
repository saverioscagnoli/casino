pub enum Endpoint {
    CreateRoom(String),
}

impl Endpoint {
    pub fn url(&self) -> String {
        match self {
            Endpoint::CreateRoom(addr) => format!("http://{}/room/create", addr),
        }
    }
}

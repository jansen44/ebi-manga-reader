pub struct Source {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
}

impl Source {
    pub fn new() -> Self {
        let base_url = String::from("https://onepieceex.net");

        Self {
            base_url,
            identifier: String::from("opex"),
            title: String::from("One Piece Ex"),
            description: String::from("One Piece Ex | De fã para fã"),
        }
    }
}

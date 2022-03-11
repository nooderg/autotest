#[derive(Serialize, Deserialize)]
pub struct Testing {
    pub file: String,
}

impl Testing {
    pub fn new(
        file: String,
    ) -> Self {
        Self {
            file: file,
        }
    }
}
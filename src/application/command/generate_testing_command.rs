#[derive(Serialize, Deserialize)]
pub struct GenerateTestingCommand {
    file : String,
}

impl GenerateTestingCommand {
    pub fn new(file: String) -> GenerateTestingCommand {
        GenerateTestingCommand {
            file : file,
        }
    }
    pub fn file(&self) -> &String {
        return &self.file;
    }
}
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ShowUserCommand {
    id: Uuid,
}

impl ShowUserCommand {
    pub fn new(id: Uuid) -> ShowUserCommand {
        ShowUserCommand {
            id: id,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
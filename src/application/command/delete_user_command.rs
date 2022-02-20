use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct DeleteUserCommand {
    id: Uuid,
}

impl DeleteUserCommand {
    pub fn new(id : Uuid) -> DeleteUserCommand {
        DeleteUserCommand {
            id : id,
        }
    }

    pub fn remove(id: Uuid) -> DeleteUserCommand {
        DeleteUserCommand {
            id : id,
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }
}
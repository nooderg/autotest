#[derive(Serialize, Deserialize)]
pub struct LoginUserCommand {
    email: String,
    password: String,
}

impl LoginUserCommand {
    pub fn new(email: String, password: String) -> LoginUserCommand {
        LoginUserCommand {
            email: email,
            password: password,
        }
    }
   
    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

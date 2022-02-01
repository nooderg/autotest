#[derive(Serialize, Deserialize)]
pub struct LoginUserCommand {
    email: String
}


impl LoginUserCommand {
    pub fn login( email: String) -> LoginUserCommand {
        LoginUserCommand {
            email: email,

        }
    }
   
}

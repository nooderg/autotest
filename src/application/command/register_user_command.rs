#[derive(Serialize, Deserialize)]
pub struct RegisterUserCommand {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl RegisterUserCommand {
    pub fn new(first_name: String, last_name: String, email: String, password :String) -> RegisterUserCommand {
        RegisterUserCommand {
            first_name: first_name,
            last_name: last_name,
            email: email,
            password : password,
        }
    }
    
    /**pub fn hash_password(password: &String) -> String {
        let mut hasher = Sha3::sha3_256(); //md1
        hasher.input_str(password);
        hasher.result_str()
    }**/

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

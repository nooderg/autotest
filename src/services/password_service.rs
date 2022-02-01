use pwhash::{sha1_crypt};

pub fn hash_password(password: String) ->   String  {
    sha1_crypt::hash(password).unwrap()
}
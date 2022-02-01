use pwhash::{sha1_crypt};
use rand::Rng;
pub fn hash_password(password: String) ->   String  {
    sha1_crypt::hash(password).unwrap()
}
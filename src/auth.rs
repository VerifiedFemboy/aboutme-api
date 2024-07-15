use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct AuthKey {
    pub value: String
}

impl AuthKey {
    pub fn generate_string() -> String {
        rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect()
    }
}
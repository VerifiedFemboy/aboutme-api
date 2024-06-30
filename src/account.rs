pub struct Account {
    name: String,
    about: String,
    create_date: String,
    auth_key: String
}

impl Account {
    pub async fn create(&self) -> std::io::Result<&str> {
        println!("{:#?}", self);
        Ok("okie ;3")
    }

    pub fn new(name: String, about: String, create_date: String, auth_key: String) -> Self {
        Self { name, about, create_date, auth_key }
    }
}
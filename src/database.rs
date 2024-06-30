use mongodb::{Client, Collection, error};
use mongodb::options::ClientOptions;
use crate::account::Account;

#[derive(Clone)]
pub struct Database {
    client: Client,
    account_collection: Collection<Account>
}

impl Database {
    pub async fn new(connection: &str, dbname: &str) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(connection).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(dbname);
        let collection = db.collection::<Account>("accounts");
        Ok(Self{client, account_collection: collection})
    }

    pub async fn insert_account(&self, account: Account) -> error::Result<()> {
        self.account_collection.insert_one(account).await?;
        Ok(())
    }
}
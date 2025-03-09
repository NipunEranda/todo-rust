use mongodb::{
    bson::{doc, Document}, Client
};

pub struct Mongo {
    pub client: Client,
    pub db: mongodb::Database,
}

impl Mongo {
    pub async fn new(database: &str) -> Mongo {
        let result = Client::with_uri_str("mongodb://localhost:27017").await;

        let client = match result {
            Ok(client) => client,
            Err(e) => panic!("Failed to connect to the database: {:?}", e),
        };

        let db = client.database(database);
        Mongo { client, db }
    }

    // pub fn get_client(&self) -> &Client {
    //     &self.client
    // }

    pub fn get_db(&self) -> &mongodb::Database {
        &self.db
    }

    pub async fn get_all_documents(db: &mongodb::Database, collection: &str) -> Vec<Document> {
        let mut documents: Vec<Document> = Vec::new();

        // let db: Mongo = Mongo::new(db).await;
        let collection: mongodb::Collection<mongodb::bson::Document> =
            db.collection(collection);

        let result: Result<Option<Document>, mongodb::error::Error> =
            collection.find_one(doc! {}).await;

        if let Ok(Some(doc)) = result {
            documents.push(doc);
        }

        documents
    }
}

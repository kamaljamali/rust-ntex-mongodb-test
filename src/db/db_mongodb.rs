use crate::models::students_model::{Fields, StudentModel};
use dotenv::dotenv;
use mongodb::Cursor;
use ntex::web;
use std::env;

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn connect_to_db() -> Result<mongodb::Client, mongodb::error::Result<()>> {
    // Replace the placeholder with your Atlas connection string
    dotenv().ok();
    let uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await;

    // Get a handle on the movies collection
    match client {
        Ok(res) => Ok(res),
        Err(err) => Err(Err(err)),
    }
}

pub async fn test_connection(connection: Client) {
    let database = connection.database("test_rust");
    let my_coll: Collection<Document> = database.collection("students");

    // Find a movie based on the title value
    let my_std = my_coll.find_one(doc! { "name": "Kamal" }, None).await;

    // Print the document
    println!("Found a movie:\n{:#?}", my_std);
}

pub async fn get_all_students2(
    connection: Client,
) -> Result<Cursor<StudentModel>, mongodb::error::Error> {
    let database = connection.database("test_rust");
    let my_coll: Collection<StudentModel> = database.collection("students");

    let filter = doc! {};
    let cursor: Result<Cursor<StudentModel>, mongodb::error::Error> =
        my_coll.find(filter, None).await;

    match cursor {
        Ok(res) => Ok(res),
        Err(err) => Err(err),
    }
}

pub async fn add_student(
    connection: Client,
    item: web::types::Json<StudentModel>,
) -> Result<bool, mongodb::error::Error> {
    let database = connection.database("test_rust");
    let my_coll: Collection<StudentModel> = database.collection("students");

    let result = my_coll.insert_one(item.into_inner(), None).await;
    match result {
        Ok(res) => Ok(true),
        Err(err) => Err(err),
    }
}

pub async fn update_student_test(
    connection: Client,
    item: web::types::Json<StudentModel>,
) -> Result<bool, mongodb::error::Error> {
    let database = connection.database("test_rust");
    let my_coll: Collection<StudentModel> = database.collection("students");

    let filter = doc! {"name":&item.name};
    let result = my_coll.replace_one(filter, item.into_inner(), None).await;
    match result {
        Ok(res) => Ok(true),
        Err(err) => Err(err),
    }
}

pub async fn update_student_fields(
    connection: Client,
    items: web::types::Json<Fields>,
    name: String,
) -> Result<bool, mongodb::error::Error> {
    let database = connection.database("test_rust");
    let my_coll: Collection<StudentModel> = database.collection("students");

    let filter = doc! {"name":name};
    let update = doc! { "$addToSet": doc! {"fields": doc! {"$each":items.fields.clone()}} };

    let result = my_coll.update_one(filter, update, None).await;
    match result {
        Ok(res) => Ok(true),
        Err(err) => Err(err),
    }
}

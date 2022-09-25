#[macro_use] extern crate rocket;

use std::path::Path;

use rocket::tokio::fs::File;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;

mod paste_id;
use paste_id::PasteId;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, retrieve, upload])
}

#[get("/")]
fn index() -> &'static str {
    "
    POST /

    Accepts raw data in the body, responds with URL containing content

    GET /<id>

    Recieves content from paste with `<id>`
    "
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[post("/", data= "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);

    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}
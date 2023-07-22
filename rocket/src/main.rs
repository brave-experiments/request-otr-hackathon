#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::response::Response;
use rocket::http::ContentType;
use rocket::fs::{NamedFile, relative};
use std::fs;


#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/http-meta-tag/index.html", routes![load_meta_tag])
        .attach(AdHoc::on_response("Add Request-OTR header", |_req, response| Box::pin(async move {
            response.set_header(Header::new("Request-OTR", "1"));
        })))
}

#[get("/")]
async fn load_meta_tag() -> NamedFile {
    let data = NamedFile::open("../../http-meta-tag/index.html").await.expect("Should have been able to read the file");
    return data;
}


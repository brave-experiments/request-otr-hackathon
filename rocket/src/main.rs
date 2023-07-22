#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::http::Header;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .attach(AdHoc::on_response("Add Request-OTR header", |_req, response| Box::pin(async move {
            response.set_header(Header::new("Request-OTR", "1"));
        })))
}

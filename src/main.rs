
use rocket::*;

//[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, WebSubmit!"
}

#[get("/<paper>")]
fn question(paper: String) -> String{
    format!("Question about {}\nWhat was one thing that confused you?", paper)
}

mod api_key; //like an import for the api_key.rs file

#[get("/protected")]
fn protected(key: api_key::ApiKey) -> String{
    format!("Welcome to WebSubmit. You presented key {}", key.0)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        hello,
        question,
        protected
        ])
}

/////
/* 
#[derive(FromForm)]
struct PaperResponse {
    input: String,
}

#[post("/submit", data = "<form>")]
fn submit(form: Form<PaperResponse>) -> String {
    form.into_inner().field
}
*/
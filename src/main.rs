use rocket::{Request, catchers, catch, launch, post, get, routes, response::{status::Created}, State};
use rocket::{serde::json::Json, uri}; 
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
//use serde_json::*; //TODO this is wrong -> look at imports on RustyRockets
//[macro_use] extern crate rocket;

#[get("/")] //"Macros"
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

#[catch(default)]
fn default_catcher() -> &'static str {
    "I couldn't find that :( Try something else?"
}

#[catch(401)]
fn incorrect_key_catcher() -> &'static str {
    "Incorrect key, try again!"
}

#[derive(Serialize, Debug, Clone)]
struct QuestionResponse{
    id: ID,
    answer: String,
}

#[derive(Deserialize, Debug)]
struct NewResponse{
    answer: String,
}

type ID = usize;
struct ResponseCount(AtomicUsize);
type ResponseMap = RwLock<HashMap<ID, QuestionResponse>>;

#[post("/responses", format = "json", data = "<response>")]
fn add_response(
    response: Json<NewResponse>,
    resp_state: &State<ResponseMap>,
    resp_count: &State<ResponseCount>) -> Created<Json<QuestionResponse>> {
        //generate new response and ID
        let new_id = resp_count.0.fetch_add(1, Ordering::Relaxed);
        let new_response = QuestionResponse {
            id: new_id,
            answer: response.0.answer,
        };

        //update HashMap with all responses
        let mut responses = resp_state.write().unwrap();
        responses.insert(new_id, new_response.clone());

        let location = uri!("/", get_response(new_id)); //uri macro to write the resonse to a new url
        Created::new(location.to_string()).body(Json(new_response)) // TODO: Json use is wrong
}

#[get("/responses/<id>")]
fn get_response(id: ID, resp_state: &State<ResponseMap>) -> Option<Json<QuestionResponse>>{
    let responses = resp_state.read().unwrap();
    responses.get(&id).map(|resp| Json(resp.clone()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![
        hello,
        question,
        protected, 
        add_response,
        get_response,
        ])
    .manage(RwLock::new(HashMap::<ID, QuestionResponse>::new()))
     //generate primary keys for responses from 1
    .manage(ResponseCount(AtomicUsize::new(1)))
    //
    .register("/", catchers![default_catcher, incorrect_key_catcher])
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
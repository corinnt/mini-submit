use rocket::*; //this feels like bad form
use serde::{Serialize, Deserialize};
use serde_json::*; //TODO this is wrong

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

#[derive(Serialize, Debug, Clone)] // why Clone?
struct QuestionResponse{
    id: ID,
    answer: String,
}

#[derive(Deserialize, Debug)]
struct NewResponse{
    answer: String,
}

struct ResponseCount(AtomicUsize);
type ResponseMap = HashMap<ID, QuestionResponse>;

#[post("/responses", format = "json", data = "<response>")]
fn add_response(
    response: Json<NewResponse>,
    resp_state: $State<ResponseMap>,
    resp_count: &State<ResponseCount>,
    ) -> Created<Json<QuestionResponse>> {
        //generate new response and ID
        let new_id = resp_count.0.fetch_add(1, Ordering::Relaxed);
        let new_response = QuestionResponse {
            id: new_id,
            answer: response.0.answer,
        };

        //update HashMap with all responses
        let mut responses = resp_state.write().unwrap();
        responses.insert(new_id, new_response.clone());

        let location = uri!("/", get_response(new_id));
        Created::new(location.to_string()).body(Json(new_response)) // TODO: Json use is wrong
    }

#[get("/a_response")]
fn get_response(id: ID, resp_state: &State<ResponseMap>) -> Option<Json<QuestionResponse>>{
    let responses = resp_state.read().unwrap();
    responses.get(&id).map(|resp| Json(resp.clone()))
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        hello,
        question,
        protected, 
        add_response,
        get_response,
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
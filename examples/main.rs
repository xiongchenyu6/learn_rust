use rocket::serde::{Deserialize, Serialize, json::Json};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool
}

#[post("/todo", data = "<task>")]
fn new(task: Json<Task<'_>>) -> Json<Task<'_>> { 
    println!("Task: {}", task.description);
    task
 }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new])
}
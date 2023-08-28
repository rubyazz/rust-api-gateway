#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{http::Method, response::Debug, State};

struct Service {
    name: String,
    url: String,
}

#[get("/")]
fn index() -> &'static str {
    "API Gateway"
}

#[get("/services")]
fn list_services(services: State<Vec<Service>>) -> Debug<Vec<Service>> {
    Debug(services.inner().clone())
}

#[post("/services", data = "<service>")]
fn add_service(service: rocket::request::Form<Service>, services: State<Vec<Service>>) -> String {
    let mut services = services.inner().clone();
    services.push(service.into_inner());
    "Service added".to_string()
}

#[delete("/services/<name>")]
fn remove_service(name: String, services: State<Vec<Service>>) -> String {
    let mut services = services.inner().clone();
    services.retain(|s| s.name != name);
    "Service removed".to_string()
}

#[catch(404)]
fn not_found() -> &'static str {
    "404 Not Found"
}

fn main() {
    let services = vec![
        Service {
            name: "Service A".to_string(),
            url: "http://localhost:8000".to_string(),
        },
        Service {
            name: "Service B".to_string(),
            url: "http://localhost:9000".to_string(),
        },
    ];
    rocket::ignite()
        .manage(services)
        .mount(
            "/",
            routes![index, list_services, add_service, remove_service],
        )
        .register(catchers![not_found])
        .launch();
}

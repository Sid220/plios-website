#[macro_use]
extern crate rocket;

mod secrets;

use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use std::fs;
use reqwest::Client;

use rocket::{fs::FileServer, get, launch, routes};
use std::collections::HashMap;

// #[get("/page")]
// fn page() -> Template {
//     // let mut context = HashMap::new();
//     // context.insert("name", "string");
//     let data: String;
//     data = fs::read_to_string("/tmp/name").expect("Unable to read file");
//     Template::render("index", context! { name: data})
// }

// #[post("/name", data = "<form>")]
// fn name(form: Form<UserName>) -> String {
//     fs::write("/tmp/name", &form.username).expect("Unable to write file");
//     form.username.clone()
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//     .attach(Template::fairing())
//     .mount("/", routes![name, page])
//     .mount("/", FileServer::from(relative!("static")))
// }
#[get("/")]
fn index() -> Template {
    Template::render("index", context! { title: "Home" })
}

#[get("/submission")]
fn submission() -> Template {
    Template::render("submission", context! { title: "Submission" })
}

#[derive(Debug, FromForm)]
struct SubmissionForm {
    pub projname: String,
    pub pername: String,
    pub email: String,
    pub relation: String,
    pub license: String,
    pub url: String,
    pub vc: String,
    pub pop: i8,
    pub platforms: String,
    pub why: String,
}

#[post("/submission", data = "<form>")]
async fn submission_submit(form: Form<SubmissionForm>) -> String {
    let email_res = secrets::routes::submit_secret("Submission Email", form.email.clone());
    let pername_res = secrets::routes::submit_secret("Submission Person Name", form.pername.clone());
    let client = Client::new();
    let opening = "Another Submission to the Plios Projects 🎉\n**Project Name**: ";
    let personName = "\n**Person Name**: ";
    let email = "\n**Email**: ";
    let relation = "\n**Relation to Project**: ";
    let license = "\n**License**: ";
    let url = "\n**Url**: ";
    let vc = "\n**Version Control**: ";
    let downloads_per_week = "\n**Approx. Users/week**: ";
    let platforms = "\n**Platforms**: ";
    let why = "\n**Why?**: ";

    let mut map = HashMap::new();
    map.insert("icon_emoji", ":information_source:");
    let text = [opening, &form.projname, personName, &pername_res, email, &email_res, relation, &form.relation, license, &form.license, vc, &form.vc, platforms, &form.platforms, why, &form.why, downloads_per_week, &form.pop.to_string()].join("");
    map.insert("text", &text);
    let res = client.post("https://mm.plios.tech/hooks/ni6yhgs45t8u3dsn4f73iekqwe")
        .json(&map).send().await;
    "Submission Received!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, submission, submission_submit])
        .mount("/secrets", secrets::routes::get_routes())
        .mount("/public", FileServer::from("public"))
}
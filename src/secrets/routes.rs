use rocket::Route;
use rocket::response::content;
use mysql::*;
use mysql::prelude::*;
use rocket::http::RawStr;

#[get("/")]
pub fn index() -> content::RawHtml<&'static str> {
    content::RawHtml("<h1>Secrets Home</h1>")
}

#[derive(Debug, PartialEq, Eq)]
struct Secret {
    pub(crate) id: i32,
    value: Option<String>,
    name: Option<String>,
}

pub fn submit_secret(name: &str, value: String) -> String {
    let url = "mysql://root:rootpass@localhost:3306/plios";
    let pool_test = Pool::new(url);
    if pool_test.is_err() {
        return "Internal Error".to_string();
    }
    let pool = pool_test.unwrap();

    let mut conn = pool.get_conn().unwrap();

    let query = format!("INSERT INTO secrets (name, value) VALUES ('{}', '{}')", name, value);
    let ins = conn.query_drop(query);
    if ins.is_err() {
        return "Internal Error".to_string();
    }
    let res = conn.query_map("SELECT id, value, name FROM secrets WHERE id = LAST_INSERT_ID()", |(id, value, name)| {
        Secret { id, value, name }
    });
    if res.is_err() {
        return "Internal Error".to_string();
    }
    let binding = res.unwrap();
    let secret_id = binding[0].id;
    format!("http://127.0.0.1:8000/secrets/secret/{}", secret_id)
}

#[get("/secret/<secret>")]
pub fn get_secret(secret: String) -> content::RawHtml<String> {

    // format!("Hello, {}!", secret)
    let contents = format!("<h1>Secret: {}</h1><form action='../secret_view/{}', method='GET'><input placeholder='Password' name='pass' required /><input type='submit' value='Query'/></form>", secret, secret);
    content::RawHtml(contents)
}

#[get("/secret_view/<secret>?<pass>")]
pub fn get_secret_view(secret: String, pass: String) -> content::RawHtml<String> {
    let url = "mysql://root:rootpass@localhost:3306/plios";
    let pool_test = Pool::new(url);
    if (pool_test.is_err()) {
        return content::RawHtml("<h1>Internal Error!</h1>".parse().unwrap());
    }
    let pool = pool_test.unwrap();

    let mut conn = pool.get_conn().unwrap();

    let master_pass_res = conn
        .query_map(
            "SELECT id, value, name FROM secrets WHERE id = 1",
            |(id, value, name)| {
                Secret { id, value, name }
            },
        );

    if (master_pass_res.is_err()) {
        return content::RawHtml("<h1>Internal Error!!</h1>".parse().unwrap());
    }
    let binding = master_pass_res.unwrap();
    let master_pass = binding[0].value.as_ref().unwrap();

    if (&*pass != master_pass) {
        return content::RawHtml("<h1>Incorect Password</h1>".parse().unwrap());
    } else {
        let secret_id = (secret.parse::<i32>());
        if (secret_id.is_err()) {
            content::RawHtml("<h1>Not Found!!</h1>".parse().unwrap())
        } else {
            let retrieved_secrets = conn.query_map(format!("SELECT id, value, name FROM secrets WHERE id = {}", secret_id.unwrap()), |(id, value, name)| {
                Secret { id, value, name }
            });
            if (retrieved_secrets.is_err()) {
                return content::RawHtml("<h1>Not Found!!</h1>".parse().unwrap());
            }
            let retrieved_secret = &(retrieved_secrets.unwrap())[0];

            return content::RawHtml(format!("<h1>{}</h1><textarea readonly>{}</textarea>", retrieved_secret.name.clone().unwrap(), retrieved_secret.value.clone().unwrap()));
        }
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![index, get_secret, get_secret_view]
}
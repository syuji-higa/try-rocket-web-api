use rocket_contrib::json::Json;

use crate::models::User;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
pub fn users() -> Json<Vec<User>> {
  Json(vec![
    User {
      id: 1,
      name: "Taro Tanaka".into(),
      age: 34,
      alive: true,
    },
    User {
      id: 2,
      name: "Hanako Yamada".into(),
      age: 22,
      alive: false,
    },
  ])
}

#[post("/users", data = "<user>")]
pub fn new_uesr(user: Json<User>) -> String {
    format!("Accepted post request! {:?}", user.0)
}

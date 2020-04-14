#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod models;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, users, new_uesr])
        .launch();
}

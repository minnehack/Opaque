/* Copyright 2020, Hemant Gouni <hemant@hemantgouni.com>
 * This file is part of Opaque.

 * Opaque is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * Opaque is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.

 * You should have received a copy of the GNU Affero General Public License
 * along with Opaque.  If not, see <https://www.gnu.org/licenses/>. 
 */

#![feature(proc_macro_hygiene, decl_macro)]

pub mod models;
pub mod schema;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::response::status::BadRequest;

use diesel::prelude::*;

use crate::models::InsertableRegistrant;
use crate::schema::registrants::dsl::*;
// use rocket_contrib::databases::diesel;

#[database("mh_reg")]
struct RegDbConn(diesel::MysqlConnection);

#[post("/api/registration", data = "<registrant>")]
fn register(conn: RegDbConn, registrant: Form<InsertableRegistrant>) -> Result<Redirect, BadRequest<String>> {
    let reg_form = diesel::insert_into(registrants)
        .values(&*registrant)
        .execute(&*conn);

    match reg_form {
        Ok(_) => Ok(Redirect::found("/register-success")),
        Err(_) => Err(BadRequest(Some("error".to_string())))
    }
}

fn main() {
    rocket::ignite()
        .attach(RegDbConn::fairing())
        .mount("/", routes![register])
        .launch();
}

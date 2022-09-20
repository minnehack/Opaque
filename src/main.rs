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
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

use std::fs;
use std::io::Error;
use std::io::ErrorKind;

use rand::Rng;

use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::data::{Capped};
use rocket::response::Redirect;
use rocket::response::status::BadRequest;

use crate::diesel::RunQueryDsl;
use crate::models::Registrant;
use crate::models::InsertableRegistrant;
use crate::schema::registrants::dsl::registrants;

#[database("mh_reg")]
struct RegDbConn(diesel::MysqlConnection);

#[post("/api/register", data = "<registrant>")]
async fn register(conn: RegDbConn, mut registrant: Form<Registrant<'_>>) ->
    Result<Redirect, BadRequest<String>> {

    let user_identifier: i64 = rand::thread_rng().gen::<i64>();

    match upload_file(registrant.resume.as_mut(), user_identifier).await {
        Err(_) => return Err(BadRequest(Some("Error while uploading resume!".to_string()))),
        Ok(_) => ()
    };

    let insertable_registrant = registrant_to_insertable(registrant, user_identifier).await;

    let db_response = conn.run(move |c|
        diesel::insert_into(registrants).values(&insertable_registrant).execute(c)).await;

    match db_response {
        Err(_) => Err(BadRequest(Some("error".to_string()))),
        Ok(_) => Ok(Redirect::found("/register-success"))
    }
}

async fn registrant_to_insertable(registrant: Form<Registrant<'_>>, user_identifier: i64) ->
    InsertableRegistrant {

    return InsertableRegistrant {
        email: registrant.email.clone(),
        first_name: registrant.first_name.clone(),
        last_name: registrant.last_name.clone(),
        gender: registrant.gender.clone(),
        phone: registrant.phone,
        country: registrant.country.clone(),
        school: registrant.school.clone(),
        level_of_study: registrant.level_of_study.clone(),
        minor: registrant.age < 18,
        age: registrant.age,
        tshirt: registrant.tshirt.clone(),
        driving: registrant.driving,
        reimbursement: registrant.reimbursement,
        reimbursement_amount: registrant.reimbursement_amount,
        reimbursement_desc: registrant.reimbursement_desc.clone(),
        reimbursement_strict: registrant.reimbursement_strict,
        accommodations: registrant.accommodations.clone(),
        dietary_restrictions: registrant.dietary_restrictions.clone(),
        mlh_mailing_list: registrant.mlh_mailing_list,
        user_identifier: user_identifier
    }
}

async fn upload_file(file: Option<&mut Capped<TempFile<'_>>>, identifier: i64) ->
    std::io::Result<()> {

    match file {

        None => Err(Error::new(ErrorKind::InvalidInput, "Unable to upload nonexistent file!")),

        Some(stream) => {

            if stream.is_complete() {

                stream.persist_to("storage/".to_owned() + &identifier.to_string()).await?;
                Ok(())

            } else {

                Err(Error::new(ErrorKind::InvalidData, "File exceeded maximum size!"))
            }
        }
    }
}

#[launch]
fn rocket() -> _ {

    match fs::create_dir_all("storage") {
        Ok(()) => (),
        Err(_) => panic!("Could not create storage directory!"),
    };

    rocket::build()
        .attach(RegDbConn::fairing())
        .mount("/", routes![register])
}

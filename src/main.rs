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

pub mod models;
pub mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

use std::fs;
use std::io::Error;
use std::io::ErrorKind;

use rocket::data::Capped;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::status::BadRequest;
use rocket::response::Redirect;

use crate::models::InsertableRegistrant;
use crate::models::Registrant;
use crate::schema::registrants::dsl::*;

use crate::diesel::RunQueryDsl;
use diesel::query_dsl::QueryDsl;

#[database("mh_reg")]
struct RegDbConn(diesel::MysqlConnection);

#[post("/api/register", data = "<registrant>")]
async fn register(
    conn: RegDbConn,
    mut registrant: Form<Registrant<'_>>,
) -> Result<Redirect, BadRequest<&str>> {
    let insertable_registrant = InsertableRegistrant::from(&registrant);

    upload_file(
        &mut registrant.resume,
        *conn
            .run(|c| {
                registrants
                    .select(db_identifier)
                    .order(db_identifier)
                    .load(c)
                    .map_err(|_| BadRequest(Some("Database error")))
            })
            .await?
            .last()
            .unwrap(),
    )
    .await
    .map_err(|_| BadRequest(Some("Error uploading file")))?;

    // we could have pattern matched here, but it might look a bit uglier
    conn.run(move |c| {
        diesel::insert_into(registrants)
            .values(&insertable_registrant)
            .execute(c)
    })
    .await
    .map(|_| Redirect::found("/register-success"))
    .map_err(|_| BadRequest(Some("Database error")))
}

impl<'a> From<&Form<Registrant<'a>>> for InsertableRegistrant {
    fn from(registrant: &Form<Registrant<'a>>) -> Self {
        return InsertableRegistrant {
            email: registrant.email.to_string(),
            first_name: registrant.first_name.to_string(),
            last_name: registrant.last_name.to_string(),
            gender: registrant.gender.to_string(),
            phone: registrant.phone,
            country: registrant.country.to_string(),
            school: registrant.school.to_string(),
            level_of_study: registrant.level_of_study.to_string(),
            minor: registrant.age < 18,
            age: registrant.age,
            tshirt: registrant.tshirt.to_string(),
            driving: registrant.driving,
            reimbursement: registrant.reimbursement,
            reimbursement_amount: registrant.reimbursement_amount,
            reimbursement_desc: registrant.reimbursement_desc.map(|str| str.to_string()),
            reimbursement_strict: registrant.reimbursement_strict,
            accommodations: registrant.accommodations.map(|str| str.to_string()),
            dietary_restrictions: registrant.dietary_restrictions.map(|str| str.to_string()),
        };
    }
}

async fn upload_file(stream: &mut Capped<TempFile<'_>>, identifier: i64) -> std::io::Result<()> {
    if stream.is_complete() && stream.len() > 0 {
        stream
            .persist_to(String::from("storage/") + &identifier.to_string())
            .await?;
        Ok(())
    } else if stream.is_complete() {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::InvalidData,
            "File exceeded maximum size!",
        ))
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

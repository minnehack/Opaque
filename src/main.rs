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

#[macro_use]
extern crate rocket;

use std::fs;
use std::io::Error;
use std::io::ErrorKind;

use rocket::data::Capped;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::status::BadRequest;
use rocket::response::Redirect;

use crate::models::Registrant;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("mh_reg")]
struct Db(sqlx::MySqlPool);

#[post("/api/register", data = "<registrant>")]
async fn register(
    mut db: Connection<Db>,
    mut registrant: Form<Registrant<'_>>,
) -> Result<Redirect, BadRequest<&str>> {
    let identifier: u64 = sqlx::query!(
        r#"INSERT INTO registrants
            (email,
             first_name,
             last_name,
             gender,
             phone,
             country,
             school,
             level_of_study,
             minor,
             age,
             tshirt,
             driving,
             reimbursement,
             reimbursement_amount,
             reimbursement_desc,
             reimbursement_strict,
             accommodations,
             dietary_restrictions)
        VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id"#,
        registrant.email,
        registrant.first_name,
        registrant.last_name,
        registrant.gender,
        registrant.phone,
        registrant.country,
        registrant.school,
        registrant.level_of_study,
        registrant.age < 18,
        registrant.age,
        registrant.tshirt,
        registrant.driving,
        registrant.reimbursement,
        registrant.reimbursement_amount,
        registrant.reimbursement_desc,
        registrant.reimbursement_strict,
        registrant.accommodations,
        registrant.dietary_restrictions,
    )
    .fetch_one(&mut *db)
    .await
    .map_err(|_| BadRequest(Some("Database Error 0")))?
    .try_get(0)
    .map_err(|_| BadRequest(Some("Database error 1")))?;

    upload_file(&mut registrant.resume, identifier)
        .await
        .map(|_| Redirect::found("/register-success"))
        .map_err(|_| BadRequest(Some("Error uploading file")))
}

async fn upload_file(stream: &mut Capped<TempFile<'_>>, identifier: u64) -> std::io::Result<()> {
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
        .attach(Db::init())
        .mount("/", routes![register])
}

/* Copyright 2020, Hemant Gouni
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

use rocket::data::Capped;
use rocket::fs::TempFile;
use rocket::form::Form;

use rocket_db_pools::sqlx::FromRow;

// will this let us automatically encode DbRegistrant as something to be sent to the db?
// use rocket_db_pools::sqlx::Type;

#[derive(FromRow)]
pub struct DbRegistrant {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub phone: u64,
    pub country: String,
    pub school: String,
    pub level_of_study: String,
    pub minor: bool,
    pub age: u64,
    pub tshirt: String,
    pub driving: bool,
    pub reimbursement: bool,
    pub reimbursement_amount: Option<i64>,
    pub reimbursement_desc: Option<String>,
    pub reimbursement_strict: Option<bool>,
    pub accommodations: Option<String>,
    pub dietary_restrictions: Option<String>,
}

// Form<T> dereferences into &T or &mut T, so we don't need a separate impl for that
impl From<&Registrant<'_>> for DbRegistrant {
    fn from(registrant: &Registrant<'_>) -> Self {
        DbRegistrant {
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
        }
    }
}

#[derive(FromForm)]
pub struct Registrant<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub gender: &'a str,
    pub phone: u64,
    pub country: &'a str,
    pub school: &'a str,
    pub level_of_study: &'a str,
    pub age: u64,
    pub tshirt: &'a str,
    pub driving: bool,
    pub reimbursement: bool,
    #[field(name = "reimbursement-amount")]
    pub reimbursement_amount: Option<i64>,
    #[field(name = "reimbursement-desc")]
    pub reimbursement_desc: Option<&'a str>,
    #[field(name = "reimbursement-strict")]
    pub reimbursement_strict: Option<bool>,
    pub accommodations: Option<&'a str>,
    pub dietary_restrictions: Option<&'a str>,
    pub resume: Capped<TempFile<'a>>,
    #[field(validate = eq(true))]
    pub student: bool,
    #[field(validate = eq(true))]
    pub coc: bool,
    #[field(validate = eq(true))]
    pub covidack: bool,
}

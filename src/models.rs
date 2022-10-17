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

use crate::schema::registrants;
use rocket::data::Capped;
use rocket::fs::TempFile;

#[derive(Insertable)]
#[table_name = "registrants"]
pub struct InsertableRegistrant {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub phone: i64,
    pub country: String,
    pub school: String,
    pub level_of_study: String,
    pub minor: bool,
    pub age: i64,
    pub tshirt: String,
    pub driving: bool,
    pub reimbursement: bool,
    pub reimbursement_amount: Option<i64>,
    pub reimbursement_desc: Option<String>,
    pub reimbursement_strict: Option<bool>,
    pub accommodations: Option<String>,
    pub dietary_restrictions: Option<String>,
}

#[derive(Queryable, QueryableByName)]
#[table_name = "registrants"]
pub struct QueryableRegistrant {
    pub db_identifier: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub phone: i64,
    pub country: String,
    pub school: String,
    pub level_of_study: String,
    pub minor: bool,
    pub age: i64,
    pub tshirt: String,
    pub driving: bool,
    pub reimbursement: bool,
    pub reimbursement_amount: Option<i64>,
    pub reimbursement_desc: Option<String>,
    pub reimbursement_strict: Option<bool>,
    pub accommodations: Option<String>,
    pub dietary_restrictions: Option<String>,
}

#[derive(FromForm)]
pub struct Registrant<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub gender: &'a str,
    pub phone: i64,
    pub country: &'a str,
    pub school: &'a str,
    pub level_of_study: &'a str,
    pub age: i64,
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

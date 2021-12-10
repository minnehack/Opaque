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
use rocket::fs::TempFile;
use rocket::data::Capped;

#[derive(Insertable)]
#[table_name = "registrants2021plusone"]
pub struct InsertableRegistrant {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub phone: i64,
    pub school: String,
    pub tshirt: String,
    pub driving: bool,
    pub reimbursement: bool,
    pub reimbursement_amount: Option<i64>,
    pub reimbursement_desc: Option<String>,
    pub reimbursement_strict: Option<bool>,
    pub minor: bool,
    pub accommodations: Option<String>,
    pub dietary_restrictions: Option<String>,
    pub student: bool,
    pub coc: bool,
    pub mlhpriv: bool,
    pub user_identifier: i64
}

#[derive(FromForm)]
pub struct Registrant<'a> {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub phone: i64,
    pub school: String,
    pub tshirt: String,
    pub driving: bool,
    pub reimbursement: bool,
    #[field(name = "reimbursement-amount")]
    pub reimbursement_amount: Option<i64>,
    #[field(name = "reimbursement-desc")]
    pub reimbursement_desc: Option<String>,
    #[field(name = "reimbursement-strict")]
    pub reimbursement_strict: Option<bool>,
    pub minor: bool,
    pub accommodations: Option<String>,
    pub dietary_restrictions: Option<String>,
    #[field(validate = eq(true))]
    pub student: bool,
    #[field(validate = eq(true))]
    pub coc: bool,
    #[field(validate = eq(true))]
    pub mlhpriv: bool,
    pub resume: Option<Capped<TempFile<'a>>>
}

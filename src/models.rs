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
#[table_name = "registrants"]
pub struct InsertableRegistrant {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub phone: i64,
    pub school: String,
    pub accommodations: String,
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
    pub accommodations: String,
    pub student: bool,
    pub coc: bool,
    pub mlhpriv: bool,
    pub resume: Option<Capped<TempFile<'a>>>
}

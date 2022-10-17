// @generated automatically by Diesel CLI.

diesel::table! {
    registrants (db_identifier) {
        db_identifier -> Bigint,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
        phone -> Bigint,
        country -> Varchar,
        school -> Varchar,
        level_of_study -> Varchar,
        minor -> Bool,
        age -> Bigint,
        tshirt -> Varchar,
        driving -> Bool,
        reimbursement -> Bool,
        reimbursement_amount -> Nullable<Bigint>,
        reimbursement_desc -> Nullable<Text>,
        reimbursement_strict -> Nullable<Bool>,
        accommodations -> Nullable<Text>,
        dietary_restrictions -> Nullable<Text>,
    }
}

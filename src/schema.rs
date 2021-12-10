table! {
    registrants (db_identifier) {
        db_identifier -> Unsigned<Bigint>,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
        phone -> Bigint,
        school -> Varchar,
        tshirt -> Varchar,
        driving -> Bool,
        reimbursement -> Bool,
        reimbursement_amount -> Nullable<Bigint>,
        reimbursement_desc -> Nullable<Text>,
        reimbursement_strict -> Nullable<Bool>,
        minor -> Bool,
        accommodations -> Nullable<Text>,
        dietary_restrictions -> Nullable<Text>,
        student -> Bool,
        coc -> Bool,
        mlhpriv -> Bool,
        user_identifier -> Bigint,
    }
}

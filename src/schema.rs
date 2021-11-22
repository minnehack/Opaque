table! {
    registrants (db_identifier) {
        db_identifier -> Unsigned<Bigint>,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
        phone -> Bigint,
        school -> Varchar,
        accommodations -> Varchar,
        student -> Bool,
        coc -> Bool,
        mlhpriv -> Bool,
        user_identifier -> Bigint,
    }
}

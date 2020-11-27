table! {
    registrants (id) {
        id -> Unsigned<Bigint>,
        email -> Varchar,
        firstName -> Varchar,
        lastName -> Varchar,
        gender -> Varchar,
        phone -> Bigint,
        school -> Varchar,
        accommodations -> Varchar,
        student -> Bool,
        coc -> Bool,
        mlhpriv -> Bool,
        resume -> Nullable<Varchar>,
    }
}

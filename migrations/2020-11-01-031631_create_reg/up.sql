CREATE TABLE registrants (

    -- checkboxes are always NOT NULL; we always have a value for em!

    -- others are situational; obviously, some people only need some fields

    -- Fields that may be NULL are explicitly marked here

    db_identifier SERIAL PRIMARY KEY,
    email VARCHAR (100) NOT NULL,
    first_name VARCHAR (100) NOT NULL,
    last_name VARCHAR (100) NOT NULL,
    gender VARCHAR (100) NOT NULL,
    phone BIGINT NOT NULL,
    school VARCHAR (100) NOT NULL,
    tshirt VARCHAR (100) NOT NULL,
    driving BOOLEAN NOT NULL,
    reimbursement BOOLEAN NOT NULL,
    -- the reimbursement fields may be NULL
    reimbursement_amount BIGINT,
    reimbursement_desc TEXT,
    reimbursement_strict BOOLEAN,
    minor BOOLEAN NOT NULL,
    -- accomodations may be NULL
    accommodations TEXT,
    -- dietary restrictions may be NULL
    dietary_restrictions TEXT,
    student BOOLEAN NOT NULL,
    coc BOOLEAN NOT NULL,
    mlhpriv BOOLEAN NOT NULL,
    user_identifier BIGINT NOT NULL
)

-- Your SQL goes here
CREATE TABLE registrants (

    -- checkboxes are always NOT NULL; we always have a value for em!

    -- others are situational; obviously, some people only need some fields

    -- Fields that may be NULL are explicitly marked here

    db_identifier BIGINT NOT NULL AUTO_INCREMENT UNIQUE PRIMARY KEY,
    email VARCHAR (100) NOT NULL,
    first_name VARCHAR (100) NOT NULL,
    last_name VARCHAR (100) NOT NULL,
    gender VARCHAR (100) NOT NULL,
    phone BIGINT NOT NULL,
    country VARCHAR (100) NOT NULL,
    school VARCHAR (100) NOT NULL,
    level_of_study VARCHAR (100) NOT NULL,
    minor BOOLEAN NOT NULL,
    age BIGINT NOT NULL,
    tshirt VARCHAR (100) NOT NULL,
    driving BOOLEAN NOT NULL,
    reimbursement BOOLEAN NOT NULL,
    -- the reimbursement fields may be NULL
    reimbursement_amount BIGINT,
    reimbursement_desc TEXT,
    reimbursement_strict BOOLEAN,
    -- accomodations may be NULL
    accommodations TEXT,
    -- dietary restrictions may be NULL
    dietary_restrictions TEXT
);

INSERT INTO registrants
    (email, first_name, last_name, gender, phone, country,
        school, level_of_study, minor, age, tshirt, driving, reimbursement,
        reimbursement_amount, reimbursement_desc, reimbursement_strict,
        accommodations, dietary_restrictions)
    VALUES
    ('test@example.com', 'John', 'Doe', 'Male', '1234567891', 'USA',
        'University of Minnesota', 'Undergraduate', FALSE, 21, 'S', FALSE,
        FALSE, null, null, null, null, null);

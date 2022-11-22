use mh_reg;

-- Your SQL goes here
CREATE TABLE registrants (

    -- checkboxes are always NOT NULL; we always have a value for em!

    -- others are situational; obviously, some people only need some fields

    -- Fields that may be NULL are explicitly marked here

    id SERIAL PRIMARY KEY,
    email VARCHAR (100) NOT NULL,
    first_name VARCHAR (100) NOT NULL,
    last_name VARCHAR (100) NOT NULL,
    gender VARCHAR (100) NOT NULL,
    phone BIGINT UNSIGNED NOT NULL,
    country VARCHAR (100) NOT NULL,
    school VARCHAR (100) NOT NULL,
    level_of_study VARCHAR (100) NOT NULL,
    minor BOOLEAN NOT NULL,
    age BIGINT UNSIGNED NOT NULL,
    tshirt VARCHAR (100) NOT NULL,
    driving BOOLEAN NOT NULL,
    discord_tag VARCHAR (100),
    reimbursement BOOLEAN NOT NULL,
    -- the reimbursement fields may be NULL
    reimbursement_amount BIGINT UNSIGNED,
    reimbursement_desc TEXT,
    reimbursement_strict BOOLEAN,
    -- accomodations may be NULL
    accommodations TEXT,
    -- dietary restrictions may be NULL
    dietary_restrictions TEXT
);

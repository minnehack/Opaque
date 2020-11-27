CREATE TABLE registrants (
    id SERIAL PRIMARY KEY,
    email VARCHAR (100) NOT NULL,
    firstName VARCHAR (100) NOT NULL,
    lastName VARCHAR (100) NOT NULL,
    gender VARCHAR (100) NOT NULL,
    phone BIGINT NOT NULL,
    school VARCHAR (100) NOT NULL,
    accommodations VARCHAR (100) NOT NULL,
    student BOOLEAN NOT NULL,
    coc BOOLEAN NOT NULL,
    mlhpriv BOOLEAN NOT NULL,
    resume VARCHAR (100)
)

ALTER TABLE registrants
    ADD COLUMN country VARCHAR (255) NOT NULL,
    ADD COLUMN level_of_study VARCHAR (255) NOT NULL,
    ADD COLUMN age BIGINT NOT NULL,
    ADD COLUMN mlh_mailing_list BOOLEAN NOT NULL;
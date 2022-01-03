-- This file should undo anything in `up.sql`
ALTER TABLE registrants
    DROP COLUMN country,
    DROP COLUMN level_of_study,
    DROP COLUMN age,
    DROP COLUMN mlh_mailing_list;

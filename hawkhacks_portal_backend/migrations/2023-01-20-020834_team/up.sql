-- Your SQL goes here
CREATE TABLE teams
(
    id              SERIAL PRIMARY KEY NOT NULL,
    team_name       text NOT NULL,
    cat_1_eligible  boolean DEFAULT FALSE NOT NULL,
    cat_2_eligible  boolean DEFAULT FALSE NOT NULL,
    cat_3_eligible  boolean DEFAULT FALSE NOT NULL
);
-- Your SQL goes here
CREATE TABLE participants
(
    id              SERIAL PRIMARY KEY NOT NULL,
    first_name      text NOT NULL,
    last_name       text NOT NULL,
    age             int NOT NULL,
    years_exp       int NOT NULL,
    email           text NOT NULL,
    discord         text NOT NULL,
    security_q_1    int NOT NULL,
    security_q_2    int NOT NULL,
    security_q_3    int NOT NULL,
    security_ans_1  text NOT NULL,
    security_ans_2  text NOT NULL,
    security_ans_3  text NOT NULL,
    uuid            text NOT NULL,
    team_id         int,
    CONSTRAINT fk_team_id FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE SET NULL
);
-- Your SQL goes here
CREATE TABLE team_invites
(
    id              SERIAL PRIMARY KEY NOT NULL,
    participant_id  int NOT NULL,
    CONSTRAINT fk_participant_id FOREIGN KEY (participant_id) REFERENCES participants (id) ON DELETE CASCADE,
    invite_UUID     text NOT NULL
);
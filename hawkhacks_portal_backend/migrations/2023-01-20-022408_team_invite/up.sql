-- Your SQL goes here
CREATE TABLE team_invites
(
    id                      SERIAL PRIMARY KEY NOT NULL,
    sender_id   int NOT NULL,
    CONSTRAINT fk_sender_id FOREIGN KEY (sender_id) REFERENCES participants (id) ON DELETE CASCADE,
    recipient_id  int NOT NULL,
    CONSTRAINT fk_recipient_id FOREIGN KEY (recipient_id) REFERENCES participants (id) ON DELETE CASCADE,
    invite_UUID     text NOT NULL
);
// @generated automatically by Diesel CLI.

diesel::table! {
    participants (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        age -> Int4,
        years_exp -> Int4,
        email -> Text,
        discord -> Text,
        security_q_1 -> Int4,
        security_q_2 -> Int4,
        security_q_3 -> Int4,
        security_ans_1 -> Text,
        security_ans_2 -> Text,
        security_ans_3 -> Text,
        uuid -> Text,
        team_id -> Nullable<Int4>,
        private_key -> Text,
    }
}

diesel::table! {
    team_invites (id) {
        id -> Int4,
        sender_id -> Int4,
        recipient_id -> Int4,
        invite_uuid -> Text,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        team_name -> Text,
        cat_1_eligible -> Bool,
        cat_2_eligible -> Bool,
        cat_3_eligible -> Bool,
    }
}

diesel::joinable!(participants -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    participants,
    team_invites,
    teams,
);

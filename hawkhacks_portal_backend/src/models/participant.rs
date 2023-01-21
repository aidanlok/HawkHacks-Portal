use serde::{Serialize, Deserialize};
use diesel::{Queryable, Identifiable, Associations, Insertable};

use crate::schema::{participants};
use crate::models::team::Team;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Clone, PartialEq, Debug)] 
#[diesel(belongs_to(Team))]
#[diesel(table_name = participants)]
pub struct Participant {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub years_exp: i32,
    pub email: String,
    pub discord: String,
    pub security_q_1: i32,
    pub security_q_2: i32,
    pub security_q_3: i32,
    pub security_ans_1: String,
    pub security_ans_2: String,
    pub security_ans_3: String,
    pub uuid: String,
    pub team_id: Option<i32>,
    pub private_key: String,
}

// 
// FILTER
// 
// DONT NEED A SEPARATE STRUCT FOR THIS

// 
// INSERT
// 
#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = participants)]
pub struct InsertParticipant {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub years_exp: i32,
    pub email: String,
    pub discord: String,
    pub security_q_1: i32,
    pub security_q_2: i32,
    pub security_q_3: i32,
    pub security_ans_1: String,
    pub security_ans_2: String,
    pub security_ans_3: String,
}

// 
// UPDATE
// 
pub struct UpdateParticipant {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i32>,
    pub years_exp: Option<i32>,
    pub email: Option<String>,
    pub discord: Option<String>,
}
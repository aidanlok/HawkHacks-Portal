use serde::{Serialize, Deserialize};
use diesel::{Queryable};

#[derive(Deserialize, Serialize, Queryable, Clone)] 
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
}
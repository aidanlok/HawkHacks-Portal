use serde::{Serialize, Deserialize};
use diesel::{Queryable, Identifiable, Insertable};

use crate::schema::{teams};

#[derive(Identifiable, Serialize, Deserialize, Queryable, Clone, PartialEq, Debug)] 
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub team_name: String,
    pub cat_1_eligible: bool,
    pub cat_2_eligible: bool,
    pub cat_3_eligible: bool,
}

// 
// FILTER
// 
// MAYBE I DONT NEED THIS?

// 
// INSERT
// 
#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = teams)]
pub struct InsertTeam {
    pub team_name: String,
    pub cat_1_eligible: bool,
    pub cat_2_eligible: bool,
    pub cat_3_eligible: bool,
}

// 
// UPDATE
// 
pub struct UpdateTeam {
    pub team_name: Option<String>,
}
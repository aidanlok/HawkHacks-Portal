use serde::{Serialize, Deserialize};
use diesel::{Queryable, Identifiable, Associations, Insertable};

use crate::schema::{team_invites};
use crate::models::participant::Participant;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Clone, PartialEq, Debug)] 
#[diesel(belongs_to(Participant, foreign_key = sender_id, foreign_key = recipient_id))]
#[diesel(table_name = team_invites)]
pub struct TeamInvite {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub invite_UUID: String,
}

// 
// FILTER
// 
// DONT NEED A SEPARATE STRUCT FOR THIS

// 
// INSERT
// 
#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = team_invites)]
pub struct InsertTeamInvite {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub invite_uuid: String,
}

// 
// UPDATE
// 
// Not allowing updates to invites.
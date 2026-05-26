use serde::{Serialize,Deserialize};




#[derive(Serialize,Deserialize)]
pub struct Note {
    pub id:i32,
    pub name:String,
    pub tag:String,
    pub created_at:String
}


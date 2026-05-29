use serde::{Serialize,Deserialize};



#[derive(Serialize,Deserialize)]
pub struct Note{
    pub id: u32,
    pub name:String,
    pub tag:String,
    pub created_at:String
}


#[derive(Debug)]
pub enum Command {
     Add{name:String,tag:String},
     List,
     Search {name:String},
}



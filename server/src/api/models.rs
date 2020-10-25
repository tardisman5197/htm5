use serde::{Serialize, Deserialize};

// AddMessage is the struct received from an Add Message
// request.
// Debug allows us to print the object
// Deserialize, Serialize allows for json marshaling
#[derive(Deserialize, Serialize, Debug)]
pub struct AddMessage {
    pub message: String,
    pub sender: String,
    pub receiver: String,
}

// ValidMessage is the struct received from a
// Valid Message request
#[derive(Deserialize, Serialize, Debug)]
pub struct ValidMessage {
    pub message: String,
}

// Message stores all the fields that the message
// table contains.
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub _id: i32,
    pub sender: String,
    pub receiver: String,
    pub timesent: String,
    pub timeread: String,
    pub message: String,
    pub artist: String,
    pub song: String,
    pub link: String,
}
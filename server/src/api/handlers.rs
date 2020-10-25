use warp::{http::StatusCode};
use serde::{Serialize, Deserialize};
use chrono::{Utc};
use postgres::{Client, NoTls};
use super::genius;


#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    _id: String,
    sender: String,
    receiver: String,
    timesent: String,
    timeread: String,
    message: String,
    artist: String,
    song: String,
    link: String,
}

// AddMessage is the struct received from an Add Message
// request.
// Debug allows us to print the object
// Deserialize, Serialize allows for json marshaling
#[derive(Deserialize, Serialize, Debug)]
pub struct AddMessage {
    message: String,
    sender: String,
    receiver: String,
}

// ValidMessage is the struct received from a
// Valid Message request
#[derive(Deserialize, Serialize, Debug)]
pub struct ValidMessage {
    message: String,
}

// add_message receives a message and finds
// its song before adding it to the database
pub async fn add_message(
    addMsg: AddMessage,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Add: {:?}", addMsg);
    
    // Check that a message is a song lyric
    // If not return bad request
    let details = match genius::get_details(msg.message.to_string()) {
        Ok(details) => details,
        Err(e) => return Ok(warp::reply::with_status(format!("Error adding message: {:?}", e), StatusCode::BAD_REQUEST))
    };
    
    // TODO:
    //  Store in DB
    // Connect to the DB
    let mut client =  match Client::connect("postgresql://admin:password@chat_database:5432", NoTls) {
        Ok(client) => {
            println!("Connected to Database");
            client
        }
        Err(e) => {
            println!("Error connection to DB: {}", e);
            return 
        }
    };

    let msg = Message {
        _id: "".to_string(),
        sender: addMsg.sender.to_string(),
        receiver: addMsg.receiver.to_string(),
        timesent: Utc::now().to_rfc3339(),
        timeread: Utc::now().to_rfc3339(),
        message: addMsg.message.to_string(),
        artist: details.0.to_string(),
        song: details.1.to_string(),
        link: "http://a.com".to_string(),
    };

    match client.execute(
        "INSERT INTO messages (sender, receiver, timesent, timeread, message, artist, song, link) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
         &[&msg.sender.to_string(), &msg.receiver.to_string(), &msg.timesent.to_string(), &msg.timeread.to_string(), &msg.message.to_string(), &msg.artist.to_string(), &msg.song.to_string(), &msg.link.to_string()]
    ) {
        Ok(client) => {
            println!("Inserted Message 2");
            client
        }
        Err(e) => {
            println!("Error sending to DB: {}", e);
            return 
        }
    };


    Ok(warp::reply::with_status(
        format!("Add: {:?} -> {:?}", msg, details),
        StatusCode::OK,
    ))
}

// valid_message checks to see if a message is
// actually a song lyric
pub async fn valid_message(
    msg: ValidMessage,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Valid: {:?}", msg);

    // Check that a message is a song lyric
    // If not return bad request
    let details = match genius::get_details(msg.message.to_string()) {
        Ok(details) => details,
        Err(e) => return Ok(warp::reply::with_status(format!("Error adding message: {:?}", e), StatusCode::BAD_REQUEST))
    };

    Ok(warp::reply::with_status(
        format!("Valid: {:?} -> {:?}", msg, details),
        StatusCode::OK,
    ))
}

// read_message modifies an existing message
// in the database to say it has been read
pub async fn read_message(
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Read: {}", id);
    // TODO:
    //  Alter message in DB
    Ok(warp::reply::with_status(
        format!("Read: {}", id),
        StatusCode::OK,
    ))
}

// messages returns a list of messages from the
// an to a certain user
pub async fn messages(
    sender: String,
    receiver: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Messages: {} -> {}", sender, receiver);
    // TODO:
    //  Retrieval of all messages with sender and receiver

    // Connect to the DB
    let mut client =  match Client::connect("postgresql://admin:password@chat_database:5432", NoTls) {
        Ok(client) => {
            println!("Connected to Database");
            client
        }
        Err(e) => {
            println!("Error connection to DB: {}", e);
            return 
        }
    };

    let mut messages: Vec<Message> = Vec::new();

    for row in match client.query("SELECT * FROM messages WHERE sender = $1 AND receiver = $2", &[&sender, &receiver]) {
        Ok(row) => {
            row
        }
        Err(e) => {
            println!("Error getting messages from DB: {}", e);
            return 
        }
    } {
        let message = Message {
            _id: "".to_string(),
            sender: row.get(1),
            receiver: row.get(2),
            timesent: row.get(3),
            timeread: row.get(4),
            message: row.get(5),
            artist: row.get(6),
            song: row.get(7),
            link: row.get(8),
        };
        messages.push(message);
    }

    println!("Messages: {:?}", messages);

    Ok(warp::reply::with_status(
        format!("Messages: {} -> {}", sender, receiver),
        StatusCode::OK,
    ))
}

// health is an endpoint which can be used to check if the
// server is running
pub async fn health() -> Result<impl warp::Reply, warp:: Rejection> {
    println!("Health check");
    Ok(StatusCode::OK)
}
use warp::{http::StatusCode};
use serde::{Serialize, Deserialize};
use super::genius;

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
    msg: AddMessage,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Add: {:?}", msg);
    // TODO:
    //  Check if song lyric
    //  Store in DB

    // Check that a message is a song lyric
    // If not return bad request
    let details = match genius::get_details(msg.message.to_string()) {
        Ok(details) => details,
        Err(e) => return Ok(warp::reply::with_status(format!("Error adding message: {:?}", e), StatusCode::BAD_REQUEST))
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
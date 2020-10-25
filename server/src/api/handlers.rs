use warp::{http::StatusCode};
use chrono::{Utc};

use super::genius;
use super::models;
use super::db;

// add_message receives a message and finds
// its song before adding it to the database
pub async fn add_message(
    add_msg: models::AddMessage,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Add: {:?}", add_msg);
    
    // Check that a message is a song lyric
    // If not return bad request
    let details = match genius::get_details(add_msg.message.to_string()) {
        Ok(details) => details,
        Err(e) => return Ok(warp::reply::with_status(format!("Error adding message: {:?}", e), StatusCode::BAD_REQUEST))
    };
    
    // Create message to store in DB
    let msg = models::Message {
        _id: 0,
        sender: add_msg.sender.to_string(),
        receiver: add_msg.receiver.to_string(),
        timesent: Utc::now().to_rfc3339(),
        timeread: Utc::now().to_rfc3339(),
        message: add_msg.message.to_string(),
        artist: details.0.to_string(),
        song: details.1.to_string(),
        link: "http://a.com".to_string(),
    };

    // Store message in DB
    match db::insert_message(&msg) {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Add: {:?} -> {:?}", msg, details),
            StatusCode::OK,
        )),
        Err(e) => return Ok(
            warp::reply::with_status(
                format!("Error adding message: {:?}", e), 
                StatusCode::BAD_REQUEST
            )
        )
    }
}

// valid_message checks to see if a message is
// actually a song lyric
pub async fn valid_message(
    msg: models::ValidMessage,
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
    id: i32,
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

    // Get messages from DB
    match db::get_messages(&sender, &receiver) {
        Ok(messages) => {
            println!("Messages: {:?}", messages);
            let messages_json = match serde_json::to_string(&messages) {
                Ok(messages_json) => messages_json,
                Err(e) => return Ok(
                    warp::reply::with_status(
                        format!("Error getting messages: {:?}", e), 
                        StatusCode::BAD_REQUEST
                    )
                )
            };
            return Ok(warp::reply::with_status(
                messages_json,
                StatusCode::OK,
            ))
        }
        Err(e) => return Ok(
            warp::reply::with_status(
                format!("Error getting messages: {:?}", e), 
                StatusCode::BAD_REQUEST
            )
        )
    }
}

// health is an endpoint which can be used to check if the
// server is running
pub async fn health() -> Result<impl warp::Reply, warp:: Rejection> {
    println!("Health check");
    Ok(StatusCode::OK)
}
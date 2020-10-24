use warp::{Filter, http::StatusCode};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
struct AddMessage {
    message: String,
    sender: String,
    receiver: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ValidMessage {
    message: String,
}

#[tokio::main]
async fn main() {
    // Read in argument for port
    // TODO

    // POST /message/add
    let send = warp::post()
        .and(warp::path!("message" / "add"))
        .and(warp::body::content_length_limit(1024 * 32)) // Set the body limit
        .and(warp::body::json()) // The body is json
        .map(|msg: AddMessage| {
            println!("Add: {:?}", msg);
            format!("Add: {:?}", msg)
        });

    // POST /message/valid
    let valid = warp::post()
        .and(warp::path!("message" / "valid"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(|msg: ValidMessage| {
            println!("Is it valid? {:?}", msg);
            format!("Is it valid? {:?}", msg)
        });

    // PATCH /message/<id>/read
    let read = warp::patch()
        .and(warp::path!("message" / String / "read"))
        .map(|id| {
            println!("Read: {}", id);
            format!("Read: {}", id)
        });

    // GET /messages/<sender>/<receiver>
    let messages = warp::get()
        .and(warp::path!("messages" / String / String ))
        .map(|sender, receiver| {
            println!("Messages: {} -> {}", sender, receiver);
            format!("Messages: {} -> {}", sender, receiver)
        });

    // GET /health
    let health = warp::get()
        .and(warp::path!("health"))
        .map(|| {
            println!("Health check");
            StatusCode::OK
        });

    // Collect all of the messages
    let routes = send
        .or(valid)
        .or(read)
        .or(messages)
        .or(health);

    // Listen for requests
    println!("Starting Server");
    warp::serve(routes).run(([0,0,0,0],8080)).await;
}
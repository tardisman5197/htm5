use warp::Filter;

// import the api::handlers module
// as filters (this file) is a child
// of the api module, you can import
// using super (api) then handlers.
use super::handlers;

// all of the routes for the API
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    send()
        .or(valid())
        .or(read())
        .or(messages())
        .or(health())
}

// POST /message/add
pub fn send() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("message" / "add"))
        .and(warp::body::content_length_limit(1024 * 32)) // Set the body limit
        .and(warp::body::json()) // The body is json
        .and_then(handlers::add_message)
}

// POST /message/valid
pub fn valid() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("message" / "valid"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and_then(handlers::valid_message)
}

// PATCH /message/<id>/read
pub fn read() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::patch()
        .and(warp::path!("message" / i32 / "read"))
        .and_then(handlers::read_message)
}

// GET /messages/<sender>/<receiver>
pub fn messages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
    .and(warp::path!("messages" / String / String ))
    .and_then(handlers::messages)
}

// GET /health
pub fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("health"))
        .and_then(handlers::health)
}
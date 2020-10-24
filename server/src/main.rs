mod api;

#[tokio::main]
async fn main() {
    // Read in argument for port
    // TODO

    // Get the routes
    let routes = api::filters::routes();

    // Listen for requests
    println!("Starting Server");
    warp::serve(routes).run(([0,0,0,0],8080)).await;
}
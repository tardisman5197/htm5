use std::{thread, time};

mod api;

#[tokio::main]
async fn main() {
    // Read in argument for port
    // TODO

    // Sleep to allow for DB to start up
    let ten_millis = time::Duration::from_millis(2000);
    thread::sleep(ten_millis);

    // Init Database
    match api::db::init_table() {
        Ok(_) => println!("Init DB"),
        Err(e) => {
            println!("Error setting up DB: {:?}", e);
            return 
        }
    }
    
    // Get the routes
    let routes = api::filters::routes();

    // Listen for requests
    println!("Starting Server");
    warp::serve(routes).run(([0,0,0,0],8080)).await;
}


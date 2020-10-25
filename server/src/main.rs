use serde::{Serialize, Deserialize};
use std::{thread, time};

mod api;


#[tokio::main]
async fn main() {
    // Read in argument for port
    // TODO

    // Sleep to allow for DB to start up
    let ten_millis = time::Duration::from_millis(2000);
    thread::sleep(ten_millis);

    match client.batch_execute("
    CREATE TABLE messages (
        id SERIAL NOT NULL PRIMARY KEY,
        sender varchar(80),
        receiver varchar(80),
        timesent varchar(200),
        timeread varchar(200),
        message varchar(200),
        artist varchar(80),
        song varchar(80),
        link varchar(80)
    );
    ") {
        Ok(_) => {
            println!("Created Table")
        }
        Err(e) => {
            println!("Error creating table: {}", e);
            return 
        }
    };

    match client.batch_execute("
        CREATE INDEX sender_index ON messages (sender);
        CREATE INDEX receiver_index ON messages (receiver);
    ") {
        Ok(client) => {
            println!("Indexed Table");
            client
        }
        Err(e) => {
            println!("Error indexing table: {}", e);
            return 
        }
    };
    
    // Get the routes
    let routes = api::filters::routes();

    // Listen for requests
    println!("Starting Server");
    warp::serve(routes).run(([0,0,0,0],8080)).await;
}


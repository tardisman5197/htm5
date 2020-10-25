use postgres::{Client, NoTls};

use super::models;

// init_table creates the messages table in the DB
pub fn init_table() -> Result<(), postgres::Error> {
    // Connect to the DB
    let mut client =  match Client::connect("postgresql://admin:password@chat_database:5432", NoTls) {
        Ok(client) => {
            println!("Connected to Database");
            client
        }
        Err(e) => {
            println!("Error creating table: {}", e);
            return Err(e)
        }
    };

    // Create Tables
    match client.batch_execute("
    CREATE TABLE messages (
        id SERIAL NOT NULL PRIMARY KEY,
        sender varchar(80) NOT NULL,
        receiver varchar(80) NOT NULL,
        timesent varchar(200) NOT NULL,
        timeread varchar(200),
        message varchar(200) NOT NULL,
        artist varchar(80) NOT NULL,
        song varchar(80) NOT NULL,
        link varchar(80) NOT NULL
    );
    ") {
        Ok(_) => println!("Created Table"),
        Err(e) => {
            println!("Error creating table: {}", e);
            return Err(e)
        }
    };

    // Index Tables
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
            return Err(e)
        }
    };

    Ok(())
}

// insert_messages tries to add a message into the database
pub fn insert_message(msg: &models::Message) -> Result<(), postgres::Error> {
    // Connect to the DB
    let mut client =  match Client::connect("postgresql://admin:password@chat_database:5432", NoTls) {
        Ok(client) => {
            println!("Connected to Database");
            client
        }
        Err(e) => {
            return Err(e)
        }
    };

    // Try to insert the message
    match client.execute(
        "INSERT INTO messages (sender, receiver, timesent, message, artist, song, link) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[
            &msg.sender.to_string(), 
            &msg.receiver.to_string(), 
            &msg.timesent.to_string(),
            &msg.message.to_string(),
            &msg.artist.to_string(), 
            &msg.song.to_string(), 
            &msg.link.to_string()
        ]
    ) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(e)
        }
    };
}

// get_messages gets all of the messages from a sender to a receiver
// in the database.
pub fn get_messages(sender: &String, receiver: &String) -> Result<Vec<models::Message>, postgres::Error> {
    // Connect to the DB
    let mut client =  match Client::connect("postgresql://admin:password@chat_database:5432", NoTls) {
        Ok(client) => {
            println!("Connected to Database");
            client
        }
        Err(e) => {
            return Err(e)
        }
    };

    let mut messages: Vec<models::Message> = Vec::new();

    // Get each row that matches the query
    for row in match client.query("SELECT * FROM messages WHERE sender = $1 AND receiver = $2", &[&sender, &receiver]) {
        Ok(row) => row,
        Err(e) => {
            return Err(e)
        }
    } {
        // Create the message struct from the db data
        // and add to the messages vector
        let message = models::Message {
            _id: row.get(0),
            sender: row.get(1),
            receiver: row.get(2),
            timesent: row.get(3),
            timeread: match row.try_get(4) {
                Ok(timeread) => timeread,
                Err(_) => "NULL".to_string(),
            },
            message: row.get(5),
            artist: row.get(6),
            song: row.get(7),
            link: row.get(8),
        };
        messages.push(message);
    }

    Ok(messages)
}

// read_message will update the read time for a message in the
// database.
pub fn read_message(id: &i32, time: &String) -> Result<(), postgres::Error> {
    // Connect to the DB
    let mut client =  match Client::connect("postgresql://admin:password@chat_database:5432", NoTls) {
        Ok(client) => {
            println!("Connected to Database");
            client
        }
        Err(e) => {
            return Err(e)
        }
    };

    // Try to insert the message
    match client.execute(
        "UPDATE messages SET timeread = $1 WHERE id = $2",
        &[
            &time,
            &id
        ]
    ) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(e)
        }
    };
}
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mini_redis::{Connection, Frame};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Creates a TCP listener bound to the address 127.0.0.1:6379. 
    // The .await is used because bind is an asynchronous operation, 
    // and await allows the program to wait for the operation to complete without blocking the current thread
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    // A new HashMap is created and wrapped with Arc (for thread-safety) and Mutex (for thread-safe sharing) to be used as the database
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // Waits for a new incoming connection. 
        // When a connection is accepted, it returns a TcpStream (the socket) and the address of the client
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map. Creates a new reference-counted pointer that points to the same underlying data. This increases the reference count, but the data itself is not copied
        let db = db.clone();

        println!("Accepted");
        // Spawns a new asynchronous task (using spawn) to handle the accepted connection. 
        // The async move closure captures the socket and db variables by value, ensuring they are moved into the new task
        tokio::spawn(async move {
            // The .await here ensures that the task waits for the process function to complete before the task finishes.
            process(socket, db).await;
        });
    }
}

// This code is an asynchronous function process that handles incoming requests from clients in a Redis-like application
async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // An incoming data frame is read from the TCP socket 
    while let Some(frame) = connection.read_frame().await.unwrap() {
        // If a frame is received successfully, it is parsed
        let response = match Command::from_frame(frame).unwrap() {
            // Depending on the type of command (Set or Get), the corresponding actions are performed:
            Set(cmd) => {
                // Access to the db database is locked
                let mut db = db.lock().unwrap();
                // A new key-value pair is inserted into the database
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }           
            Get(cmd) => {
                // Access to the db database is locked
                let db = db.lock().unwrap();
                // The value for the specified key is retrieved from the database
                if let Some(value) = db.get(cmd.key()) {
                    // A response frame (Frame) is created based on the result of executing the command
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
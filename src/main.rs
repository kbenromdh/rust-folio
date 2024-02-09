// import html for responses, GET for getting request handlers, and Router for routing requests
use axum::{response::Html, routing::get, Router};

// set async function main as entry point
#[tokio::main]
async fn main(){

    // create Router instance and register route
    // route will listen for GET requests on root path
    // delegate handling of GET requests to handler function
    let app = Router::new().route("/", get(handler));

    // bind TcpListener instance to localhost on port 3000 asynchronously
    // await to pause execution until operation complete
    // unwrap to retrieve the value inside or force crash
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await.unwrap();

    // print address on console
    println!("Listening on {}", listener.local_addr().unwrap());

    // serve incoming requests with Axum app (runs indefinitely)
    axum::serve(listener, app).await.unwrap();
}

// handler is called whenever a GET request is made to the root path
// asynchronous function, responsible for defining how the server responds to the requests
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!<h1>")
}

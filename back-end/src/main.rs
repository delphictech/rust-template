use axum::{
    http::{HeaderValue, Method},
    response::Html,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {
    // Create a CORS middleware to handle Cross-Origin Resource Sharing
    let cors_middleware = CorsLayer::new()
        .allow_origin("http://localhost:8000/".parse::<HeaderValue>().unwrap()) // Allow requests from localhost:8000
        .allow_methods([Method::GET, Method::POST]); // Allow GET and POST methods

    // Define the routes
    let app = Router::new()
        .route("/", get(|| async { Html("HELLO WORLD") })) // Define a route for the root path
        // .route("/test", get(handlers::handler_test_real)) // Define a route for /test
        // .nest("/coins", {
        //     // Nest routes under /coins
        //     Router::new()
        //         .route("/:userid", get(handlers::handler_coins_balance)) // Define a route for getting the coin balance for a user
        //         .route_layer(middleware::from_fn(middlewares::auth_middleware)) // Apply authentication middleware to all routes under /coins
        // })
        .layer(cors_middleware); // Apply CORS middleware to the entire application

    // Define the address and port
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap(); // Bind the server to listen on all interfaces on port 8000
    println!("->> LISTENING on {:?}\n", listener.local_addr()); // Print out the address and port the server is listening on
    axum::serve(listener, app).await.unwrap(); // Start serving the application
}

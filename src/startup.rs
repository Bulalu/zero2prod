use crate::routes::{health_check, subscribe};
use actix_web::{web, App, HttpServer, dev::Server};
use sqlx::PgConnection;
use std::net::TcpListener;

// Function to start and run the HTTP server.
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the database connection in Actix web::Data for sharing across handlers.
    let connection = web::Data::new(connection);

    // Start building the Actix web server.
    let server = HttpServer::new(move || {
        // Create a new Actix App.
        App::new()
            // Attach the `health_check` handler to `/health_check` route.
            .route("/health_check", web::get().to(health_check))
            // Attach the `subscribe` handler to `/subscriptions` route.
            .route("/subscriptions", web::post().to(subscribe))
            // Attach the cloned database connection to the App.
            .app_data(connection.clone())
    })
        // Bind the server to the provided TcpListener.
        .listen(listener)?
        // Start the server and begin listening for requests.
        .run();

    // Return the Actix Server instance.
    Ok(server)
}

use actix_web::{web, App, get, HttpServer, Responder, HttpResponse};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

// async fn get_item(info: web::Path<(u32,)>) -> impl Responder {
//     let item_id = info.0;
//     // Retrieve item from the database or some data source
//     format!("Item with id {} requested", item_id)
// }

// async fn create_item() -> impl Responder {
//     // Logic to create a new item
//     "Item created"
// }

// async fn update_item(info: web::Path<(u32,)>) -> impl Responder {
//     let item_id = info.0;
//     // Logic to update the item with the given ID
//     format!("Item with id {} updated", item_id)
// }

// async fn delete_item(info: web::Path<(u32,)>) -> impl Responder {
//     let item_id = info.0;
//     // Logic to delete the item with the given ID
//     format!("Item with id {} deleted", item_id)
// }

// // Custom 404 handler
// async fn handle_not_found() -> HttpResponse {
//     HttpResponse::NotFound().body("404 Not Found")
// }

// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    // Create Actix web application
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
            // .service(
            //     web::scope("/")
            //         .guard(guard::Host("www.rust-lang.org"))
            //         .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            // )
            // .service(
            //     web::scope("/")
            //         .guard(guard::Host("users.rust-lang.org"))
            //         .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            // )
            // .route("/", web::get().to(index))
            // .route("/items/{id}", web::get().to(get_item))
            // .route("/items", web::post().to(create_item))
            // .route("/items/{id}", web::put().to(update_item))
            // .route("/items/{id}", web::delete().to(delete_item))
            // // Define the custom 404 handler for all other routes
            // .default_service(web::route().to(handle_not_found))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

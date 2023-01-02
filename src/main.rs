mod ctl;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ctl::product_list_controller)
            .service(ctl::product_detail_controller)
            .service(ctl::product_updating_controller)
            .service(ctl::product_creation_controller)
            .service(ctl::product_deletion_controller)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
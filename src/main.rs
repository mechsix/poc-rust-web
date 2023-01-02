mod ctl;
mod db;

use actix_web::{App, HttpServer, web};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::get_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
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
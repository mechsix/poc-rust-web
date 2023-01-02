use actix_web::{web, get, post, put, delete, Responder};

#[get("/product")]
pub async fn product_list_controller() -> impl Responder {
    "Product list"
}

#[get("/product/{id}")]
pub async fn product_detail_controller(product_id: web::Path<String>) -> impl Responder {
    format!("Product Detail on {}", &product_id)
}

#[put("/product/{id}")]
pub async fn product_updating_controller(product_id: web::Path<String>) -> impl Responder {
    format!("Update Product on {}", product_id)
}

#[post("/product")]
pub async fn product_creation_controller() -> impl Responder {
    format!("Create Product")
}

#[delete("/product/{id}")]
pub async fn product_deletion_controller(product_id: web::Path<String>) -> impl Responder {
    format!("Delete Product {}", product_id)
}

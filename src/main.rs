use actix_web::{get, web, App, HttpServer, Responder};
use log::info; 
use serde::Serialize;
mod docker;

#[derive(Serialize)]
struct StatusResponse {
    containers: Vec<docker::ContainerInfo>,
}

#[get("/status")]
async fn get_status() -> impl Responder {
    let containers = docker::get_containers_status().await;
    web::Json(StatusResponse { containers })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting HTTP server");
    HttpServer::new(|| {
        App::new()
            .service(get_status)  // Route qui retourne le statut des conteneurs Docker
    })
    .bind("0.0.0.0:5747")?  // Serveur sur le port 80
    .run()
    .await
}
use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use log::info;
mod docker;

#[get("/api/projects")]
async fn get_status() -> impl Responder {
    let projects = docker::get_projects_status().await;
    web::Json(projects)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting HTTP server");
    HttpServer::new(|| {
        App::new()
            .service(get_status) // Route qui retourne le statut des projets Docker compose
            .service(Files::new("/", "static").index_file("index.html")) // Serveur de fichiers statiques
    })
    .bind("0.0.0.0:5747")? // Serveur sur le port 5747
    .run()
    .await
}

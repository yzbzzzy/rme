mod response_data;
mod system_info;

use std::collections::HashMap;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::web::Json;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Map;


#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./static/index.html"))
}
async fn network() -> impl Responder {
    let data = response_data::response_data{ code: 200, data: system_info::get_network_info() };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

async fn memory() -> impl Responder {
    let data = response_data::response_data{ code: 200, data: system_info::get_mem_info() };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

async fn disk() -> impl Responder {
    let data = response_data::response_data{ code: 200, data: system_info::get_disk_info() };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/network", web::get().to(network))
            .route("/memory",web::get().to(memory))
            .route("/disk",web::get().to(disk))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
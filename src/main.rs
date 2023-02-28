mod response_data;
mod system_info;

use std::{thread};
use std::collections::HashMap;
use std::time::Duration;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use sysinfo::{CpuExt, System, SystemExt};

static mut USAGE:f32 = 0.0;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./static/index.html"))
}
async fn network() -> impl Responder {
    let data = response_data::ResponseData{ code: 200, data: system_info::get_network_info() };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

async fn memory() -> impl Responder {
    let data = response_data::ResponseData{ code: 200, data: system_info::get_mem_info() };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

async fn disk() -> impl Responder {
    let data = response_data::ResponseData{ code: 200, data: system_info::get_disk_info() };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

async  fn cpu() -> impl Responder {
    let mut  map:HashMap<String,String> = HashMap::new();
    unsafe {
        map.insert("cpu_usage".to_string(),USAGE.clone().to_string());
    }
    let data = response_data::ResponseData{ code: 200, data: map};
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(move || unsafe {
        let mut s = System::new();
        s.refresh_all();
        loop {
            thread::sleep(Duration::from_secs(3));
            USAGE=0.0;
            s.refresh_all();
            for cpu in s.cpus(){
                USAGE+=cpu.cpu_usage();
            }
        }
    });
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/network", web::get().to(network))
            .route("/memory",web::get().to(memory))
            .route("/disk",web::get().to(disk))
            .route("/cpu",web::get().to(cpu))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

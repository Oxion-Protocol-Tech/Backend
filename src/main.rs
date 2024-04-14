use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn new_page() -> impl Responder {
    // Открытие файла и чтение его в строку
    let content = include_str!("new.html");
    HttpResponse::Ok().body(content)
}


async fn main_page() -> impl Responder {
    // Открытие файла и чтение его в строку
    let content = include_str!("main.html");
    HttpResponse::Ok().body(content)
}


async fn pools_page() -> impl Responder {
    // Открытие файла и чтение его в строку
    let content = include_str!("pools.html");
    HttpResponse::Ok().body(content)
}


async fn protocol_page() -> impl Responder {
    // Открытие файла и чтение его в строку
    let content = include_str!("protocol.html");
    HttpResponse::Ok().body(content)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/new", web::get().to(new_page)) // Добавление обработчика для /new
            .route("/pools", web::get().to(pools_page))
            .route("/main", web::get().to(main_page))
            .route("/protocol", web::get().to(protocol_page))
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

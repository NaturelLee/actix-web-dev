use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/welcome")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/return")]
async fn returns(req: String) -> impl Responder {
    HttpResponse::Ok().body(req)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn manual() -> impl Responder {
    HttpResponse::Ok().body("Manual")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(returns)
            .service(welcome)
            .route("/hey", web::get().to(manual_hello))
            .route("/manual", web::get().to(manual))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

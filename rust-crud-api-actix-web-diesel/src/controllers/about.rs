use actix_web::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Ola mundo!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
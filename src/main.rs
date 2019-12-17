use actix_web::{get, App, HttpServer, Responder, HttpRequest, HttpResponse};
use serde_json::{json, Value, Map};


#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let headers: Map<String, Value> = req.headers().iter().map(|(name, value)| {
        (String::from(name.as_str()), json!(value.to_str().unwrap()))
    }).collect();

    HttpResponse::Ok().json(headers)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
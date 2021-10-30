use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("Got a GET request!");
    HttpResponse::Ok().finish()
}

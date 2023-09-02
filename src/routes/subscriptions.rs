use actix_web::{web, HttpResponse};


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}


// your handler does not have to deal with the raw incoming request and
// can instead work directly with strongly-typed information,
// significantly simplifying the code that you need to write to handle a request
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
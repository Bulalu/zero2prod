use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}


// your handler does not have to deal with the raw incoming request and
// can instead work directly with strongly-typed information,
// significantly simplifying the code that you need to write to handle a request
pub async fn subscribe(
    _form: web::Form<FormData>,
    // Retrieving the database connection from the App state
    pool: web::Data<PgPool>

) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %_form.email,
        subscriber_name= %_form.name
    );
    let _request_span_guard = request_span.enter();
    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments // in the query future lifetime
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
);
   match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    ).execute(pool.as_ref())
        .await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e); HttpResponse::InternalServerError().finish()
        }
    }

}


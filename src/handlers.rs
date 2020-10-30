use actix_web::{client::Client, web, Error, HttpResponse};

use crate::payload::email::Email;
use crate::utils::send_email::send_verification;

pub async fn get_subscriber(
    some_data: web::Json<Email>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    send_verification(some_data.into_inner(), &client).await?;
    Ok(HttpResponse::Ok().finish())
}

use futures::StreamExt;

use crate::payload::{email::Email, smtp_payload::SMTP2goResponse};
use actix_web::{client::Client, error::ErrorBadRequest, web::BytesMut, Error};
use validator::Validate;

pub async fn send_verification(data: Email, client: &Client) -> Result<(), Error> {
    data.validate().map_err(ErrorBadRequest)?;
    let mut res = client
        .post("https://api.smtp2go.com/v3/email/send")
        .send_json(&data)
        .await
        .map_err(Error::from)?; // <- convert SendRequestError to an Error
    debug!("{:?}", res);

    if res.status() == 200 {
        return Ok(());
    }

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let body: SMTP2goResponse = serde_json::from_slice(&body).unwrap();
    Ok(())
}

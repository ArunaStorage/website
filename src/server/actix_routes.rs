use crate::server::oidc::Authorizer;
use actix_session::Session;
use actix_web::{
    http::StatusCode,
    web::{Bytes, Data, Redirect},
    *,
};
use futures::channel::mpsc;
use serde::Deserialize;
use std::sync::Mutex;

#[get("/login")]
pub async fn login(session: Session, data: Data<Mutex<Authorizer>>) -> Result<impl Responder> {
    let my_data = data
        .lock()
        .map_err(|_| error::InternalError::new("Poison", StatusCode::INTERNAL_SERVER_ERROR))?;

    let url = my_data.get_challenge(session).map_err(|_| {
        error::InternalError::new("GetChallenge", StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    Ok(Redirect::to(url.to_string()).see_other())
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub state: String,
    pub session_state: String,
    pub code: String,
}

#[get("/callback")]
pub async fn callback(
    req: HttpRequest,
    session: Session,
    data: Data<Mutex<Authorizer>>,
) -> Result<impl Responder> {
    let query_params = web::Query::<Params>::from_query(req.query_string())?;

    let my_data = data
        .lock()
        .map_err(|_| error::InternalError::new("Poison", StatusCode::INTERNAL_SERVER_ERROR))?;

    my_data
        .exchange_challenge(session, &query_params.code)
        .await
        .map_err(|_| {
            error::InternalError::new("Exchange Code", StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    Ok(Redirect::to("/panel").see_other())
}

#[get("/api/events")]
async fn update_events(session: Session) -> Result<impl Responder> {
    let (tx, rx) = mpsc::unbounded::<Result<Bytes, Error>>();
    session.insert("user_id", format!("{}", uuid::Uuid::new_v4()))?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .streaming(rx))
}

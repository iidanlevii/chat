use crate::error::Error;
use crate::model::user::{User, UserModel};
use crate::{web, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};

pub fn routes(user_model: UserModel) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/register", post(register))
        .with_state(user_model)
}

async fn api_login(
    cookies: Cookies,
    State(user_model): State<UserModel>,
    payload: Json<User>,
) -> Result<()> {
    println!("->> {:<12} - login", "HANDLER");

    let Json(user) = payload;
    let db_user = user_model.get_user(user.clone()).await?;

    if user.password != db_user.password {
        return Err(Error::LoginFail);
    }

    add_user_cookie(cookies, db_user);

    Ok(())
}

async fn register(
    cookies: Cookies,
    State(user_model): State<UserModel>,
    Json(payload): Json<User>,
) -> Result<()> {
    println!("->> {:<12} - register", "HANDLER");

    user_model.add_user(payload.clone()).await?;

    add_user_cookie(cookies, payload);

    Ok(())
}

fn add_user_cookie(cookies: Cookies, user: User) {
    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
}

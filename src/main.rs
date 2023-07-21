use crate::{error::Result, model::user::UserModel};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use web::routes_user;

use axum::{middleware, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::model::message::MessageModel;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let message_model = MessageModel::new().await?;
    let user_model = UserModel::new().await?;

    let message_api = web::routes_message::routes(message_model.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let user_api = routes_user::routes(user_model.clone());

    // build our application with a single route
    let app = Router::new()
        .merge(user_api)
        .nest("/api", message_api)
        .layer(middleware::from_fn_with_state(
            message_model.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

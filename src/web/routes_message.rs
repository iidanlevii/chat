use axum::{extract::State, Json, Router};

use crate::{
    ctx::Ctx,
    error::Result,
    model::message::{Message, MessageModel, MessageToCreate},
};
use axum::routing::post;

pub fn routes(message_controller: MessageModel) -> Router {
    Router::new()
        .route("/messages", post(add_message).get(all_messages))
        .with_state(message_controller)
}

async fn add_message(
    State(message_model): State<MessageModel>,
    ctx: Ctx,
    Json(message_tc): Json<MessageToCreate>,
) -> Result<()> {
    println!("->> {:<12} - add_message", "HANDLER");

    message_model.add_message(ctx, message_tc).await?;

    Ok(())
}

async fn all_messages(
    State(message_model): State<MessageModel>,
    ctx: Ctx,
) -> Result<Json<Vec<Message>>> {
    println!("->> {:<12} - all_messages", "HANDLER");

    let messages = message_model.find_messages_by_user(ctx.user_id()).await?;

    Ok(Json(messages))
}

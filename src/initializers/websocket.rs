use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;
// use serde::{Deserialize, Serialize};
// use socketioxide::{
//     extract::{Data, Extension, SocketRef, State},
//     SocketIo,
// };
use socketioxide::{SocketIo, extract::SocketRef};
// use std::sync::{atomic::AtomicUsize, Arc};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::services::broadcast::broadcaster;

#[allow(clippy::module_name_repetitions)]
pub struct WebSocketInitializer;

#[async_trait]
impl Initializer for WebSocketInitializer {
    fn name(&self) -> String {
        "websocket-broadcaster".to_string()
    }
    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let (layer, io) = SocketIo::builder().build_layer();

        io.ns("/", |socket_ref: SocketRef| {
            socket_ref.on("message", |socket_ref: SocketRef| {
                socket_ref.emit("message-back", "Hello World!").ok();
            });
            socket_ref.on_disconnect(|socket_ref: SocketRef| async move {
                println!(
                    "Socket {} on ns {} disconnected",
                    socket_ref.id,
                    socket_ref.ns(),
                );
            });
        });

        // Initialize the global broadcaster with the SocketIo instance
        broadcaster::init(io);

        let router = router.layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

        Ok(router)
    }
}

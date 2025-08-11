use once_cell::sync::OnceCell;
use serde::Serialize;
use socketioxide::SocketIo;

static SOCKET_IO: OnceCell<SocketIo> = OnceCell::new();

#[allow(clippy::missing_panics_doc)]
pub fn init(io: SocketIo) {
    SOCKET_IO
        .set(io)
        .expect("Socket.IO instance already initialized");
}

#[allow(clippy::missing_panics_doc)]
pub async fn broadcast(event: &str, data: impl Serialize + Send + Sync + 'static) {
    if let Some(io) = SOCKET_IO.get() {
        io.emit(event, &data).await.ok();
    }
}

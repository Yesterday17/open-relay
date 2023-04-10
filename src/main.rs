use std::{net::SocketAddr, process::Command};

use axum::{routing::post, Router};

async fn open(body: String) -> String {
    let mut command = Command::new("xdg-open");
    command.arg(body);
    match command.status() {
        Ok(status) => {
            if status.success() {
                format!("success")
            } else {
                format!("code: {:?}", status.code())
            }
        }
        Err(e) => format!("error: {e}"),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(open));

    let addr = SocketAddr::from(([0, 0, 0, 0], 6655));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

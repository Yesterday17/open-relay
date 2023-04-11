use std::{
    net::SocketAddr,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use axum::{routing::post, Router};

async fn open(body: String) -> Result<(), String> {
    eprintln!("body: {}", body);
    let mut command = Command::new("xdg-open")
        .arg(body)
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to spawn: {e:?}"))?;

    sleep(Duration::from_secs(5));
    let _ = command.kill();

    Ok(())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(open));

    let addr = SocketAddr::from(([0, 0, 0, 0], 6000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

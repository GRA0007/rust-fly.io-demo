use std::{env, io::BufWriter, net::SocketAddr};

use axum::{response::IntoResponse, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!(
        "Listening at http://{} in {} mode",
        addr,
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );

    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler")
        })
        .await
        .unwrap();
}

async fn get_root() -> impl IntoResponse {
    let mut writer = BufWriter::new(Vec::new());
    ferris_says::say(
        &env::var("MESSAGE").expect("MESSAGE env variable not set!"),
        100,
        &mut writer,
    )
    .unwrap();
    String::from_utf8(writer.into_inner().unwrap()).unwrap()
}

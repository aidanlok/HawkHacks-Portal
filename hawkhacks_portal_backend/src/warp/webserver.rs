use warp::{Filter};

use crate::custom_types::{Users, PGConnPool};
use crate::warp::websocket;

pub async fn start_webserver(pool: PGConnPool, users: Users) {
    let pool_filter = warp::any().map(move || pool.clone());
    let users_filter = warp::any().map(move || users.clone());

    let ws_path = warp::path("websocket")
        .and(warp::ws())
        .and(users_filter)
        .and(pool_filter)
        .map(|ws: warp::ws::Ws, users, pool| {
            ws.on_upgrade(move |socket| websocket::ws_connected(socket, users, pool))
        });

    let routes = ws_path
        .with(warp::cors().allow_any_origin());

    println!("starting webserver");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
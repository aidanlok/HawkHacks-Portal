use std::{sync::atomic::{AtomicUsize, Ordering}, time::{Duration}};
use diesel::PgConnection;
use futures::{StreamExt, SinkExt, TryFutureExt};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::{sync::{mpsc}, time::timeout};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{ws::{Message, WebSocket}};
use log::*;

use crate::custom_types::{Users, PGConnPool};
use crate::models::{team, participant, team_invite};

#[derive(Serialize, Deserialize)]
struct ResultStruct { message: bool }
#[derive(Serialize, Deserialize)]
struct ReturnJSON { result: ResultStruct }

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub async fn ws_connected(ws: WebSocket, users: Users, pool: PGConnPool) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    println!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
            .send(message)
            .unwrap_or_else(|e| {
                error!("websocket send error: {}", e);
            })
            .await;
        }
    });

    // Save the sender in our list of connected users.
    users.write().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.
    trace!("pre recv while loop");

    loop {
        let ws_rx_future = timeout(Duration::from_secs(3), user_ws_rx.next());
        match ws_rx_future.await { 
            Ok(v) => { 
                match v {
                    Some(s) => {
                        info!("new ws msg");
                        
                        let msg = match s {
                            Ok(msg) => msg,
                            Err(e) => {
                                error!("websocket error(uid={}): {}", my_id, e);
                                break;
                            }
                        };

                        process_ws_msg(msg, my_id, &users, &pool).await;
                    },
                    None => { error!("ws rx err"); break; }
                }
            },
            Err(e) => { error!("ws rx future err: {e}") }
        }  
    }

    trace!("post recv while loop");

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users).await;
}

pub async fn process_ws_msg(msg: Message, my_id: usize, users: &Users, pool: &PGConnPool) {
    let p_msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        error!("process ws msg not ok ??");
        return;
    };

    let new_msg = p_msg.to_string();

    info!("{new_msg}");

    // Parse the string of data into serde_json::Value.
    let v: Value = match serde_json::from_str(&new_msg) {
        Result::Ok(val) => {val},
        Result::Err(err) => {
            error!("json decode err: {err}"); 
            return;
        }
    };

    trace!("processed via serde");
     
    // Access parts of the data by indexing with square brackets.
    // info!("ws req op == {}", v["op"]);

    let mut pool_get = pool.get();
    let conn = match &mut pool_get {
        Result::Ok(v) => { v },
        Result::Err(e) => { error!("pool conn get err: {e}"); return; }
    };

    trace!("processing op");
    
    if v["op"] == "c" {
        let body = v["body"].to_string();

        // info!("ws c body: {body}");

        let (nm, did_parse) = message::Message::create_new_message_from_str(&body);

        if did_parse {
            let msg_did_insert = message::Message::insert_new_from_nmsg(conn, &nm, false);

            let rtn = ReturnJSON {
                result: ResultStruct { message: msg_did_insert }
            };
            let msg_insert_result = match serde_json::to_string(&rtn) {
                Ok(v) => { v },
                Err(e) => { eprintln!("{e}"); return; }
            };

            send_ws(users, my_id, &msg_insert_result).await;
        }

    } else if v["op"] == "r" {
        trace!("processing cmd");

        if v["cmd"] == "sms" {
            trace!("sms cmd");
        } else if v["cmd"] == "all_bulk" {
        } else if v["cmd"] == "select_bulk" {
            let extract = v["bulk_name"].to_string();
            let bulk_name = extract.trim_matches('"');

            // info!("{bulk_name}");

            select_bulk_and_ws_send(bulk_name.to_string(), users, my_id, conn).await;
        } else {
            trace!("bad cmd");
            send_ws(users, my_id, "{\"result\": \"bad cmd\"}").await;
        }
    } else {
        trace!("processing cmd");

        send_ws(users, my_id, "{\"result\": \"bad op\"}").await;
    }

    trace!("end of process");
}

pub async fn user_disconnected(my_id: usize, users: &Users) {
    info!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}

pub async fn send_ws(users: &Users, my_id: usize, msg: &str) {
    trace!("sending ws");

    for (&_uid, tx) in users.read().await.iter() {
        if _uid == my_id {
            if let Err(_disconnected) = tx.send(Message::text(msg)) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
                error!("error on ws broadcast: {_disconnected}");
            }
        }
    }

    trace!("sent ws");
}

pub async fn broadcast_ws(users: &Users, msg: &str) {
    trace!("starting bc ws");

    for (&_uid, tx) in users.read().await.iter() {
        if let Err(_disconnected) = tx.send(Message::text(msg)) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
            error!("error on ws broadcast: {_disconnected}");
        }
    }

    trace!("finished bc ws");
}
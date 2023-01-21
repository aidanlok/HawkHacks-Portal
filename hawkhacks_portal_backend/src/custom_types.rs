use std::collections::HashMap;
use std::sync::{
    Arc,
};
use diesel::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message};

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
pub type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

/// PostgreSQL pool of connections.
pub type PGConnPool = Pool<ConnectionManager<PgConnection>>;

/// PostgreSQL connection from a pool.
pub type PGConn = PooledConnection<ConnectionManager<PgConnection>>;

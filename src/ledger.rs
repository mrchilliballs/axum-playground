use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::State};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub(crate) fn router(conn: Arc<Connection>) -> Router {
    Router::new().route("/", root).with_state(conn)
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    id: u32,
    account_id: u32,
    amount: i32,
}

fn root(State(conn): State<Arc<Connection>>) -> Result<Json<Value>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, account_id, amount FROM transactions")?;
    let transactions: Vec<_> = stmt
        .query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                account_id: row.get(1)?,
                amount: row.get(2)?,
            })
        })?
        .collect();
    Ok(Json(json!({"data": transactions})))
}

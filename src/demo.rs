use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde::Serialize;
use tokio_postgres::{NoTls, Client};

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

async fn get_users_handler() -> Json<Vec<User>> {
    // DBに接続
    let (client, connection) =
        tokio_postgres::connect("host=db user=postgres password=postgres dbname=demo", NoTls)
            .await
            .expect("DB接続失敗");

    // 接続をバックグラウンドで処理
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // クエリ実行
    let rows = client.query("SELECT id, name, email, created_at FROM users", &[])
        .await
        .expect("クエリ失敗");

    // データをRust構造体に変換
    let users: Vec<User> = rows.into_iter().map(|row| User {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        created_at: row.get("created_at"),
    }).collect();

    Json(users)
}

#[tokio::main]
async fn main() {
    // ルーター作成
    let app = Router::new()
        .route("/users", get(get_users_handler));

    // サーバー起動
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app
    )
    .await
    .unwrap();
}

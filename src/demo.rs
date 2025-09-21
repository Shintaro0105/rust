use tokio_postgres::{NoTls, Error};
use postgres_types::FromSql; // chrono と統合するため

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 接続文字列
    let conn_str = "host=localhost port=5432 user=postgres password=postgres dbname=demo";

    // 接続
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;

    // connection を別タスクで駆動（これをしないと通信が動かない）
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // データを取得
    let rows = client.query("SELECT id, name, email, created_at FROM users", &[]).await?;

    println!("--- users table ---");
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let created_at: chrono::NaiveDateTime = row.get("created_at");
        println!("{}: {} <{}> ({})", id, name, email, created_at);
    }

    Ok(())
}

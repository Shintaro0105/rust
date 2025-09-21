# Rust + PostgreSQL DevContainer Demo

このリポジトリは、Rust から PostgreSQL に接続してデータを操作するサンプルプロジェクトです。  
最初は CLI アプリとして動作し、拡張すれば **Web サーバー (REST API)** として利用できます。  
VS Code の DevContainer 機能を利用して、Rust アプリと PostgreSQL を Docker 上で動かせます。

---

## 🚀 セットアップ

### 1. DevContainer 起動
VS Code でこのプロジェクトを開き、コマンドパレットから以下を実行します。

```
Dev Containers: Reopen in Container
```

Rust と PostgreSQL の環境が自動的に立ち上がります。

---

### 2. PostgreSQL 確認
DevContainer 起動後、データベースコンテナに接続してテーブルを確認できます。

```bash
docker exec -it rust_devcontainer-db-1 psql -U postgres -d demo
```
---
##　🖥 CLI アプリとして実行

Rust 側から PostgreSQL に接続して users テーブルの内容を表示します。

```bash
cargo run --bin demo
```
---
## 🌐 Web サーバーとして実行
### 1. 依存関係
```Cargo.toml``` に以下を追加します：
```toml
[dependencies]
axum = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

(既に入っている ```tokio```, ```tokio-postgres```, ```chrono```, ```postgres-types``` も利用します)

### 2. サーバー起動
```bash
cargo run
```
### 3. 動作確認
```bash
curl http://localhost:3000/users
```
---
## 🛠 使用技術
- Rust
- Tokio + tokio-postgres
- PostgreSQL
- Docker Compose
- DevContainer (VS Code)
# Rust + PostgreSQL DevContainer Demo

このリポジトリは、Rust から PostgreSQL に接続してデータを操作するサンプルプロジェクトです。  
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
### 3. Rust アプリの実行

Rust 側から PostgreSQL に接続して users テーブルの内容を表示します。

```bash
cargo run --bin demo
```
---
## 🛠 使用技術
- Rust
- Tokio + tokio-postgres
- PostgreSQL
- Docker Compose
- DevContainer (VS Code)
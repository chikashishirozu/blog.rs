# blog.rs

This project is based on:

https://github.com/mateusfg7/blog.rs

Modified by @chikashishirozu


# blog.rs を動かすために必要なもの

# 1. 必須インフラ

PostgreSQLとRedisのローカルインスタンスが必要 githubです。

# PostgreSQL を起動

bash

docker run -d -p 5432:5432 \\

  -e POSTGRES_USER=loco \\
  
  -e POSTGRES_DB=loco_app \\
  
  -e POSTGRES_PASSWORD="loco" \\
  
  postgres:15.3-alpine

# Redis を起動

docker run -d -p 6379:6379 redis:alpine

# 2. 追加で必要な設定ファイル

リポジトリには含まれていない可能性が高いファイル:

.env ファイル（環境変数）

プロジェクトルートに作成:

# .env

DATABASE_URL=postgres://loco:loco@localhost:5432/loco_app

REDIS_URL=redis://localhost:6379

JWT_SECRET=your_secret_key_change_this_in_production

config/development.yaml の確認

開発環境の設定ファイルを確認する必要があります githubが、リポジトリに含まれていない場合は以下を参考に作成:

# config/development.yaml

server:
  port: 5150
  host: 0.0.0.0

database:
  uri: postgres://loco:loco@localhost:5432/loco_app
  enable_logging: true
  auto_migrate: true

redis:
  uri: redis://localhost:6379

auth:
  jwt:
    secret: "development-secret-change-in-production"
    expiration: 3600
    
# 3. Rust のインストール

# Rust がインストールされていない場合

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 最新版に更新

bash

rustup update

# 4. Loco CLI のインストール

bash

cargo install loco-cli

# 5. セットアップ手順

# 1. リポジトリをクローン

bash

git clone https://github.com/chikashishirozu/blog.rs.git

cd blog.rs

# 2. 依存関係をインストール

cargo build

# 3. データベースマイグレーション実行

cargo loco db migrate

# 4. アプリケーション起動

cargo loco start

# 5. 実際に使える Argon2 ハッシュ

Password for all sample users: "password"

Generated via: argon2id, m=19456, t=2, p=1

⚠️ This project is under active development.

⚠️ NOT production-ready.

⚠️ All credentials are dummy values.


# ---- ビルドステージ ----
FROM rust:latest AS builder

# ALSAの開発ヘッダやSSLライブラリをインストール
RUN apt-get update \
 && apt-get install -y \
 pkg-config \
 libssl-dev \
 libasound2-dev \
 && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを作成・移動
WORKDIR /usr/src/app

# 依存関係を先にコピーしてキャッシュを効かせる
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// dummy" > src/lib.rs
RUN cargo fetch

# プロジェクトのソースをコピーしてコンパイル
COPY . .
RUN cargo build --release

# ---- 実行ステージ ----
FROM debian:bookworm-slim

# ランタイムに必要なライブラリを入れる（例：SSL）
RUN apt-get update \
 && apt-get install -y \
      libssl3 \
      libasound2 \
 && rm -rf /var/lib/apt/lists/*

# ビルド結果だけをコピー
COPY --from=builder /usr/src/app/target/release/tetris-rs /usr/local/bin/

# デフォルトでバイナリを実行
ENTRYPOINT ["tetris-rs"]
# syntax=docker/dockerfile:1

# --- 階段 1: 安裝 cargo-chef ---
FROM rust:slim-trixie AS chef

#映像檔減肥: 安裝必要的系統套件，並清理暫存檔案
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef

WORKDIR /usr/src/app

# --- 階段 2: Planner (生成食譜) ---
FROM chef AS planner

COPY ./my_project/ ./my_project
COPY ./sql_derives/ ./sql_derives
COPY ./Cargo.toml ./Cargo.toml

# 分析專案依賴，生成 recipe.json
RUN cargo chef prepare --recipe-path recipe.json

# --- 階段 3: Cacher (編譯依賴) ---
FROM chef AS cacher

COPY --from=planner /usr/src/app/recipe.json recipe.json
# 這是最關鍵的一步：編譯所有依賴檔，但不編譯你的代碼
RUN cargo chef cook --release --recipe-path recipe.json

# --- 階段 4: Builder (編譯主程式) ---
FROM chef AS builder
COPY ./my_project/ ./my_project
COPY ./sql_derives/ ./sql_derives
COPY ./Cargo.toml ./Cargo.toml
# 從 cacher 階段複製已經編譯好的依賴檔
COPY --from=cacher /usr/src/app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# 現在編譯主程式，速度會非常快
RUN cargo build --release

# --- 階段 5: Runtime (執行環境) ---
FROM debian:trixie-slim

RUN touch mysql.db

COPY --from=builder /usr/src/app/target/release/architecture /architecture

COPY my_project/Configure.toml /Configure.toml

EXPOSE 3000

CMD ["/architecture"]

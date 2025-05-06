# Zero2Prod (Rust Backend Playground)

This repository is a Rust-based backend application project. It serves as a personal learning ground and eventual foundation for production-ready backend systems.

## 🚀 Purpose

The goal of this project is to build a robust, maintainable, and performant backend service in Rust. I am following the guidance of the excellent book [_Zero To Production in Rust_](https://www.zero2prod.com/) by Luca Palmieri.

This book walks through building a production-grade backend web service in Rust using:

- **Actix Web** for the web framework
- **PostgreSQL** for the database
- **SQLx** for database interaction
- **Docker** for containerization
- **CI/CD**, testing, observability, and more

Once I've completed the book, I plan to switch out the runtime environment and experiment with alternatives such as:

- Replacing Actix Web with [Axum](https://github.com/tokio-rs/axum) or [Warp](https://github.com/seanmonstar/warp)
- Exploring different async runtimes like [async-std](https://github.com/async-rs/async-std) instead of [Tokio](https://tokio.rs/)
- Modifying the project to be fully serverless, WASM-compatible, or distributed

## 🧱 Tech Stack (Initial)

- 🦀 **Rust**
- ⚙️ **Actix Web** (for now)
- 🛢️ **PostgreSQL**
- 🧪 **SQLx**
- 📦 **Docker**
- 🧪 **Integration & Unit Testing**
- 📈 **Telemetry (Tracing, Logging, Metrics)**
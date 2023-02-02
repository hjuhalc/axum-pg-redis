# axum-pg-redis (WIP)

A highly scalable backend template built with [axum](https://github.com/tokio-rs/axum/), [PostgreSQL](https://www.postgresql.org/), and [Redis](https://redis.io/).

## Why axum?

Quite simple: It's ergonomic. And because it's built using Rust, a programming language that is safe by default. It's harder to introduce crashes, security vulnerabilities, or other unintended behavior in a Rust application. Aside from its safety features, Rust is insanely fast. Some would even say, ["Blazingly fast!"](https://i.redd.it/kczaqedt9ww81.jpg)

This repository aims to be a template for projects that want to utilize the technologies mentioned in the header. It also aims to be super speedy. In a single node and on a normal day, it should be able to handle a few hundred thousand concurrent requests as shown [here](https://web-frameworks-benchmark.netlify.app/compare?f=axum,laravel,fastify,fastapi) and [here](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md).

## Technologies

- [axum](https://github.com/tokio-rs/axum/): A Rust-based web framework that's simple to use and enables the creation of speedy and robust APIs and web applications.

- [PostgreSQL](https://www.postgresql.org/): A free and open-source relational database management system known for its reliability, feature-richness, and performance.

- [Redis](https://redis.io/): Redis is a fast, in-memory data store for managing high-speed transactions and real-time analytics. Versatile and scalable, it's perfect for tough data management challenges.

- [SeaORM](https://github.com/SeaQL/sea-orm/): An async and dynamic Object Realtional Mapper (ORM) for Rust.

## Setup

1. Install [cargo-watch](https://crates.io/crates/cargo-watch) and [sea-orm-cli](https://crates.io/crates/sea-orm-cli):

```sh
cargo install cargo-watch sea-orm-cli
```

2. `git clone` this repository.
3. `cd` into the project directory, `/axum-pg-redis`
4. Create your copy of `.env.example`, omit `.example`, and setup your database.
5. Use `sea-orm-cli migrate` to run migrations.

> Refer to the [SeaORM docs](https://www.sea-ql.org/SeaORM/docs/index/) for more information.

6. Use `cargo run` to install the project dependencies and to start the application.

## Project Structure

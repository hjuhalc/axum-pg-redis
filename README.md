# axum-pg-redis

A highly scalable backend template built with [axum](https://github.com/tokio-rs/axum/), [PostgreSQL](https://www.postgresql.org/), and [Redis](https://redis.io/).

## Why axum?

Quite simple: Because it's built using Rust, a programming language that is safe by default. It's harder to introduce crashes, security vulnerabilities, or other unintended behavior in a Rust application. Aside from its safety features, Rust is insanely fast. Some would even say, ["Blazingly fast!"](https://i.redd.it/kczaqedt9ww81.jpg)

This repository aims to be a template for projects that want to utilize the technologies mentioned in the header. It also aims to be super speedy. In a single node and on a normal day, it should be able to handle a few hundred thousand concurrent requests as shown [here](https://web-frameworks-benchmark.netlify.app/compare?f=axum,laravel,fastify,fastapi) and [here](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md).

## Technologies

- [axum](https://github.com/tokio-rs/axum/): A Rust-based web framework that's simple to use and enables the creation of speedy and robust APIs and web applications.

- [PostgreSQL](https://www.postgresql.org/): PostgreSQL is a free and open-source relational database management system known for its reliability, feature-richness, and performance.

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

## Maintenance

This section mainly discusses how we can maintain the stability and performance of the application in a single node. We'll only be going through the technologies mentioned above.

### Axum



Rust is hard to learn and axum is relatively easy to use. As much as possible, use built-in features and avoid reinventing the wheel.

### PostgreSQL

PostgreSQL can become pretty bloated over a period of time. It's recommended to remove some of the bloat and reclaim storage. A better solution is to prevent the problem in the first place. So to prevent bloat in a PostgreSQL database:

1. Use the `autovacuum` feature to automatically reclaim dead space. You may manually run `VACUUM` from time to time, but avoid using `VACUUM FULL` [unless you know what you're doing](https://www.postgresql.org/docs/current/sql-vacuum.html). You may also use tools like [pgcompacttable](https://github.com/dataegret/pgcompacttable) and [pg_repack](https://reorg.github.io/pg_repack/).
2. Enable [table partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html). You may also regularly truncate or drop large, unused tables.
2. Regularly analyze tables using the `ANALYZE` command.
3. Set `maintenance_work_mem` to a reasonable value for your system.
4. Use the `CLUSTER` command sparingly.
5. Use the right data types to avoid wasting space.

### Redis

### SeaORM

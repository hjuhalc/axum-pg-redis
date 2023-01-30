# axum-pg-redis

A highly scalable backend template built with [axum](https://github.com/tokio-rs/axum/), [PostgreSQL](https://www.postgresql.org/), and [Redis](https://redis.io/).

This repository aims to be a template for projects that want to utilize the technologies mentioned above. It also aims to be 
[blazingly fast](https://i.redd.it/t7ns9qtb5gh81.jpg). In a single node and on a normal day, it should be able to handle a few hundred thousand concurrent requests as shown [here](https://web-frameworks-benchmark.netlify.app/compare?f=axum,laravel,fastify,fastapi) and [here](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md).

## Technologies

- [axum](https://github.com/tokio-rs/axum/): A Rust-based web framework that's simple to use and enables the creation of speedy and robust APIs and web applications.

- [PostgreSQL](https://www.postgresql.org/): PostgreSQL is a free and open-source relational database management system known for its reliability, feature-richness, and performance.

- [Redis](https://redis.io/): Redis is a fast, in-memory data store for managing high-speed transactions and real-time analytics. Versatile and scalable, it's perfect for tough data management challenges.

-

## Features


## Setup

1. `git clone` this repository.
2. `cd` into the project directory, `/axum-pg-redis`.
3. run `cargo run` on your cli.

# axum-sdb-redis

A highly scalable backend built with [axum](https://github.com/tokio-rs/axum/), [SurrealDB](https://github.com/surrealdb/surrealdb/), and [Redis](https://redis.io/).

This repository aims to be a template for projects that want to utilize the technologies mentioned above. It also aims to be 
[blazingly fast](https://i.redd.it/t7ns9qtb5gh81.jpg). In a single node and on a normal day, it should be able to handle at least a hundred thousand concurrent requests as shown [here](https://web-frameworks-benchmark.netlify.app/compare?f=axum,laravel,fastify,fastapi) and [here](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md).

## Technologies:

- [axum](https://github.com/tokio-rs/axum/)
- [SurrealDB](https://github.com/surrealdb/surrealdb/)
- [Redis](https://redis.io/)
-

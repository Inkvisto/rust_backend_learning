
# Tokio tutorial

A brief description of what i learned


### Basic implementation / hello-redis.rs

- Client-server simple implementation

- Explanation why usage of async runtime and how async fn `main` works.

### Spawning / spawning.rs

- Concurrency vs Parallelism

- Tasks with `tokio::spawn`  & `JoinHandle` returned struct.

- `'static` and `Send` bound for safety.

### Shared state 
- Different ways to share state
- Using Mutex to implemet shared state 

### Channels / channels.rs
- Channel primitives
- Client-server with channels

### I/O
- Asynchronous I/O with `AsyncRead` and `AsyncWrite`
- Helper functions overwiew

### Framing
- `read_frame`, `parse_frame` & `Buf` trait

### Async in Depth
- Explanation of Futures, Wakers
Sources:
 https://marabos.nl/atomics/
 Asynchronous Programming
in Rust


### Select
- Async management with tokio::select!

### Streams

- Remark about how to implement Streams and what it is.

# queuer
Service to recive raw news information from individual parsers and put it to Redis stream for further processing. It handles `POST /submit` requests with a JSON like 

```rust
struct News {
    title: String,
    text: String,
    uri: String,
    posted: String,
    links: Vec<String>,
}
```

# GRPC Server for Rust
m
## Dev WorkFlow
1. <kbd>CMD</kbd> + <kbd> SHIFT </kbd> + <kbd> P </kbd>

2. Tasks: Run Task

3. type:
-  when changing on our src/ re-triggers cargo run
```sh
watch
```
or
- when changing on our protos/ re-triggers cargo build

```sh
proto build
```

4. create new .proto file on ./protos folder

5. add new module on ./src i.e. counter

```sh
counter
  -- counter_impl.rs
  -- mod.rs
```


6. add new service on main.rs

```rs
let addr = "127.0.0.1:50051".parse().unwrap();
let counter = MyCounter::new();
// <!--  add more here

Server::builder()
        .add_service(CounterServer::new(counter))
        // <!-- add more here
        .serve(addr)
        .await?;
```

7. Use postman
```
grpc://localhost:50051
```

8. upload proto file

9. pick an RPC method to trigger
set the message based on your proto request message
ie. counter.proto
```
message IncrementRequest {
    int32 value = 1;
}
```

we just need to pass
```
{
    "value": 10
}
```


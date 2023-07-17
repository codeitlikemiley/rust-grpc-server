# Dev WorkFlow

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

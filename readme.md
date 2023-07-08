# GRPC Server for Rust

## VsCode Snippets for GRPC services
Press <kbd>OPT</kbd> + <kbd>F1</kbd>
1. for creating .proto search: <kbd>proto</kbd>
2. for creating mod.rs search: <kbd>tonic_mod_rs</kbd>
3. for creating service_impl.rs search: <kbd>rust_tonic_impl_rs</kbd>

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

## use `Workspace` for Large MonoRepo 

you can either use a lib.rs or main.rs , and use the `-p ${workspace}` tag on the `cargo` command

ie: `cargo build -p server` would invoke the build.rs on server workspace

or `cargo run -p server` would invoke the main.rs on server workspace

### Creating new Workspace
run command: `cargo new --vcs none --lib ${workspace}` to generate shared library
run command: `cargo new --vcs none ${workspace}` to generate package

### Modify Cargo.toml
```rust
[workspace]

members = [
    "server",
]
```

## building whole packages in all workspace
run command: `cargo build` : this would invoke build in all packages type , install dependencies as well

## Production
We can deploy this in an AWS EC2 Instance with AWS Api Gateway + Route 53


## TODO
- Terraform config to quickly deploy on AWS

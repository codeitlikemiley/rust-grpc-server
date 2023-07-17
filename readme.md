# GRPC Server for Rust

[![Gitpod ready-to-code](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/codeitlikemiley/rust-grpc-server)

## Production Dockerfile

- `docker build -t codeitlikemiley/rust-grpc --build-arg DB_URL=postgres://username@localhost/dbname --build-arg GRPC_SERVER_ADDRESS=127.0.0.1:50051 .`

- `docker run -p 50051:50051 -e GRPC_SERVER_ADDRESS=127.0.0.1:50051 codeitlikemiley/rust-grpc`

## TODO
- add Dockerfile for development
    - add `protoc`
```sh
sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev
```

- add Dockerfile with Multi Stage build for production

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


10.  [Use with Evans CLI](https://github.com/ktr0731/evans#usage-cli)

- connect to your grpc server
```sh
evans -p 50051 -r
```

- list all packages

```sh
show package
```

- switch to specific package

```sh
package ${NAME_OF_PACKAGE}
```

- list all service of a package, note you need to be inside a package to use this
```sh
show service
```

- list all messages of package
```sh
show message
```
- describe a message
```sh
desc ${MESSAGE}
```

- switch to a service
```sh
service ${SERVICE_NAME}
```

- call an RPC
```sh
call ${RPC}
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

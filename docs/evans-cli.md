# [Use with Evans CLI](https://github.com/ktr0731/evans#usage-cli)

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

- Authentication with Bearer Token
```sh
header Authorization="Bearer some-secret-token"
header show
```

- call an RPC
```sh
call ${RPC}
```

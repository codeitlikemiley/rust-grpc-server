# Building Dockerfile for Production (multi stage build)

> We can use our Production `Dockerfile` and deploy this in AWS by uploading first our Docker Image on ECR , then Set up an ECS cluster: Amazon Elastic Container Service (ECS). Set up a Load Balancer with AWS Api Gateway, and configure domain name with Route 53.



1. Building Docker Image with/out ENV VAR

```sh
docker build -t codeitlikemiley/rust-grpc --build-arg DB_URL=postgres://username@localhost/dbname .
// note --build-arg is optional since we have default value when its not pass
```

2. Test it on you local machine

```sh
docker run -p 50051:50051 -e PRODUCTION=true codeitlikemiley/rust-grpc
```

3. Closing the Container
```sh
docker ps
// get the container i
// then run this command
docker stop ${container_id}
```


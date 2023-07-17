## pinned using timestamp to avoid rebuilding
## check here https://hub.docker.com/r/gitpod/workspace-full/tags
FROM gitpod/workspace-full:2023-07-13-03-31-28


## We need to install protoc so we can run cargo build
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
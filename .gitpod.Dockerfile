## pinned using timestamp to avoid rebuilding
## https://www.gitpod.io/docs/configure/workspaces/workspace-image#why-is-my-custom-dockerfile-rebuilding-everytime-even-with-no-change-made-to-it
## check here https://hub.docker.com/r/gitpod/workspace-full/tags
FROM gitpod/workspace-rust:latest


## We need to install protoc so we can run cargo build
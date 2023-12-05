#!/bin/sh

RESULTS_DIR="results/"

export GRPC_SERVER_CPUS=${GRPC_SERVER_CPUS:-"1"}
export GRPC_SERVER_RAM=${GRPC_SERVER_RAM:-"900m"}

docker run \
        --name "rust-grpc" \
		--rm \
		--cpus "1" \
		--memory "900m" \
        -e GRPC_SERVER_CPUS \
		-e GRPC_SERVER_RAM \
        -e PRODUCTION=true \
		-p 50051:50051 \
		--detach \
		--tty \
		codeitlikemiley/rust-grpc >/dev/null


ghz \
    --proto=./protos/auth.proto \
    --call=auth.Auth/Login \
    --concurrency="1000" \
    --connections="50" \
    --rps="0" \
    --duration "20s" \
    --data-file ./payload/payload \
    127.0.0.1:50051 >"${RESULTS_DIR}/rust-grpc".report

cat << EOF
done.
Results:
$(cat "${RESULTS_DIR}/rust-grpc".report | grep "Requests/sec" | sed -E 's/^ +/    /')
EOF

docker container stop rust-grpc >/dev/null
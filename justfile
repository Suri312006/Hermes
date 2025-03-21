
init:
    # TODO: figure this out later

sparta: 
    #!/usr/bin/env bash
    docker build -f sparta.Dockerfile -t sparta .
    nitro-cli build-enclave --docker-uri sparta --output-file sparta.eif
    nitro-cli run-enclave --eif-path sparta.eif --cpu-count 2 --memory 4096 --debug-mode --enclave-cid 16

trojan:
    docker build -f trojan.Dockerfile -t trojan .
    docker run -it -p 50051:50051 --device=/dev/vsock --security-opt seccomp=unconfined trojan

kill:
    nitro-cli terminate-enclave --all
    docker kill $(docker ps -q)


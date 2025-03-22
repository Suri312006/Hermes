
init:
    #!/usr/bin/env bash
    
    # install basic tools
    sudo yum install git docker make glibc-devel gcc patch -y
    
    # nitro-enclaves cli
    sudo amazon-linux-extras install aws-nitro-enclaves-cli -y

    #protobuf-compiler installation
    PB_REL="https://github.com/protocolbuffers/protobuf/releases"
    curl -LO $PB_REL/download/v25.1/protoc-25.1-linux-x86_64.zip
    unzip protoc-25.1-linux-x86_64.zip -d $HOME/.local
    export PATH="$PATH:$HOME/.local/bin"


sparta: 
    #!/usr/bin/env bash
    docker build -f sparta.Dockerfile -t sparta .
    nitro-cli build-enclave --docker-uri sparta --output-file sparta.eif
    nitro-cli run-enclave --eif-path sparta.eif --cpu-count 2 --memory 4096 --enclave-cid 16 
    sleep 5
    docker build -f trojan.Dockerfile -t trojan .
    docker run -it -p 50051:50051 --device=/dev/vsock --security-opt seccomp=unconfined trojan

kill:
    nitro-cli terminate-enclave --all
    docker kill $(docker ps -q)


bench:
    #!/usr/bin/env bash
    cd sator
    cargo bench

proxy *ARGS:
    #!/usr/bin/env bash
    cargo run --bin proxy -- {{ARGS}}

cli *ARGS:
    #!/usr/bin/env bash
    cargo run --bin cli -- {{ARGS}} 

FROM rust:1.81 as proof_api_builder

RUN apt-get update && apt-get install -y ca-certificates docker.io && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# proof_api のソースコードをコピー
COPY . .

RUN cargo build --release
RUN mkdir -p /root/.sp1/circuits/groth16/v4.0.0-rc.3 \
    && curl -L https://sp1-circuits.s3-us-east-2.amazonaws.com/v4.0.0-rc.3-groth16.tar.gz \
       | tar -xz -C /root/.sp1/circuits/groth16/v4.0.0-rc.3
# バイナリを実行
CMD ["./target/release/proof_api"]
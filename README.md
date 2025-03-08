# SP1 Game
## API Server
```
cargo build --release
mkdir -p /root/.sp1/circuits/groth16/v4.0.0-rc.3 \
    && curl -L https://sp1-circuits.s3-us-east-2.amazonaws.com/v4.0.0-rc.3-groth16.tar.gz \
       | tar -xz -C /root/.sp1/circuits/groth16/v4.0.0-rc.3
./target/release/proof_api
```

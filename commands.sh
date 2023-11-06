/usr/bin/docker run --userns host -e 'PKG_CONFIG_ALLOW_CROSS=1' -e 'XARGO_HOME=/xargo' -e 'CARGO_HOME=/cargo' -e 'CARGO_TARGET_DIR=/target' -e 'CROSS_RUNNER=' -e TERM -e 'USER=root' --rm --user 0:0 -v /root/.xargo:/xargo:z -v /root/.cargo:/cargo:z -v /cargo/bin -v /root/spin:/project:z -v /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu:/rust:z,ro -v /root/spin/target:/target:z -w /project -i -t cross-custom-spin:x86_64-unknown-linux-musl-bab7b

sh -c 'PATH=$PATH:/rust/bin cargo build --verbose --release --target x86_64-unknown-linux-musl --features openssl/vendored'


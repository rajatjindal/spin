## longevity-apps-test

longevity test is to ensure `wasm` file(s) compiled with a version of `Spin` continues to work with runtime of future version of `Spin`. 

The current wasm files are created using following templates with `Spin v0.9.0`

- http-go
- http-rust
- http-js
- http-ts

The `wasm` files are built using `spin build` and copied over here for validation.
# Carnival

An experimental Replit Games Viewer.
The backend (GQL API written in rust) for the replit community project - Carnival!

---

Some commands to get started...

1. Cloning the GitHub respository
```shell
git clone https://github.com/TheDrone7/carnival-backend.git carnival/backend
```

2. Building the executable
```shell
cargo build
```

3. Running the built executable (also builds in case not already built)
```shell
cargo run
```

4. Formatting the source code
```shell
rustfmt --color always --verbose --edition 2021 src/main.rs
```

5. Linting the source code.
```shell
cargo clippy
```

6. Adding release target
```shell
rustup target add x86_64-unknown-linux-musl
```

7. Building for release
```shell
cargo build --release --target=x86_64-unknown-linux-musl
```

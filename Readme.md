# Tutorial

### Prerequisites

#### After installing Rust, you can use Cargo to install trunk by running:
```bash
cargo install trunk
```
#### We will also need to add the WASM build target by running:
```bash
rustup target add wasm32-unknown-unknown
```
### Setting up the project

#### First, create a new cargo project:
```bash
cargo new yew-app
cd yew-app
```
***To verify the Rust environment is set up properly, run the initial project using the cargo build tool. After the output about the build process, you should see the expected "Hello, world!" message.***
```bash
cargo run
```
## Start the development server
```bash
trunk serve
```
***[Перевірка версій пакетів Link ](https://crates.io/search?q=yew)***
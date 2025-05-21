# WireChat

WireChat is a secure chat application that uses WireGuard for encrypted communication.

## Features
- **Secure Communication**: All messages are encrypted using WireGuard.
- **WebSocket Support**: Real-time messaging between front and back using WebSockets.
- **API**: Simple HTTP API for sending and receiving messages between peers' backends.
- **Vue.js Frontend**: A lightweight frontend built with Vue.js for easy interaction.

## Getting Started
### Prerequisites
- Rust (latest stable version)
- Node.js and npm (for the frontend)
```
cd front
npm install
```
- libsqlite3-dev (for SQLite database support)
```
sudo apt install libsqlite3-dev
```
- diesel_cli (to initialize the database)
```
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run --database-url=.db.sqlite 
```
or
```
diesel migration redo --database-url=.db.sqlite
```

### Installation
1. Clone the repository:
   ```bash
    git clone https://github.com/loouis-t/WireChat.git
    cd WireChat/front
    ```
2. Build the Application:
   ```bash
    npm run dev
    ```

## Architecture

```
WireChat/
├── front/
│   ├── pkg/
│   ├── public/
│   ├── src/
│   │   ├── assets/
│   │   ├── components/
│   │   ├── router/
│   │   ├── views/
│   │   ├── App.vue
│   │   ├── main.ts
│   ├── index.html
│
├── migrations/
│
├── pkg/
│
├── src/
│   ├── client/
│   ├── database/
│   ├── server/
│   ├── wireguard/
│   ├── main.rs
│
├── Cargo.toml

```
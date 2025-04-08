# WireChat

```
peer_app/
├── Cargo.toml               # Contains dependencies for tokio, wireguard‑rs, actix-web/warp, etc.
├── src
│   ├── main.rs              # Entry point; starts both WireGuard and local API server
│   ├── wireguard_mod.rs     # WireGuard configuration and peer management module
│   ├── messaging.rs         # Application‑level messaging over the secure tunnel
│   └── api.rs               # HTTP/WebSocket API server exposing status and control endpoints
└── frontend/
├── package.json         # Vue.js project config
├── src
│   ├── main.js         # Vue entrypoint
│   ├── App.vue         # Main Vue component
│   └── components      # UI components (e.g., connection status, chat box)
└── public               # Static assets
```

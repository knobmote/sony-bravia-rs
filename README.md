# SONY BRAVIA TV REST API

The `sony-bravia-api` crate defines Sony REST API commands and their typed results.
Commands are grouped by service: `guide`, `system`, `audio`, `av_content`,
`app_control`, `encryption`, `video`, and `video_screen`.

The `sony-bravia-client` crate provides a sans-I/O client. It prepares the
request path and JSON body, assigns request IDs, and decodes responses into the
command's result type. The caller supplies the HTTP transport, TV address,
authentication, and timeouts:

```rust
use sony_bravia_api::system::GetPowerStatus;
use sony_bravia_client::Client;

let mut client = Client::new();
let request = client.prepare(GetPowerStatus)?;
// POST request.body() to the TV's base URL + request.path(), with the
// appropriate authentication and Content-Type: application/json.
// Supply the actual HTTP status and response body after the request completes.
let status = 200;
let body = br#"{"id":1,"result":[{"status":"active"}]}"#;
let power = request.complete(status, body)?;
# Ok::<(), sony_bravia_client::Error>(())
```

Each pending request retains its command type and checks the response ID. IDs
are unique within a client instance and are not reused after exhaustion; keep
pending requests associated with the client/connection that prepared them.

Both crates support `no_std` and require `alloc`. Embedded applications must
provide a global allocator; standard Rust applications can use the same crates
without additional configuration. Response bodies and parsed values allocate,
so transports should limit response sizes to fit the device's memory.

To check the libraries for the ESP32-S31 bare-metal target:

```sh
rustup target add riscv32imafc-unknown-none-elf
cargo check --workspace --target riscv32imafc-unknown-none-elf
```

Support varies by display, firmware, region, and authentication level. Some
professional-display APIs are localhost-only. The command definitions do not yet
cover every method or version in Sony's reference.

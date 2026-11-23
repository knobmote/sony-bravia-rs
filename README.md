# SONY BRAVIA TV REST API

See [API coverage](API_COVERAGE.md) for the documented methods, implemented
versions, authentication levels, model restrictions, and observed TV results.

The `sony-bravia-api` crate defines Sony REST API commands and their typed results.
Commands are grouped by service: `access_control`, `app_control`, `audio`,
`av_content`, `browser`, `cec`, `encryption`, `guide`, `system`, `video`, and
`video_screen`. The `discovery` module provides common service methods.

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

Each pending request retains its command type and checks the response ID.
IDs advance from 1 through `i32::MAX` and then wrap to 1. Keep pending
requests associated with the connection that prepared them and retire each
request before its ID can be reused.

The API and sans-I/O crates support `no_std` and require `alloc`. Embedded applications must
provide a global allocator; standard Rust applications can use the same crates
without additional configuration. Response bodies and parsed values allocate,
so transports should limit response sizes to fit the device's memory.

To check the libraries for the ESP32-S31 bare-metal target:

```sh
rustup target add riscv32imafc-unknown-none-elf
cargo +stable check -p sony-bravia-reqwless --target riscv32imafc-unknown-none-elf
```

## HTTP client for a computer

`sony-bravia-reqwest` provides an async, `std`-based HTTP client over the
sans-I/O client. It works with any command defined by the API crate:

```rust,no_run
use sony_bravia_api::system::GetPowerStatus;
use sony_bravia_reqwest::Client;

# async fn example() -> Result<(), sony_bravia_reqwest::Error> {
let mut tv = Client::new("http://192.168.1.100", Some("your-pre-shared-key"))?;
let power = tv.execute(GetPowerStatus).await?;
println!("{power:?}");
# Ok(())
# }
```

The client executes one request at a time, uses a ten-second timeout, and
limits response bodies to 1 MiB by default; `set_max_response_bytes` changes
the limit. It disables redirects and proxies. This implementation currently
supports HTTP and optional pre-shared-key authentication; other authentication
methods are not implemented. It requires a Tokio runtime.

## HTTP client for embedded devices

`sony-bravia-reqwless` provides the same typed command execution for
`no_std` applications with a global allocator. Supply an asynchronous
connection that implements `embedded_io_async::Read` and
`embedded_io_async::Write`, and apply a deadline to the whole operation.

```rust,no_run
use core::future::ready;

use embedded_io_async as io;

use sony_bravia_api as api;
use sony_bravia_reqwless as bravia;

async fn read_power(
    stream: impl io::Read + io::Write,
) -> Result<api::system::PowerStatus, bravia::Error> {
    let mut tv = bravia::Client::new();
    tv.execute(
        api::system::GetPowerStatus,
        ready(Ok(stream)),
        "192.168.1.100",
        Some("your-pre-shared-key"),
    )
    .await
}
```

You can also pass a future that opens the connection. The client prepares the
request before polling it. Each exchange uses plain HTTP, sends
`Connection: close`, and uses fixed request, header, and response buffers.
Retain the client between commands to keep request IDs advancing.

## Check a real TV from a computer

Enable IP control on the TV and, if required, configure a pre-shared key in its
network settings. Run this read-only power-status example on the same network,
substituting the TV's address and key:

```sh
export BRAVIA_PSK='your-pre-shared-key'
cargo run -p sony-bravia-reqwest --example tv_power_status -- http://192.168.1.100
```

Omit `BRAVIA_PSK` if the TV does not require a key. The example uses plain HTTP,
so the key travels over the network unencrypted; use it on a trusted network.
It reports transport, HTTP, Sony API, and response decoding errors. The host-only
HTTP dependency is confined to the `sony-bravia-reqwest` crate.

The host remote example supports state inspection and explicit control:

```sh
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 power
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 audio
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 audio-output
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 inputs
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 mute on
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 volume speaker 20
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 volume-step speaker +1
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 power on
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 input 'extInput:hdmi?port=1'
```

Use `mute off`, `power off`, or `volume headphone LEVEL` as needed.
`audio-output` reads the active terminal where supported. `volume-step`
sends a signed adjustment such as `+1` or `-1`. The effective volume change
depends on the TV and the selected audio output. `playing`
and `wol` read the current content and Wake-on-LAN setting where supported. The
input command accepts a URI reported by `inputs`; availability depends on the TV.

To list launchable apps and start one by its reported URI:

```sh
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 apps
cargo run -p sony-bravia-reqwest --example tv_remote -- http://192.168.1.100 app 'APP_URI'
```

`app-status` displays the TV's application status values; those values do not
necessarily identify the currently displayed app.

For a wider read-only compatibility check, run:

```sh
cargo run -p sony-bravia-reqwest --example tv_validate -- http://192.168.1.100
```

It reports each command's result without printing the returned device data, and
exits unsuccessfully if any command fails. A failure may indicate an unsupported
method, missing authentication, or the TV's current state as well as a client
or response decoding problem.

Support varies by display, firmware, region, and authentication level. Some
professional-display APIs are localhost-only. The command definitions do not yet
cover every method or version in Sony's reference.

## GUITION board development

[Knobmote](https://github.com/knobmote/knobmote) uses these crates for its
ESP32-S31 remote firmware.

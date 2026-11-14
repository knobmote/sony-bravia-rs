# Discovered BRAVIA methods

This document distinguishes the TV's advertised method signatures from
independently published material and from live-tested behavior. The
[historical sweep](TV_API_SWEEP.md) records the 104 method/version combinations
advertised by one TV on 2026-11-14; models and firmware can differ.

## External information

- Sony's [BRAVIA IP control manual](https://pro.sony/s3/cms-static-content/file/25/1237492002825.pdf)
  describes `getVersions` and `getMethodTypes` as common service methods and
  documents filtering the latter by version. An empty filter was observed to
  return all methods on this TV.
- Sony's [REST API reference](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/)
  documents the 1.1 text-form methods, including optional encryption, but its
  duplicate `getTextForm` entries contain conflicting guidance about whether
  `encKey` may be omitted. The commands here always send the field; encryption
  and decryption are the caller's responsibility.
- Community implementations document a two-parameter `actRegister` request and
  a PIN/cookie pairing flow; see the [registration example](https://github.com/breunigs/bravia-auth-and-remote/blob/master/auth_cookie_examples/auth.sh)
  and [recorded successful response](https://github.com/home-assistant/core/issues/82403).
  These are implementation reports, not a Sony specification for every model.
- A [published discovery trace](https://community.openhab.org/t/sony-devices-binding/14052/274)
  shows the `internetTVMode` field within multi-screen options. The same field
  was observed in this TV's read-only response.
- A [community integration](https://github.com/cmos486/Bravia-REST-API/releases)
  identifies `outputTerminal` as a sound-setting target. A read-only
  `getSoundSettings` v1.1 request for that target succeeded on the TV and
  reported `audioSystem`; available values can vary by device.

## Library support

All 104 advertised combinations have command definitions, including
`GetVersions<S>` and `GetMethodTypes<S>` for each advertised service. For
example, a host client can query audio signatures with:

```rust,no_run
use sony_bravia_api::discovery::GetMethodTypes;
use sony_bravia_api::discovery::service;
use sony_bravia_reqwest::Client;

# async fn example() -> Result<(), sony_bravia_reqwest::Error> {
let mut tv = Client::new("http://192.168.1.100", None)?;
let methods = tv.execute(GetMethodTypes::<service::Audio>::new("")).await?;
println!("{methods:?}");
# Ok(())
# }
```

The library supports the observed `results` envelope as well as `result`.
Discovery, content-favorite sources, sound settings, external-input status
v1.1, multi-screen getters, parental-rating settings, and the content
list/count versions were validated through the typed client on the TV. Other
new commands, particularly state-changing ones, are defined from the TV's
signatures and have **not** been live-tested. A signature alone does not reveal
all required fields, accepted values, side effects, or model restrictions.

`SetPlayTvContentV1_1::channel` remains an opaque JSON value because the TV
advertised `PlayTvChannel` without describing its fields. Likewise, banner
candidate details are kept as JSON values. These are intentionally provisional
until supported shapes can be verified; the rest of the command is typed.

`ActRegister` emits the observed two positional parameters, but a complete
PIN/cookie registration flow is **not implemented**. A transport must handle
the authentication challenge and response cookie; the current reqwest client
uses a pre-shared key and does not expose that pairing flow. Registration was
not sent to the TV during validation.

The API and sans-I/O crates remain `no_std + alloc`. The reqwest client is a
separate host-side transport. See the [coverage matrix](../API_COVERAGE.md)
for methods, implementation links, authentication, and per-TV test status.

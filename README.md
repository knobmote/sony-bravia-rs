# SONY BRAVIA TV REST API

The `sony-bravia-api` crate defines Sony REST API commands and their typed results.
Commands are grouped by service: `guide`, `system`, `audio`, `av_content`,
`app_control`, `encryption`, `video`, and `video_screen`.

The crate currently provides command definitions and JSON encoding/decoding; it
does not yet send requests to a display. For example, a transport can build a
request from `audio::SetAudioMute { status: true }`, send it to
`/sony/audio`, and decode the response using the same command type. The
transport is responsible for assigning request IDs and matching responses.

Support varies by display, firmware, region, and authentication level. Some
professional-display APIs are localhost-only. The command definitions do not yet
cover every method or version in Sony's reference.

# BRAVIA REST API coverage

Snapshot: 2026-11-14. Source: [Sony BRAVIA Professional Displays REST API reference](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/). The source targets professional displays; support and authentication can differ on consumer models.

This is a method/version inventory, not a guarantee that every field, parameter, response shape, or error has been fully validated. Some device capabilities are absent from Sony’s published professional-display reference. The TV discovery snapshot below comes from `guide/getSupportedApiInfo` on the user’s TV and is specific to that device and firmware.

The reference contains 59 distinct service/method/version combinations; this repository defines typed commands for 45. The TV advertised 104 combinations, including 57 outside that reference snapshot. Those counts describe this snapshot, not a guarantee of implementation correctness or interoperability.

## Reading the tables

- **Implementation** links the typed command in this repository; it does not imply full schema coverage.
- **Reference auth** is Sony’s documented level. `none`, `generic`, and `private` are API access levels, not directly the names of authentication methods.
- **TV advertised** is the discovery response for the exact service/method/version, with its reported authentication level; `—` means not advertised in this snapshot, not conclusive proof of absence. A reported level of `none` includes omitted `authLevel`, per Sony’s documented default.
- **Live result** is a test on this TV. `Pass` means the typed client processed a successful request and response; state-changing passes are described below. `Raw discovery` means the raw method was queried outside the typed client. `Not tested` includes methods that are advertised but have not been exercised. The three error results are device responses, not decoder failures.
- All methods remain subject to device, firmware, region, power-state and authentication differences. The notes below flag specific documented restrictions; they are not an exhaustive compatibility guarantee.

## Published methods

### guide

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`getSupportedApiInfo`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1664) | 1.0 | none | [`GetSupportedApiInfo`](api/src/guide.rs) | Yes (none) | Raw discovery |

### appControl

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`getApplicationList`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1774) | 1.0 | private | [`GetApplicationList`](api/src/app_control.rs) | Yes (private) | Pass |
| [`getApplicationStatusList`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1809) | 1.0 | none | [`GetApplicationStatusList`](api/src/app_control.rs) | Yes (none) | Pass |
| [`getTextForm`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1843) | 1.1 | private | Not implemented | Yes (private) | Not tested |
| [`getWebAppStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1856) | 1.0 | private | [`GetWebAppStatus`](api/src/app_control.rs) | Yes (private) | Not tested |
| [`installApp`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-13051) | 1.0 | generic | Not implemented | — | Not tested |
| [`prepareAppUpload`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-13043) | 1.0 | generic | Not implemented | — | Not tested |
| [`setActiveApp`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2044) | 1.0 | generic | [`SetActiveApp`](api/src/app_control.rs) | Yes (generic) | Pass |
| [`setTextForm`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2219) | 1.0 | generic | [`SetTextForm`](api/src/app_control.rs) | Yes (generic) | Not tested |
| [`setTextForm`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2225) | 1.1 | generic | Not implemented | Yes (generic) | Not tested |
| [`terminateApps`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2216) | 1.0 | generic | [`TerminateApps`](api/src/app_control.rs) | Yes (generic) | Not tested |
| [`uninstallApp`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-13072) | 1.0 | generic | Not implemented | — | Not tested |

### audio

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`setAudioVolume`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2124) | 1.0 | generic | [`SetAudioVolume`](api/src/audio.rs) | Yes (generic) | Pass |
| [`getSpeakerSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2138) | 1.0 | none | [`GetSpeakerSettings`](api/src/audio.rs) | Yes (none) | Pass |
| [`getVolumeInformation`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2134) | 1.0 | none | [`GetVolumeInformation`](api/src/audio.rs) | Yes (none) | Pass |
| [`setAudioMute`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2130) | 1.0 | generic | [`SetAudioMute`](api/src/audio.rs) | Yes (generic) | Pass |
| [`setAudioVolume`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2119) | 1.2 | generic | [`SetAudioVolumeV1_2`](api/src/audio.rs) | Yes (generic) | Not tested |
| [`setSoundSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2113) | 1.1 | generic | [`SetSoundSettings`](api/src/audio.rs) | Yes (generic) | Not tested |
| [`setSpeakerSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2105) | 1.0 | generic | [`SetSpeakerSettings`](api/src/audio.rs) | Yes (generic) | Not tested |

### avContent

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`setPlayContent`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2055) | 1.0 | generic | [`SetPlayContent`](api/src/av_content.rs) | Yes (generic) | Not tested |
| [`getCurrentExternalInputsStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2064) | 1.1 | none | Not implemented | Yes (none) | Not tested |
| [`getCurrentExternalInputsStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2078) | 1.0 | none | [`GetCurrentExternalInputsStatus`](api/src/av_content.rs) | Yes (none) | Pass |
| [`getPlayingContentInfo`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2060) | 1.0 | private | [`GetPlayingContentInfo`](api/src/av_content.rs) | Yes (private) | IllegalState |
| [`getContentCount`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2099) | 1.0 | private | [`GetContentCount`](api/src/av_content.rs) | Yes (private) | Not tested |
| [`getContentCount`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2093) | 1.1 | private | Not implemented | Yes (private) | Not tested |
| [`getContentList`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2083) | 1.5 | private | [`GetContentList`](api/src/av_content.rs) | Yes (private) | Not tested |
| [`getSchemeList`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2074) | 1.0 | none | [`GetSchemeList`](api/src/av_content.rs) | Yes (none) | Pass |
| [`getSourceList`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2069) | 1.0 | none | [`GetSourceList`](api/src/av_content.rs) | Yes (none) | Not tested |

### encryption

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`getPublicKey`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2049) | 1.0 | none | [`GetPublicKey`](api/src/encryption.rs) | Yes (none) | Not tested |

### system

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`getScreenshot`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-12980) | 1.0 | private | Not implemented | — | Not tested |
| [`setPowerSavingMode`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2230) | 1.0 | generic | [`SetPowerSavingMode`](api/src/system.rs) | Yes (generic) | Not tested |
| [`getCurrentTime`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1951) | 1.0 | none | [`GetCurrentTime`](api/src/system.rs) | Yes (none) | Pass |
| [`getCurrentTime`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1956) | 1.1 | none | [`GetCurrentTimeV1_1`](api/src/system.rs) | Yes (none) | Pass |
| [`getNetworkSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2033) | 1.0 | generic | [`GetNetworkSettings`](api/src/system.rs) | Yes (generic) | Not tested |
| [`getInterfaceInformation`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1961) | 1.0 | none | [`GetInterfaceInformation`](api/src/system.rs) | Yes (none) | Pass |
| [`getRemoteDeviceSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2013) | 1.0 | none | [`GetRemoteDeviceSettings`](api/src/system.rs) | Yes (none) | Not tested |
| [`getLEDIndicatorStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2039) | 1.0 | generic | [`GetLedIndicatorStatus`](api/src/system.rs) | Yes (generic) | Pass |
| [`getPowerSavingMode`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2030) | 1.0 | none | [`GetPowerSavingMode`](api/src/system.rs) | Yes (none) | Pass |
| [`getPowerStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2026) | 1.0 | none | [`GetPowerStatus`](api/src/system.rs) | Yes (none) | Pass |
| [`getRemoteControllerInfo`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2020) | 1.0 | none | [`GetRemoteControllerInfo`](api/src/system.rs) | Yes (none) | Pass |
| [`getSystemInformation`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-2004) | 1.0 | private | [`GetSystemInformation`](api/src/system.rs) | Yes (private) | Pass |
| [`getSystemInformation`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1998) | 1.7 | private | Not implemented | — | Not tested |
| [`getSystemSupportedFunction`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1993) | 1.0 | none | [`GetSystemSupportedFunction`](api/src/system.rs) | Yes (none) | Pass |
| [`getSettingsSchema`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-74573) | 1.0 | generic | Not implemented | — | Not tested |
| [`getSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-74578) | 1.0 | generic | Not implemented | — | Not tested |
| [`setSettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-74581) | 1.0 | generic | Not implemented | — | Not tested |
| [`getWolMode`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1990) | 1.0 | generic | [`GetWolMode`](api/src/system.rs) | Yes (generic) | Pass |
| [`requestReboot`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1987) | 1.0 | generic | [`RequestReboot`](api/src/system.rs) | Yes (generic) | Not tested |
| [`setLEDIndicatorStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1982) | 1.1 | generic | [`SetLedIndicatorStatus`](api/src/system.rs) | Yes (generic) | Not tested |
| [`setPowerStatus`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1978) | 1.0 | generic | [`SetPowerStatus`](api/src/system.rs) | Yes (generic) | Pass |
| [`setWolMode`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1970) | 1.0 | generic | [`SetWolMode`](api/src/system.rs) | Yes (generic) | Not tested |

### video

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`getScreenRotation`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-13035) | 1.0 | none | [`GetScreenRotation`](api/src/video.rs) | — | NoSuchMethod |
| [`setScreenRotation`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-13038) | 1.0 | generic | [`SetScreenRotation`](api/src/video.rs) | — | Not tested |
| [`getPictureQualitySettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1930) | 1.0 | none | [`GetPictureQualitySettings`](api/src/video.rs) | Yes (none) | Not tested |
| [`getPictureQualitySettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-36021) | 1.1 | none | Not implemented | — | Not tested |
| [`setPictureQualitySettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1946) | 1.0 | generic | [`SetPictureQualitySettings`](api/src/video.rs) | Yes (generic) | Not tested |
| [`setPictureQualitySettings`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-36022) | 1.1 | generic | Not implemented | — | Not tested |

### videoScreen

| Method | Version | Reference auth | Implementation | TV advertised (auth) | Live result |
| --- | --- | --- | --- | --- | --- |
| [`getSceneSetting`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-35627) | 1.0 | none | [`GetSceneSetting`](api/src/video_screen.rs) | Yes (none) | IllegalState |
| [`setSceneSetting`](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-1919) | 1.0 | generic | [`SetSceneSetting`](api/src/video_screen.rs) | Yes (generic) | Not tested |

## Specific restrictions and reference caveats

- `appControl/installApp`, `prepareAppUpload`, and `uninstallApp`: Sony describes these as localhost-only on supported FW-BZxxx models, subject to firmware/generation requirements.
- `system/getScreenshot`: localhost-only; Sony excludes BZ40P/BZ35P/BZ30P and lists FW-BZxxx firmware/generation requirements for other supported models.
- `system/getSettingsSchema`, `getSettings`, and `setSettings`: localhost-only on supported FW-BZxxxP firmware/generation versions.
- `video/getScreenRotation` and `setScreenRotation`: Sony describes these as localhost-only on supported FW-BZxxx firmware/generation versions.
- `audio/getSpeakerSettings` and `setSpeakerSettings`, and `videoScreen/getSceneSetting` and `setSceneSetting`: Sony says BZ40P/BZ35P/BZ30P are unsupported.
- Sony documents regional restrictions for `appControl/setTextForm` and one of its two `getTextForm` v1.1 entries. The `getTextForm` entries describe `encKey` as optional but also say it must always be set; the table groups them by method/version and links the first entry. Consult [the second entry](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/#article-35945) before implementing.
- Authentication in Sony’s reference is not identical to every practical access policy; for example, the TV returned HTTP 403 for some private or generic calls without the pre-shared key.

## Real-TV validation detail

- Authenticated read-only sweep: 16/19 succeeded. `getPlayingContentInfo` and `getSceneSetting` returned `IllegalState`; `getScreenRotation` returned `NoSuchMethod`.
- Authenticated state-changing checks: mute on/off, volume 22 → 21 → 22, and power standby → active succeeded; audio was restored. Two app activations (Netflix and YouTube) were visually confirmed.
- Input enumeration succeeded; input switching was not tried because the original input could not be reliably determined for restoration. No prolonged deep-standby or Wake-on-LAN packet test was performed.
- TV app icons are served over HTTP; sample downloads succeeded without authentication. The observed icon `Content-Type` header was malformed, despite valid PNG data.

## Additional methods advertised by this TV

These exact service/method/version combinations were advertised by the TV but do not appear in the reference snapshot above. Their authentication levels below are from TV discovery, not the published reference. Except for the raw `appControl/getMethodTypes` discovery call, they have not been live-tested; names alone do not establish their parameters or behavior.


### accessControl

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `actRegister` | 1.0 | none | Not implemented |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |

### appControl

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |

### audio

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getSoundSettings` | 1.1 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |

### avContent

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `deleteContent` | 1.0 | generic | Not implemented |
| `getContentList` | 1.0 | private | Not implemented |
| `getContentList` | 1.2 | private | Not implemented |
| `getFavoriteList` | 1.0 | private | Not implemented |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getParentalRatingSettings` | 1.0 | generic | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |
| `setDeleteProtection` | 1.0 | generic | Not implemented |
| `setFavoriteContentList` | 1.0 | generic | Not implemented |
| `setPlayTvContent` | 1.0 | generic | Not implemented |
| `setPlayTvContent` | 1.1 | generic | Not implemented |
| `setTvContentVisibility` | 1.0 | generic | Not implemented |

### browser

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `actBrowserControl` | 1.0 | generic | Not implemented |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getTextUrl` | 1.0 | private | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |
| `setTextUrl` | 1.0 | generic | Not implemented |

### cec

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |
| `setCecControlMode` | 1.0 | generic | Not implemented |
| `setMhlAutoInputChangeMode` | 1.0 | generic | Not implemented |
| `setMhlPowerFeedMode` | 1.0 | generic | Not implemented |
| `setPowerSyncMode` | 1.0 | generic | Not implemented |

### encryption

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |

### guide

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getServiceProtocols` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |

### system

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getDeviceMode` | 1.0 | generic | Not implemented |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getStorageList` | 1.1 | generic | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |
| `mountStorage` | 1.0 | generic | Not implemented |
| `setDeviceMode` | 1.0 | generic | Not implemented |
| `setLanguage` | 1.0 | generic | Not implemented |

### video

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |

### videoScreen

| Method | Version | TV auth | Implementation |
| --- | --- | --- | --- |
| `getAudioSourceScreen` | 1.0 | none | Not implemented |
| `getBannerMode` | 1.0 | none | Not implemented |
| `getMethodTypes` | 1.0 | none | Not implemented |
| `getMultiScreenMode` | 1.1 | none | Not implemented |
| `getPipSubScreenPosition` | 1.0 | none | Not implemented |
| `getVersions` | 1.0 | none | Not implemented |
| `requestToNotifyScreenState` | 1.1 | none | Not implemented |
| `setAudioSourceScreen` | 1.0 | generic | Not implemented |
| `setBannerMode` | 1.0 | generic | Not implemented |
| `setMultiScreenMode` | 1.0 | generic | Not implemented |
| `setPapScreenSize` | 1.0 | generic | Not implemented |
| `setPipSubScreenPosition` | 1.0 | generic | Not implemented |

The TV advertises `avContent/getFavoriteList`, but its name alone does not establish that it contains app favorites; it was not called in this audit.

## Scope and maintenance

The matrix covers the current REST reference and this TV’s advertised REST methods. It does not cover the separate IRCC-IP, Simple IP Control, or Cloud APIs, nor does it claim complete error-code coverage. Recheck the reference, source command definitions, TV capability discovery, and live results after firmware or library changes.

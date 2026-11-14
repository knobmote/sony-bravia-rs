# Real-TV REST API sweep

Snapshot: 2026-11-14. This is one TV and firmware, not a claim about all BRAVIA devices. The sweep used the pre-shared key; it did not modify the TV or retain response values, credentials, or app/content titles.

All 11 advertised services answered `getMethodTypes` and `getVersions`. The 104 advertised service/method/version combinations exactly matched the method signatures returned by those services. The sanitized [machine-readable signatures](tv-api-signatures.json) preserve the TV’s parameter and result descriptions, which are type hints rather than a complete validation schema.

The current authenticated typed validator passed 25 of 28 commands. The three failures were `IllegalState` for playing content and scene settings, and `NoSuchMethod` for screen rotation (not advertised). Successful raw calls are not proof that the Rust decoder accepts the response.

Every listed state-changing, registration, deletion, reboot, and notification-request method was left untouched in this sweep. Four controls had been separately verified in earlier, explicitly authorized live tests. The browser URL and text-form getters were not read to avoid collecting on-screen data.

Where applicable, GET methods were called with an empty/default filter. Content counts and lists were tested with the first non-empty external-input source; lists were limited to one item. A rejected filter or state error does not prove the API is unsupported.

## Findings

- Application inventory and launching work through the typed client; icons are separate HTTP resources. The TV’s icon responses contained valid PNG data with a malformed `Content-Type` header.
- `avContent/getFavoriteList` returned four `fav` sources. A one-item page from each was empty; this does not demonstrate an app-favorites list.
- Raw audio sound-settings, external-input status v1.1, parental-rating settings, picture settings, network settings, and several multi-screen getters returned data. Successful typed operations are marked separately below.
- `system/getDeviceMode` rejected the empty filter with error 3; a valid value has not been established. `system/getStorageList`, `avContent/getPlayingContentInfo`, and `videoScreen/getSceneSetting` returned error 7 in the current state.
- `getMethodTypes` and `guide/getServiceProtocols` return a `results` field on this TV; the current standard response decoder expects `result`. Their successful raw calls therefore do not establish support in the typed client.
- `getContentList` 1.0, 1.2, and 1.5 and `getContentCount` 1.0 and 1.1 returned data for an external-input source. The existing typed v1.0 count and v1.5 list commands also passed.

## Method-by-method results

Authentication levels are TV-advertised values; an omitted `authLevel` is recorded as `none` using Sony’s documented default. The complete input/output signature for each exact method/version is in the JSON snapshot.

### accessControl

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `actRegister` | 1.0 | none | Not invoked: may change state |

### appControl

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `getApplicationStatusList` | 1.0 | none | Typed pass |
| `terminateApps` | 1.0 | generic | Not invoked: may change state |
| `getTextForm` | 1.1 | private | Skipped: private text/URL |
| `setActiveApp` | 1.0 | generic | Previously verified; not invoked in sweep |
| `setTextForm` | 1.0 | generic | Not invoked: may change state |
| `setTextForm` | 1.1 | generic | Not invoked: may change state |
| `getWebAppStatus` | 1.0 | private | Typed pass |
| `getApplicationList` | 1.0 | private | Typed pass |

### audio

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `setAudioVolume` | 1.0 | generic | Previously verified; not invoked in sweep |
| `setAudioVolume` | 1.2 | generic | Not invoked: may change state |
| `getVolumeInformation` | 1.0 | none | Typed pass |
| `getSoundSettings` | 1.1 | none | Raw pass |
| `setSoundSettings` | 1.1 | generic | Not invoked: may change state |
| `setAudioMute` | 1.0 | generic | Previously verified; not invoked in sweep |
| `setSpeakerSettings` | 1.0 | generic | Not invoked: may change state |
| `getSpeakerSettings` | 1.0 | none | Typed pass |

### avContent

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `getContentList` | 1.0 | private | Raw pass |
| `getContentList` | 1.2 | private | Raw pass |
| `getContentList` | 1.5 | private | Typed pass |
| `getSchemeList` | 1.0 | none | Typed pass |
| `getCurrentExternalInputsStatus` | 1.0 | none | Typed pass |
| `getCurrentExternalInputsStatus` | 1.1 | none | Raw pass |
| `setPlayTvContent` | 1.0 | generic | Not invoked: may change state |
| `setPlayTvContent` | 1.1 | generic | Not invoked: may change state |
| `setDeleteProtection` | 1.0 | generic | Not invoked: may change state |
| `getPlayingContentInfo` | 1.0 | private | TV error 7: Illegal State |
| `deleteContent` | 1.0 | generic | Not invoked: may change state |
| `getFavoriteList` | 1.0 | private | Raw pass |
| `getSourceList` | 1.0 | none | Typed pass |
| `setPlayContent` | 1.0 | generic | Not invoked: may change state |
| `setTvContentVisibility` | 1.0 | generic | Not invoked: may change state |
| `getContentCount` | 1.0 | private | Typed pass |
| `getContentCount` | 1.1 | private | Raw pass |
| `getParentalRatingSettings` | 1.0 | generic | Raw pass |
| `setFavoriteContentList` | 1.0 | generic | Not invoked: may change state |

### browser

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `actBrowserControl` | 1.0 | generic | Not invoked: may change state |
| `getTextUrl` | 1.0 | private | Skipped: private text/URL |
| `setTextUrl` | 1.0 | generic | Not invoked: may change state |

### cec

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `setCecControlMode` | 1.0 | generic | Not invoked: may change state |
| `setMhlPowerFeedMode` | 1.0 | generic | Not invoked: may change state |
| `setPowerSyncMode` | 1.0 | generic | Not invoked: may change state |
| `setMhlAutoInputChangeMode` | 1.0 | generic | Not invoked: may change state |

### encryption

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `getPublicKey` | 1.0 | none | Typed pass |

### guide

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `getServiceProtocols` | 1.0 | none | Raw pass: `results` envelope |
| `getSupportedApiInfo` | 1.0 | none | Typed pass |

### system

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `getDeviceMode` | 1.0 | generic | TV error 3: Illegal Argument |
| `requestReboot` | 1.0 | generic | Not invoked: may change state |
| `getInterfaceInformation` | 1.0 | none | Typed pass |
| `getRemoteDeviceSettings` | 1.0 | none | Typed pass |
| `getNetworkSettings` | 1.0 | generic | Typed pass |
| `setDeviceMode` | 1.0 | generic | Not invoked: may change state |
| `getSystemSupportedFunction` | 1.0 | none | Typed pass |
| `getPowerSavingMode` | 1.0 | none | Typed pass |
| `setWolMode` | 1.0 | generic | Not invoked: may change state |
| `getWolMode` | 1.0 | generic | Typed pass |
| `setPowerSavingMode` | 1.0 | generic | Not invoked: may change state |
| `mountStorage` | 1.0 | generic | Not invoked: may change state |
| `setPowerStatus` | 1.0 | generic | Previously verified; not invoked in sweep |
| `getLEDIndicatorStatus` | 1.0 | generic | Typed pass |
| `getPowerStatus` | 1.0 | none | Typed pass |
| `getCurrentTime` | 1.0 | none | Typed pass |
| `getCurrentTime` | 1.1 | none | Typed pass |
| `getStorageList` | 1.1 | generic | TV error 7: Illegal State |
| `getSystemInformation` | 1.0 | private | Typed pass |
| `getRemoteControllerInfo` | 1.0 | none | Typed pass |
| `setLEDIndicatorStatus` | 1.1 | generic | Not invoked: may change state |
| `setLanguage` | 1.0 | generic | Not invoked: may change state |

### video

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `getPictureQualitySettings` | 1.0 | none | Typed pass |
| `setPictureQualitySettings` | 1.0 | generic | Not invoked: may change state |

### videoScreen

| Method | Version | TV auth | Outcome |
| --- | --- | --- | --- |
| `getMethodTypes` | 1.0 | none | Raw discovery pass |
| `getVersions` | 1.0 | none | Raw discovery pass |
| `setBannerMode` | 1.0 | generic | Not invoked: may change state |
| `setAudioSourceScreen` | 1.0 | generic | Not invoked: may change state |
| `requestToNotifyScreenState` | 1.1 | none | Not invoked: may change state |
| `setMultiScreenMode` | 1.0 | generic | Not invoked: may change state |
| `getPipSubScreenPosition` | 1.0 | none | Raw pass |
| `setPipSubScreenPosition` | 1.0 | generic | Not invoked: may change state |
| `getBannerMode` | 1.0 | none | Raw pass |
| `getSceneSetting` | 1.0 | none | TV error 7: Illegal State |
| `setPapScreenSize` | 1.0 | generic | Not invoked: may change state |
| `getMultiScreenMode` | 1.1 | none | Raw pass |
| `getAudioSourceScreen` | 1.0 | none | Raw pass |
| `setSceneSetting` | 1.0 | generic | Not invoked: may change state |

See the [reference coverage matrix](../API_COVERAGE.md) for repository implementation links and Sony’s documented model and locality restrictions.

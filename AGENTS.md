# Rust development

Follow [Rust Coding Standards](https://github.com/imp/rust-coding-standards/blob/main/CODING_STANDARDS.md) from [imp/rust-coding-standards](https://github.com/imp/rust-coding-standards) when writing or reviewing Rust here. Use [Rust Skills](https://github.com/leonardomso/rust-skills) and [Microsoft's Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/agents/all.txt) as supporting references. The standards' local overrides take precedence over the upstream guidance. Apply conditional sections where applicable and account for `no_std` and the firmware's interrupt and memory constraints.

- Keep production code free of `.unwrap()` and `.expect()`. Use typed errors for recoverable failures and prefer reasoned `#[expect]` attributes for lint suppressions. Follow the standards' test exceptions.
- Return owned results by default. An optional buffer-reuse API needs measurement on a hot path and a doc comment recording the evidence.
- Qualify workspace-crate types and calls through short crate aliases. Use `serde_json as json` and group imports manually as described in the standards.
- Keep decoded Sony API values distinct from raw response bytes. Keep both the API and sans-I/O client usable from `no_std`, and apply dependency and lint conventions within each Cargo workspace.

Use `curl` or `xh` for simple HTTP probes. Prefer Rust for more complex or reusable tooling.

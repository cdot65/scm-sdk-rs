# Dependency and source review

This Phase 7 dependency snapshot describes the resolved `Cargo.lock` and `cargo metadata --locked --all-features` on 2026-10-16. The lockfile fixes repository verification inputs; downstream applications resolve their own compatible dependency graph. Review lockfile changes and rerun compiler, feature, advisory, and license checks before release.

## Resolved direct dependencies

| Dependency | Resolved version | Declared Rust minimum | Declared license |
| --- | --- | --- | --- |
| `base64` | 0.22.1 | 1.48.0 | MIT OR Apache-2.0 |
| `bytes` | 1.12.1 | 1.57 | MIT |
| `futures-util` | 0.3.34 | 1.71 | MIT OR Apache-2.0 |
| `getrandom` | 0.4.3, optional IAM/native WS | 1.85 | MIT OR Apache-2.0 |
| `hmac` | 0.12.1 | Unspecified | MIT OR Apache-2.0 |
| `http` | 1.5.0 | 1.57.0 | MIT OR Apache-2.0 |
| `httparse` | 1.10.1, optional native WS | Unspecified | MIT OR Apache-2.0 |
| `httpdate` | 1.0.3 | 1.56 | MIT OR Apache-2.0 |
| `mime` | 0.3.17 | Unspecified | MIT OR Apache-2.0 |
| `regex` | 1.13.1 | 1.65 | MIT OR Apache-2.0 |
| `ryu-js` | 1.0.3 | 1.71 | Apache-2.0 OR BSL-1.0 |
| `reqwest` | 0.13.5, optional HTTP | 1.85.0 | MIT OR Apache-2.0 |
| `aws-lc-rs` | 1.18.2, optional direct native WS | 1.71.0 | ISC AND (Apache-2.0 OR ISC) |
| `rustls` | 0.23.46, optional direct native WS | 1.71 | Apache-2.0 OR ISC OR MIT |
| `rustls-native-certs` | 0.8.5, optional direct native WS | 1.71 | Apache-2.0 OR ISC OR MIT |
| `serde` | 1.0.229 | 1.56 | MIT OR Apache-2.0 |
| `serde_json` | 1.0.154 | 1.71 | MIT OR Apache-2.0 |
| `sha2` | 0.10.9 | Unspecified | MIT OR Apache-2.0 |
| `tokio` | 1.53.3 | 1.71 | MIT |
| `tokio-rustls` | 0.26.6, optional native WS | 1.71 | MIT OR Apache-2.0 |
| `tokio-websockets` | **=0.13.3**, optional native WS | **1.89** | MIT |
| `tokio-util` | 0.7.20 | 1.85 | MIT |
| `time` | 0.3.55 | 1.88 | MIT OR Apache-2.0 |
| `url` | 2.5.8 | 1.63 | MIT OR Apache-2.0 |
| `zeroize` | 1.9.1 | 1.85 | Apache-2.0 OR MIT |
| `flate2`, test dependency | 1.1.10 | 1.67.0 | MIT OR Apache-2.0 |

The metadata also contains transitive `base64` 0.23.1; it does not replace the SDK's direct 0.22.1 dependency. Direct optional rustls/AWS-LC declarations serve the native WebSocket adapter; these packages also occur transitively through the independently selectable reqwest adapter. IAM uses `getrandom` for native helper generation. The validator uses regex, exact JavaScript number formatting and calendar parsing. Node and the TypeScript toolchain are absent from the crate.

Product/client features are `models`, `scan`, `management`, `dlp`, `iam`, `model-security`, `red-team`, `gateway`, and `agentguard`; `realtime` adds the caller-connector lifecycle and `ws-transport` selects native framing/TCP/TLS. Default features remain reqwest HTTP plus scanning. `tracing` is an HTTP-stack transitive dependency; the SDK has no tracing feature or global subscriber.

The current resolved all-feature graph's highest declared Rust minimum is **1.89**, from pinned `tokio-websockets` 0.13.3. The workspace minimum is now 1.89 for every feature selection, including minimal builds. This replaces the earlier 1.88 choice driven by ICU 2.3 and time 0.3.55. Historical accepted 1.88.0 checks retain their prior source/dependency state. Current Phase 7 executable MSRV/feature qualification remains subject to its gate; metadata alone is not proof. The historical Forgejo 1.88 matrix has a pending Phase 8 update.

## HTTP, cryptography, compression, and native code

Reqwest enables `rustls`, `http2`, `stream`, `gzip`, and `system-proxy`, with its default features disabled. The resolved transport uses `rustls` 0.23.46, `aws-lc-rs` 1.18.2, and `aws-lc-sys` 0.46.0. The selected AWS-LC features are `aws-lc-sys` and `prebuilt-nasm`; FIPS features are absent. No OpenSSL/native-tls TLS implementation is selected by the default SDK transport. Transitive [`openssl-probe`](https://github.com/rustls/openssl-probe#readme) only discovers CA certificate paths for Linux root loading; it does not link or implement OpenSSL TLS.

AWS-LC contains native C/assembly and FFI. The pinned non-FIPS version builds with a native C/C++ compiler and provides generated bindings; CMake, Go, and bindgen are not requirements of this configuration. The graph contains the `cmake` helper crate, which does not establish that a CMake executable is needed for this selected build. Windows x86_64 can use the selected prebuilt NASM fallback; installing NASM permits source assembly compilation. Different downstream feature unification or FIPS selection can change requirements. These facts come from the [pinned AWS-LC Rust README](https://docs.rs/crate/aws-lc-rs/1.18.2/source/README.md).

Project-owned Rust forbids unsafe code. That guarantee does not extend to third-party dependencies or AWS-LC's FFI. A native Rust crate removes the Node runtime requirement while retaining the chosen native cryptography dependency. The current configuration does not claim FIPS validation. AWS-LC licensing is recorded as `ISC AND (Apache-2.0 OR ISC)` for `aws-lc-rs`, with additional Apache-2.0, MIT, BSD-3-Clause, ISC, and MIT-0 expressions in `aws-lc-sys`'s bundled sources.

The HTTP stack also resolves `hyper` 1.12.0 and `hyper-util` 0.1.21. Gzip uses the Rust compression backend in `flate2` 1.1.10 (`miniz_oxide`, `rust_backend`, runtime detection); the presence of other optional compression packages in metadata does not mean every backend is selected. macOS system proxy support resolves `system-configuration` 0.7.0 and its `system-configuration-sys` 0.6.0 FFI to platform frameworks. Certificate roots and proxy behavior require native platform qualification as well as local fixture tests.

### Root stores and certificate verification

The active musl-host graph selects `rustls-platform-verifier` 0.7.1 and
`rustls-native-certs` 0.8.5 through reqwest. Linux loads system roots and verifies
with webpki; Windows uses its certificate API, and Apple platforms use Security.framework.
Those platform paths include native interfaces outside project-owned safe Rust.
Linux containers therefore need a usable CA bundle; platform trust changes and
custom-root behavior require target execution. This follows the
[verifier's platform table](https://github.com/rustls/rustls-platform-verifier#readme),
checked against the pinned crate source on 2026-10-16. Linux root loading does not
provide the OS revocation behavior available on Windows and Apple platforms.

`ring` appears in the lockfile as an alternative cryptographic dependency, but
`cargo +1.99.0 tree -p scm-sdk -i ring --target x86_64-unknown-linux-musl -e normal,build`
selects no active package. AWS-LC is the selected host provider. Lockfile presence,
all-platform metadata, and active features are distinct evidence; downstream
feature unification can select additional providers. Reqwest exposes no SDK
verification bypass, and added PEM roots retain hostname/certificate verification.

## Optional WebSocket dependency decision

The native adapter pins registry `tokio-websockets =0.13.3`, MIT, declared MSRV 1.89, with checksum `d52efb639344a7c6adb8e62c6f3d2c19c001ff1b79a5041ba1c6ed42e19c6aa5`. The exact lockfile and crate source are the verification inputs. The development audit used `/tmp/scm-ws-013-reference-n917gj4m/tokio-websockets-0.13.3`; this temporary reference is not a packaged input. The [versioned changelog](https://docs.rs/crate/tokio-websockets/0.13.3/source/CHANGELOG.md) and manifest retain the upstream declarations.

The earlier 0.12 candidate was rejected after inspection of the upstream 0.13.1 ARM NEON undefined-behavior fix. That fix addresses use of an inadequately aligned pointer in the old intrinsic path. A lower declared MSRV did not justify retaining that unpatched line. Upstream's Miri claims do not qualify our ARM or other native targets. `tungstenite` 0.30 was also rejected: its raw authenticated request trace was incompatible with the SDK's credential-formatting boundary. The SDK does not suppress application logging globally to compensate for a dependency.

`realtime::NativeWebSocketConnector` owns a manually bounded HTTP/1.1 upgrade. It validates the 101 status, exact canonical accept value, required singleton headers and connection tokens, and rejects redirects, unrequested protocols/extensions and HTTP body framing. Only then does it use the framing crate's `take_over`. Framing/payload bounds, strict UTF-8, masking and control replies remain the explicit native dependency boundary. The adapter avoids the dependency's built-in upgrade parsing and keeps credential-bearing request buffers zeroizing.

Native sockets use direct TCP and an explicitly selected AWS-LC rustls provider with verified certificates and HTTP/1.1 ALPN. The connector does not install a global crypto provider or logging subscriber, and does not inherit reqwest's HTTP proxy, root or TLS settings. `with_additional_certificates` configures explicit PEM roots; `with_tls_config` accepts a trusted caller-owned rustls configuration. Proxies and alternate networking policies require a caller `WebSocketConnector`. A custom connector owns its TLS/proxy policy and must satisfy the sanitized bounded transport contract.

The current native adapter is restricted to 64-bit targets. Other target widths can use the caller-connector `realtime` feature without `ws-transport`. Only retained native execution establishes a target's behavior; additional platforms and live provider readiness remain unqualified. An HTTP upgrade alone is not proof of a working inference session. See [REALTIME.md](REALTIME.md) for lifecycle and [DEVELOPMENT.md](DEVELOPMENT.md) for actual platform evidence.

## Source and distribution boundaries

Project-owned code is Apache-2.0. The crate carries copies of the repository `LICENSE` and `NOTICE`; `scripts/check-rust.py` verifies they match. Crate packaging must preserve them, and the final delivery phase checks the actual archive and consumer build.

The frozen TypeScript source is separately licensed Apache-2.0 at commit `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Source hashes, original notices, permitted specification inputs, corrections, and fixture provenance are retained in `contracts/manifest.json`. Generated/native adaptations retain the applicable attribution. Product names do not imply a Palo Alto Networks endorsement.

The official PANW Gateway specification is not bundled or used for generation while its redistribution license remains unresolved. The dated evidence and historical licensed specification records are recorded by the provenance ledger. Development-only TypeScript tooling and specification fixtures are outside the crate's package include list. Cargo dependencies retain their own licenses and notices; license expressions above report package metadata and do not replace inspection of bundled source notices.

## Remaining release checks

This dependency review records actual choices and build requirements while preserving historical foundation evidence. Automated advisory scanning and a machine-enforced resolved license/source policy are explicitly deferred to phase 8; `docs/phase-status.json` records this phase-2 limitation. Local compiler/test/provenance reports establish only their recorded source checks. The historically configured CI matrix still needs its MSRV update and retained remote execution; dependency-policy qualification remains pending. Packaged notice verification, target execution, and release artifact validation retain their own qualification evidence.

Applicable advisories block releases unless `cdot65` approves a scoped, documented, expiring exception. Keep any advisory database revision and tool version with the final report; an old clean scan is not a statement about future dependency safety.

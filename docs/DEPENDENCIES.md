# Dependency and source review

This foundation snapshot describes the resolved `Cargo.lock` and `cargo metadata --locked --all-features` on 2026-10-16. The lockfile fixes repository verification inputs; downstream applications resolve their own compatible dependency graph. Review lockfile changes and rerun compiler, feature, advisory, and license checks before release.

## Resolved direct dependencies

| Dependency | Resolved version | Declared Rust minimum | Declared license |
| --- | --- | --- | --- |
| `base64` | 0.22.1 | 1.48.0 | MIT OR Apache-2.0 |
| `bytes` | 1.12.1 | 1.57 | MIT |
| `futures-util` | 0.3.34 | 1.71 | MIT OR Apache-2.0 |
| `hmac` | 0.12.1 | Unspecified | MIT OR Apache-2.0 |
| `http` | 1.5.0 | 1.57.0 | MIT OR Apache-2.0 |
| `httpdate` | 1.0.3 | 1.56 | MIT OR Apache-2.0 |
| `mime` | 0.3.17 | Unspecified | MIT OR Apache-2.0 |
| `reqwest` | 0.13.5, optional | 1.85.0 | MIT OR Apache-2.0 |
| `serde` | 1.0.229 | 1.56 | MIT OR Apache-2.0 |
| `serde_json` | 1.0.154 | 1.71 | MIT OR Apache-2.0 |
| `sha2` | 0.10.9 | Unspecified | MIT OR Apache-2.0 |
| `tokio` | 1.53.3 | 1.71 | MIT |
| `tokio-util` | 0.7.20 | 1.85 | MIT |
| `url` | 2.5.8 | 1.63 | MIT OR Apache-2.0 |
| `zeroize` | 1.9.1 | 1.85 | Apache-2.0 OR MIT |
| `flate2`, test dependency | 1.1.10 | 1.67.0 | MIT OR Apache-2.0 |

The metadata also contains transitive `base64` 0.23.1; it does not replace the SDK's direct 0.22.1 dependency. `tracing` remains an HTTP-stack transitive dependency; SDK instrumentation and a direct tracing feature will arrive with their implementation. The foundation's only implemented optional feature is `reqwest-transport`. Product and realtime feature declarations are added in their owning phases.

The highest declared Rust minimum in the current resolved graph is 1.88, from ICU 2.3 components used by URL/IDNA. This supports choosing workspace MSRV 1.88; successful checks on Rust 1.88.0 provide the executable proof. Declared minimum versions alone do not prove source compatibility.

## HTTP, cryptography, compression, and native code

Reqwest enables `rustls`, `http2`, `stream`, `gzip`, and `system-proxy`, with its default features disabled. The resolved transport uses `rustls` 0.23.46, `aws-lc-rs` 1.18.2, and `aws-lc-sys` 0.46.0. The selected AWS-LC features are `aws-lc-sys` and `prebuilt-nasm`; FIPS features are absent. No OpenSSL/native-tls dependency is selected by the default SDK transport.

AWS-LC contains native C/assembly and FFI. The pinned non-FIPS version builds with a native C/C++ compiler and provides generated bindings; CMake, Go, and bindgen are not requirements of this configuration. The graph contains the `cmake` helper crate, which does not establish that a CMake executable is needed for this selected build. Windows x86_64 can use the selected prebuilt NASM fallback; installing NASM permits source assembly compilation. Different downstream feature unification or FIPS selection can change requirements. These facts come from the [pinned AWS-LC Rust README](https://docs.rs/crate/aws-lc-rs/1.18.2/source/README.md).

Project-owned Rust forbids unsafe code. That guarantee does not extend to third-party dependencies or AWS-LC's FFI. A native Rust crate removes the Node runtime requirement while retaining the chosen native cryptography dependency. The current configuration does not claim FIPS validation. AWS-LC licensing is recorded as `ISC AND (Apache-2.0 OR ISC)` for `aws-lc-rs`, with additional Apache-2.0, MIT, BSD-3-Clause, ISC, and MIT-0 expressions in `aws-lc-sys`'s bundled sources.

The HTTP stack also resolves `hyper` 1.12.0 and `hyper-util` 0.1.21. Gzip uses the Rust compression backend in `flate2` 1.1.10 (`miniz_oxide`, `rust_backend`, runtime detection); the presence of other optional compression packages in metadata does not mean every backend is selected. macOS system proxy support resolves `system-configuration` 0.7.0 and its `system-configuration-sys` 0.6.0 FFI to platform frameworks. Certificate roots and proxy behavior require native platform qualification as well as local fixture tests.

## Source and distribution boundaries

Project-owned code is Apache-2.0. The crate carries copies of the repository `LICENSE` and `NOTICE`; `scripts/check-rust.py` verifies they match. Crate packaging must preserve them, and the final delivery phase checks the actual archive and consumer build.

The frozen TypeScript source is separately licensed Apache-2.0 at commit `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Source hashes, original notices, permitted specification inputs, corrections, and fixture provenance are retained in `contracts/manifest.json`. Generated/native adaptations retain the applicable attribution. Product names do not imply a Palo Alto Networks endorsement.

The official PANW Gateway specification is not bundled or used for generation while its redistribution license remains unresolved. The dated evidence and historical licensed specification records are recorded by the provenance ledger. Development-only TypeScript tooling and specification fixtures are outside the crate's package include list. Cargo dependencies retain their own licenses and notices; license expressions above report package metadata and do not replace inspection of bundled source notices.

## Remaining release checks

The foundation dependency review records actual choices and build requirements. Automated advisory scanning and a machine-enforced resolved license/source policy are explicitly deferred to phase 8; `docs/phase-status.json` records this phase-2 limitation. Compiler, test, and provenance CI results establish their recorded checks while dependency-policy qualification remains pending. Packaged notice verification, target execution, and release artifact validation retain their own qualification evidence.

Applicable advisories block releases unless `cdot65` approves a scoped, documented, expiring exception. Keep any advisory database revision and tool version with the final report; an old clean scan is not a statement about future dependency safety.

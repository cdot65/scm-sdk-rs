# Package qualification

The published `scm-sdk` archive contains native Rust sources, examples, compact
source-derived fixtures, `LICENSE`, `NOTICE`, the normalized manifest and the
Cargo lockfile. It requires no Node runtime, development contract repository or
tenant registration files. Development integration tests and the full Zod oracle
remain in this repository.

## Actual archive checks

Run each compiler explicitly:

```sh
python3 scripts/check-package.py --toolchain 1.89.0 --output .local/checks/package-msrv.json
python3 scripts/check-package.py --toolchain 1.99.0 --output .local/checks/package-current.json
```

The checker builds the real `.crate`, rejects duplicate paths, traversal, links,
devices, unexpected credential/development files and excessive archive sizes,
then writes only validated ordinary files into a private temporary directory.
It checks the packaged legal notices and VCS revision. Against that extracted
source it runs locked offline minimal unit tests, all-feature unit tests,
all-feature example compilation, doctests and a separate public consumer with
all 3,820 frozen export probes. The original repository is not the consumer's
SDK dependency. Examples are compiled without contacting services.

The model regression unit tests embed a 6.7 KiB subset with 18 exact normalized
outputs from the independent frozen Zod oracle. Reproduce or verify the subset:

```sh
python3 tools/model_fixtures.py --check
python3 -m unittest discover -s tools -p test_model_fixtures.py
python3 -m unittest discover -s tools -p test_package_policy.py
```

The subset records the upstream baseline, original corpus SHA, exact JSON
pointers, oracle/input hashes and generator hash. The full development schema
suite continues to check all public roots; the packaged subset preserves the
unit regressions without redistributing the 47 MiB oracle.

## Clean release artifact

After the reviewed source is committed, use `--require-clean` with a new output
path. This requires the complete repository to be clean before packaging, checks
`.cargo_vcs_info.json` against the exact current release commit and requires its
`dirty` value to be false. [Cargo omits a false dirty field](https://doc.rust-lang.org/stable/nightly-rustc/src/cargo/ops/cargo_package/vcs.rs.html); the checker accepts that defined omission while rejecting a present non-boolean or true value. It preserves raw metadata and records the effective dirty value separately. It checks the commit and clean status again after all
archive tests. Reports include archive SHA-256, a retained `.local/artifacts/` copy, exact file hashes/sizes, compiler,
actual native host and individual command outcomes. Preserve accepted phase
reports; write the clean artifact proof as a separate release report.

```sh
python3 scripts/check-package.py --toolchain 1.89.0 --require-clean --output .local/checks/release-package-msrv.json
```

A report written to an untracked repository path makes the next clean check
fail until it is committed or ignored. The `.local/` example is ignored. Root
release bookkeeping can retain the final proof in a later evidence-only commit.
A package build proves the recorded archive on the recorded compiler and host;
service availability and other platforms require their separate evidence.

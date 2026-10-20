# Quick start

Make one Prisma AIRS runtime scan using an API key and an existing AIRS profile.
This example uses the default scanning feature and sends a request to the US
scan service.

## 1. Add dependencies

```toml
[dependencies]
scm = { package = "scm-sdk", version = "=0.2.3" }
tokio = { version = "1", features = ["macros", "rt"] }
```

Rust 1.89 or newer is required. See [installation](/getting-started/installation)
for other product features.

## 2. Supply application inputs

Arrange for your application environment to contain:

| Variable | Value |
| --- | --- |
| `AIRS_API_KEY` | An API key authorized to scan with your selected profile. |
| `AIRS_PROFILE_NAME` | The name of an existing AIRS security profile. |

These names belong to this example. The SDK does not discover them automatically.
Use your existing secret manager or deployment configuration to supply the key.

## 3. Run a scan

Put this in `src/main.rs`:

```rust
use scm::{
    Secret,
    airs::scan::{Content, Region, Scanner, models::AiProfile},
    data::Optional,
    validation::Mode,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = Secret::new(std::env::var("AIRS_API_KEY")?)?;
    let profile_name = std::env::var("AIRS_PROFILE_NAME")?;

    let scanner = Scanner::for_region(Region::Us, Some(key), None)?;
    let profile = AiProfile::builder()
        .profile_name(Optional::Value(profile_name))
        .build_validated(Mode::Request)?;
    let content = Content::builder()
        .prompt("A harmless SDK connectivity check")
        .build()?;

    let response = scanner.sync_scan(&profile, &content, None).await?;
    println!("Scan action: {}", response.action);
    Ok(())
}
```

```sh
cargo run
```

The application owns the Tokio runtime. `Secret` redacts ordinary formatting,
and the example prints only the returned action. A successful request does not
establish that the content is allowed: interpret the result according to your
application's enforcement policy, including unfamiliar action values.

## Choose the right region

`Scanner::for_region` supports `Region::Us`, `Region::Eu`, `Region::India`, and
`Region::Singapore`. Select the region for your deployment and credentials.
For an explicit endpoint, use the
[configuration guide](/getting-started/configuration).

## Continue

The scanner also supports asynchronous batch submission, result lookup by scan
ID, and report lookup by report ID. A submitted batch receipt is not a completed
scan result.

Scan submissions do not automatically replay after an ambiguous failure.
Applications should inspect remote-outcome information before deciding how to
recover.

- [Runtime scanning](/airs/scanning)
- [Explicit environment loading](/getting-started/environment-variables)
- [Rust API reference](/reference/api/)

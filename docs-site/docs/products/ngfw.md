# NGFW

**Planned — no dedicated NGFW API client ships in `scm-sdk` 0.2.3.**

Next-Generation Firewall management is a future sibling product family within
the SDK's Strata Cloud Manager scope. Current implementation focuses on Prisma
AIRS and shared DLP and IAM services.

## What is available today

The shared HTTP foundation provides explicit OAuth credentials, destination
policies, bounded requests, cancellation, and sanitized error interfaces.
Those reusable components do not establish NGFW endpoint coverage.

There is currently no NGFW Cargo feature, typed NGFW resource module, or
qualified NGFW workflow to configure.

## Path to support

A future NGFW implementation needs a pinned API contract and an explicit
inventory of supported resources. Work should then add typed requests and
responses, authentication and tenant-scope handling, pagination, and
compatibility fixtures before service qualification.

NGFW will remain a peer of AIRS in the product model. Its public Rust namespace,
initial resource set, and release schedule are not committed.

## Explore implemented services

- [Product overview](/products/overview)
- [AIRS scanning](/airs/scanning)
- [Shared DLP](/shared/dlp)
- [Shared IAM](/shared/iam)
- [Current Rust API reference](/reference/api/)

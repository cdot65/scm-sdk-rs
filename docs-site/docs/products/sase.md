# SASE

**Planned — no dedicated SASE API client ships in `scm-sdk` 0.2.3.**

Secure Access Service Edge is a future sibling product family within the SDK's
Strata Cloud Manager scope. Its APIs will be evaluated independently from the
existing AIRS clients.

## Hostnames and product support

Some implemented AIRS services use an
`api.sase.paloaltonetworks.com` endpoint. The hostname identifies a service
destination; it does not mean this crate implements SASE management APIs.

There is currently no SASE Cargo feature, typed SASE resource module, or
qualified SASE management workflow. Shared IAM and DLP capabilities remain
available through their own modules.

## Path to support

A future SASE implementation needs a pinned API specification, a resource
inventory, and verified authentication and tenant boundaries. Typed clients,
pagination and mutation behavior, offline contract fixtures, and service
qualification would follow that contract review.

The shared Rust foundation can supply HTTP transport, OAuth, request limits,
and cancellation. Product-specific endpoints and behavior still require their
own implementation and review.

SASE will remain a peer of AIRS in the product model. Its public Rust namespace,
initial resource set, and release schedule are not committed.

## Explore implemented services

- [Product overview](/products/overview)
- [AIRS management](/airs/management)
- [Shared DLP](/shared/dlp)
- [Shared IAM](/shared/iam)
- [Current Rust API reference](/reference/api/)

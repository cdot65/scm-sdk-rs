# Live acceptance controls

Live evidence certifies specific Rust workflows on a named service deployment;
offline contracts certify request and response behavior independently. Keep those
claims separate in the parity matrix.

Select a tenant registration explicitly from `airs cli tenant list --output json`.
Load its external config file at process runtime. Credentials, registration files,
tokens, raw payloads, and signed URLs remain outside the repository and package.
Use process-only configuration and sanitized evidence in `.local/` before exporting
an allowlisted report.

Use the designated nonproduction tenant and workspace for mutation and inference
acceptance. Create uniquely named owned synthetic fixtures, record resource IDs,
bound requests and spend, and retire every supported disposable resource. Cleanup
failures and operations with unavailable deletion remain visible in the report.
Existing resources are read-only acceptance inputs unless explicitly selected for
an authorized workflow.

Service-account OAuth, AIRS scan keys, and gateway runtime credentials have separate
ownership. Configure runtime origin and credential explicitly. Apply normal TLS,
destination admission, time budgets, and redaction to token exchange as well as
API requests. Record RBAC, provider, storage, and network prerequisites separately
from SDK defects.

Promote a capability to stable only after its independent contracts, negative
cases, and live workflow succeed. Experimental capabilities remain implemented
with their true verification status when external prerequisites prevent success.
An unavailable live prerequisite never counts as a passing service operation.

# Portkey a27feb9 access-policy source findings

The [supplied commit](https://github.com/Portkey-AI/openapi/commit/a27feb9efaac3653d2d94eca5d7d210944b6e160)
adds 700 lines without deletions to openapi.yaml. Its first parent is
8f91369f15e6b6765a13c1af7e1ee3ea336f83ba. Local deterministic comparison confirms
five new operations, nine schemas, and no changes to any preexisting route or
schema. Original bytes, license and SHA256 values are retained in
contracts/vendor/portkey-access/a27feb9/source.json; normalized subset in contract.json.

The [pinned specification](https://github.com/Portkey-AI/openapi/blob/a27feb9efaac3653d2d94eca5d7d210944b6e160/openapi.yaml)
places all five operations on ControlPlaneServers and requires Portkey-Key
(x-portkey-api-key). POST/GET /policies/access and GET/PUT/DELETE
/policies/access/{accessPolicyId} all return JSON at status200. Update requires
version>=1, subjects and targets; stale version produces409. Update and delete
return extensible objects, not a specified empty HTTP body. All detail and list
response properties are optional in the schema. Subject display names and email,
policy description and created_by can be null; response vocabulary must remain
open without treating unknown effects as authorization.

List has twelve query parameters and nonnegative page_size/current_page (zero
is explicitly permitted). It supplies no cursor, snapshot, or upper page bound.
Create effect inference and allow-subject constraints occur in prose, not JSON
Schema. Provider models omission is described as ALL without a schema default.
Create enabled=true is the only explicit default added by this schema family;
it belongs to a request, and leaving it absent lets the backend apply it.

The [parent source](https://github.com/Portkey-AI/openapi/blob/8f91369f15e6b6765a13c1af7e1ee3ea336f83ba/openapi.yaml)
contains none of these routes or schemas. This is a scoped delta, not a full
baseline migration. No source in this change establishes deployment on PANW SCM
or a mapping from its Portkey key contract to SCM OAuth. The SDK should therefore
expose explicit compatible control-plane configuration and experimental support,
with implementation, upstream conformance and live acceptance recorded separately.

Research was performed directly after the background research worker failed twice
because its selected model was at capacity. No secondary sources or unexecuted
live-service claims were used.

//! Native model contracts and their reachable dependencies; client availability is recorded separately.
pub use super::generated::{
    IamScope, IamScopeBuilder, IamScopeCreateInput, IamScopeCreateInputBuilder,
    IamScopeCreateRequest, IamScopeCreateRequestBuilder, IamScopeListResponse,
    IamScopeListResponseBuilder, IamScopeResource, IamScopeResourceBuilder, IamScopeResourceInput,
    IamScopeResourceInputBuilder, IamScopeUpdateInput, IamScopeUpdateInputBuilder,
    IamScopeUpdateRequest, IamScopeUpdateRequestBuilder,
};
pub use super::support::*;
pub use crate::data::{FiniteNumber, JsonObject, JsonValue, Nullable, Optional, Patch};

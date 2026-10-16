use super::{
    ModelSecurityEvaluationListOptions, ModelSecurityFileListOptions,
    ModelSecurityLabelListOptions, ModelSecurityScanListAllOptions, ModelSecurityScanListOptions,
    ModelSecurityViolationListOptions, models::*, next_skip, oauth, query::Query, total, uuid,
};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::Optional,
    pagination::{CollectionResult, Page, Pages},
};
use http::Method;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::{
    hash::{Hash, Hasher},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
/// Scan offset plus the latched evidence that reported totals were capped.
///
/// Clones share only the monotonic count-capped evidence within their walk. Their
/// offset, equality, and hash remain immutable. Serialization records the current
/// evidence so a later resume can restore it without any global or runtime state.
#[derive(Clone)]
pub struct ScanCursor {
    skip: u64,
    count_capped: Arc<AtomicBool>,
}
impl ScanCursor {
    pub fn new(skip: u64) -> Result<Self> {
        Self::with_count_capped(skip, false)
    }
    /// Restore a position and conservative count evidence from a saved snapshot.
    pub fn with_count_capped(skip: u64, count_capped: bool) -> Result<Self> {
        if skip > 9_007_199_254_740_991 {
            Err(Error::new(ErrorKind::Validation))
        } else {
            Ok(Self {
                skip,
                count_capped: Arc::new(AtomicBool::new(count_capped)),
            })
        }
    }
    pub fn skip(&self) -> u64 {
        self.skip
    }
    pub fn count_capped(&self) -> bool {
        self.count_capped.load(Ordering::Acquire)
    }
}
// Position identity is immutable; the monotonic safety evidence is not a cursor key.
impl PartialEq for ScanCursor {
    fn eq(&self, other: &Self) -> bool {
        self.skip == other.skip
    }
}
impl Eq for ScanCursor {}
impl Hash for ScanCursor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.skip.hash(state);
    }
}
impl Serialize for ScanCursor {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        let mut value = serializer.serialize_struct("ScanCursor", 2)?;
        value.serialize_field("skip", &self.skip)?;
        value.serialize_field("count_capped", &self.count_capped())?;
        value.end()
    }
}
impl<'de> Deserialize<'de> for ScanCursor {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Snapshot {
            skip: u64,
            count_capped: bool,
        }
        let snapshot = Snapshot::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom("invalid scan cursor"))?;
        Self::with_count_capped(snapshot.skip, snapshot.count_capped)
            .map_err(|_| serde::de::Error::custom("invalid scan cursor"))
    }
}
impl std::fmt::Debug for ScanCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScanCursor")
            .field("count_capped", &self.count_capped())
            .finish_non_exhaustive()
    }
}
#[derive(Clone, Debug)]
pub struct ModelSecurityScansClient {
    pub(super) client: Client,
}
impl ModelSecurityScansClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: oauth(client)?,
        })
    }
    pub async fn create(
        &self,
        body: &ScanCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ScanBaseResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/scans")
                    .named("airs.model_security.scans.create"),
                body,
                "ScanCreateRequestSchema",
                "ScanBaseResponseSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        query: &ModelSecurityScanListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ScanList> {
        self.client
            .send_contract(
                RawRequest::read("/v1/scans")
                    .named("airs.model_security.scans.list")
                    .query(query.query()?),
                "ScanListSchema",
                options,
            )
            .await
    }
    pub fn pages(
        &self,
        query: ModelSecurityScanListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, ScanBaseResponse, ScanCursor>> {
        query.query()?;
        let limit = query.limit.unwrap_or(50);
        let initial = ScanCursor::new(query.skip.unwrap_or(0))?;
        let client = self.clone();
        Pages::new(
            initial,
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |cursor, options| {
                let client = client.clone();
                let mut query = query.clone();
                query.skip = Some(cursor.skip);
                query.limit = Some(limit);
                async move {
                    let page = client.list(&query, Some(&options)).await?;
                    if matches!(page.count_capped, Optional::Value(true)) {
                        cursor.count_capped.store(true, Ordering::Release);
                    }
                    let count_capped = cursor.count_capped();
                    let reported = if count_capped {
                        None
                    } else {
                        total(&page.pagination.total_items)?
                    };
                    let next =
                        next_skip(cursor.skip, page.scans.len(), limit, reported)?.map(|skip| {
                            ScanCursor {
                                skip,
                                count_capped: cursor.count_capped.clone(),
                            }
                        });
                    Ok(Page {
                        items: page.scans,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        query: ModelSecurityScanListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<ScanBaseResponse, ScanCursor> {
        let limit = query.limit.unwrap_or(50);
        self.pages(query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ScanBaseResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scans")
                    .segment(id)?
                    .named("airs.model_security.scans.get"),
                "ScanBaseResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_evaluations(
        &self,
        id: &str,
        query: &ModelSecurityEvaluationListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<RuleEvaluationList> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scans")
                    .segment(id)?
                    .segment("evaluations")?
                    .named("airs.model_security.scans.get_evaluations")
                    .query(query.query()?),
                "RuleEvaluationListSchema",
                options,
            )
            .await
    }
    pub async fn get_files(
        &self,
        id: &str,
        query: &ModelSecurityFileListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<FileList> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scans")
                    .segment(id)?
                    .segment("files")?
                    .named("airs.model_security.scans.get_files")
                    .query(query.query()?),
                "FileListSchema",
                options,
            )
            .await
    }
    pub async fn add_labels(
        &self,
        id: &str,
        body: &LabelsCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<LabelsResponse> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/scans")
                    .segment(id)?
                    .segment("labels")?
                    .named("airs.model_security.scans.add_labels"),
                body,
                "LabelsCreateRequestSchema",
                "LabelsResponseSchema",
                options,
            )
            .await
    }
    pub async fn set_labels(
        &self,
        id: &str,
        body: &LabelsCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<LabelsResponse> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/scans")
                    .segment(id)?
                    .segment("labels")?
                    .named("airs.model_security.scans.set_labels"),
                body,
                "LabelsCreateRequestSchema",
                "LabelsResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete_labels(
        &self,
        id: &str,
        keys: &[String],
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        uuid(id)?;
        let mut query = Query::new();
        for key in keys {
            query.text("keys", key)?;
        }
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/v1/scans")
                    .segment(id)?
                    .segment("labels")?
                    .named("airs.model_security.scans.delete_labels")
                    .query(query.finish()),
                options,
            )
            .await
    }
    pub async fn get_violations(
        &self,
        id: &str,
        query: &ModelSecurityViolationListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ViolationList> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scans")
                    .segment(id)?
                    .segment("rule-violations")?
                    .named("airs.model_security.scans.get_violations")
                    .query(query.query()?),
                "ViolationListSchema",
                options,
            )
            .await
    }
    pub async fn get_label_keys(
        &self,
        query: &ModelSecurityLabelListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<LabelKeyList> {
        self.client
            .send_contract(
                RawRequest::read("/v1/scans/label-keys")
                    .named("airs.model_security.scans.get_label_keys")
                    .query(query.query()?),
                "LabelKeyListSchema",
                options,
            )
            .await
    }
    pub async fn get_label_values(
        &self,
        key: &str,
        query: &ModelSecurityLabelListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<LabelValueList> {
        self.client
            .send_contract(
                RawRequest::read("/v1/scans/label-keys")
                    .segment(key)?
                    .segment("values")?
                    .named("airs.model_security.scans.get_label_values")
                    .query(query.query()?),
                "LabelValueListSchema",
                options,
            )
            .await
    }
    pub async fn get_evaluation(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RuleEvaluationResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/evaluations")
                    .segment(id)?
                    .named("airs.model_security.scans.get_evaluation"),
                "RuleEvaluationResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_violation(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ViolationResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/violations")
                    .segment(id)?
                    .named("airs.model_security.scans.get_violation"),
                "ViolationResponseSchema",
                options,
            )
            .await
    }
}

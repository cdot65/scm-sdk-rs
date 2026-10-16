//! Runtime resources use the same runtime key, routing snapshot and bounded transport.
use super::super::models::*;
use super::{RuntimeClient, common};
use crate::{RequestOptions, Result};
use http::Method;
impl RuntimeClient {
    pub async fn create_image(
        &self,
        body: &GatewayInferenceInputCreateImageRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceImagesResponse> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/images/generations",
                    &[],
                    "gateway.create_image",
                )?,
                body,
                "GatewayInferenceInputCreateImageRequestSchema",
                "GatewayInferenceImagesResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_rerank(
        &self,
        body: &GatewayInferenceInputCreateRerankRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceCreateRerankResponse> {
        self.client
            .send_model(
                self.request(Method::POST, "/rerank", &[], "gateway.create_rerank")?,
                body,
                "GatewayInferenceInputCreateRerankRequestSchema",
                "GatewayInferenceCreateRerankResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_ocr(
        &self,
        body: &GatewayInferenceInputCreateOcrRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceCreateOcrResponse> {
        self.client
            .send_model(
                self.request(Method::POST, "/ocr", &[], "gateway.create_ocr")?,
                body,
                "GatewayInferenceInputCreateOcrRequestSchema",
                "GatewayInferenceCreateOcrResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_fine_tuning_job(
        &self,
        body: &GatewayInferenceInputCreateFineTuningJobRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceFineTuningJob> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/fine_tuning/jobs",
                    &[],
                    "gateway.create_fine_tuning_job",
                )?,
                body,
                "GatewayInferenceInputCreateFineTuningJobRequestSchema",
                "GatewayInferenceFineTuningJobSchema",
                options,
            )
            .await
    }
    pub async fn create_moderation(
        &self,
        body: &GatewayInferenceInputCreateModerationRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceCreateModerationResponse> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/moderations",
                    &[],
                    "gateway.create_moderation",
                )?,
                body,
                "GatewayInferenceInputCreateModerationRequestSchema",
                "GatewayInferenceCreateModerationResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_vector_store(
        &self,
        body: &GatewayInferenceInputCreateVectorStoreRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreObject> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/vector_stores",
                    &[],
                    "gateway.create_vector_store",
                )?,
                body,
                "GatewayInferenceInputCreateVectorStoreRequestSchema",
                "GatewayInferenceVectorStoreObjectSchema",
                options,
            )
            .await
    }
    pub async fn modify_vector_store(
        &self,
        vector_store_id: &str,
        body: &GatewayInferenceInputUpdateVectorStoreRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreObject> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/vector_stores",
                    &[vector_store_id],
                    "gateway.modify_vector_store",
                )?,
                body,
                "GatewayInferenceInputUpdateVectorStoreRequestSchema",
                "GatewayInferenceVectorStoreObjectSchema",
                options,
            )
            .await
    }
    pub async fn create_vector_store_file(
        &self,
        vector_store_id: &str,
        body: &GatewayInferenceInputCreateVectorStoreFileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreFileObject> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/vector_stores",
                    &[vector_store_id, "files"],
                    "gateway.create_vector_store_file",
                )?,
                body,
                "GatewayInferenceInputCreateVectorStoreFileRequestSchema",
                "GatewayInferenceVectorStoreFileObjectSchema",
                options,
            )
            .await
    }
    pub async fn create_vector_store_file_batch(
        &self,
        vector_store_id: &str,
        body: &GatewayInferenceInputCreateVectorStoreFileBatchRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreFileBatchObject> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/vector_stores",
                    &[vector_store_id, "file_batches"],
                    "gateway.create_vector_store_file_batch",
                )?,
                body,
                "GatewayInferenceInputCreateVectorStoreFileBatchRequestSchema",
                "GatewayInferenceVectorStoreFileBatchObjectSchema",
                options,
            )
            .await
    }
    pub async fn create_batch(
        &self,
        body: &GatewayInferenceInputCreateBatchRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceBatch> {
        self.client
            .send_model(
                self.request(Method::POST, "/batches", &[], "gateway.create_batch")?,
                body,
                "GatewayInferenceInputCreateBatchRequestSchema",
                "GatewayInferenceBatchSchema",
                options,
            )
            .await
    }
    pub async fn create_feedback(
        &self,
        body: &GatewayInferenceInputFeedbackRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceFeedbackResponse> {
        self.client
            .send_model(
                self.request(Method::POST, "/feedback", &[], "gateway.create_feedback")?,
                body,
                "GatewayInferenceInputFeedbackRequestSchema",
                "GatewayInferenceFeedbackResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_feedback(
        &self,
        id: &str,
        body: &GatewayInferenceInputFeedbackUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceFeedbackResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                self.request(Method::PUT, "/feedback", &[id], "gateway.update_feedback")?,
                body,
                "GatewayInferenceInputFeedbackUpdateRequestSchema",
                "GatewayInferenceFeedbackResponseSchema",
                options,
            )
            .await
    }
    pub async fn retrieve_file(
        &self,
        file_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceOpenAIFile> {
        self.client
            .send_contract(
                self.request(Method::GET, "/files", &[file_id], "gateway.retrieve_file")?,
                "GatewayInferenceOpenAIFileSchema",
                options,
            )
            .await
    }
    pub async fn delete_file(
        &self,
        file_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceDeleteFileResponse> {
        self.client
            .send_contract(
                self.request(Method::DELETE, "/files", &[file_id], "gateway.delete_file")?,
                "GatewayInferenceDeleteFileResponseSchema",
                options,
            )
            .await
    }
    pub async fn download_file(
        &self,
        file_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<bytes::Bytes> {
        self.client
            .send_bytes(
                self.request(
                    Method::GET,
                    "/files",
                    &[file_id, "content"],
                    "airs.gateway.files.content",
                )?,
                options,
            )
            .await
    }
    pub async fn retrieve_fine_tuning_job(
        &self,
        fine_tuning_job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceFineTuningJob> {
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/fine_tuning/jobs",
                    &[fine_tuning_job_id],
                    "gateway.retrieve_fine_tuning_job",
                )?,
                "GatewayInferenceFineTuningJobSchema",
                options,
            )
            .await
    }
    pub async fn cancel_fine_tuning_job(
        &self,
        fine_tuning_job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceFineTuningJob> {
        self.client
            .send_contract(
                self.request(
                    Method::POST,
                    "/fine_tuning/jobs",
                    &[fine_tuning_job_id, "cancel"],
                    "gateway.cancel_fine_tuning_job",
                )?,
                "GatewayInferenceFineTuningJobSchema",
                options,
            )
            .await
    }
    pub async fn retrieve_model(
        &self,
        model: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceModel> {
        self.client
            .send_contract(
                self.request(Method::GET, "/models", &[model], "gateway.retrieve_model")?,
                "GatewayInferenceModelSchema",
                options,
            )
            .await
    }
    pub async fn delete_model(
        &self,
        model: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceDeleteModelResponse> {
        self.client
            .send_contract(
                self.request(Method::DELETE, "/models", &[model], "gateway.delete_model")?,
                "GatewayInferenceDeleteModelResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete_response(
        &self,
        response_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        self.client
            .send_empty(
                self.request(
                    Method::DELETE,
                    "/responses",
                    &[response_id],
                    "gateway.delete_response",
                )?,
                options,
            )
            .await
    }
    pub async fn get_vector_store(
        &self,
        vector_store_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreObject> {
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/vector_stores",
                    &[vector_store_id],
                    "gateway.get_vector_store",
                )?,
                "GatewayInferenceVectorStoreObjectSchema",
                options,
            )
            .await
    }
    pub async fn delete_vector_store(
        &self,
        vector_store_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceDeleteVectorStoreResponse> {
        self.client
            .send_contract(
                self.request(
                    Method::DELETE,
                    "/vector_stores",
                    &[vector_store_id],
                    "gateway.delete_vector_store",
                )?,
                "GatewayInferenceDeleteVectorStoreResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_vector_store_file(
        &self,
        vector_store_id: &str,
        file_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreFileObject> {
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/vector_stores",
                    &[vector_store_id, "files", file_id],
                    "gateway.get_vector_store_file",
                )?,
                "GatewayInferenceVectorStoreFileObjectSchema",
                options,
            )
            .await
    }
    pub async fn delete_vector_store_file(
        &self,
        vector_store_id: &str,
        file_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceDeleteVectorStoreFileResponse> {
        self.client
            .send_contract(
                self.request(
                    Method::DELETE,
                    "/vector_stores",
                    &[vector_store_id, "files", file_id],
                    "gateway.delete_vector_store_file",
                )?,
                "GatewayInferenceDeleteVectorStoreFileResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_vector_store_file_batch(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreFileBatchObject> {
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/vector_stores",
                    &[vector_store_id, "file_batches", batch_id],
                    "gateway.get_vector_store_file_batch",
                )?,
                "GatewayInferenceVectorStoreFileBatchObjectSchema",
                options,
            )
            .await
    }
    pub async fn cancel_vector_store_file_batch(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceVectorStoreFileBatchObject> {
        self.client
            .send_contract(
                self.request(
                    Method::POST,
                    "/vector_stores",
                    &[vector_store_id, "file_batches", batch_id, "cancel"],
                    "gateway.cancel_vector_store_file_batch",
                )?,
                "GatewayInferenceVectorStoreFileBatchObjectSchema",
                options,
            )
            .await
    }
    pub async fn retrieve_batch(
        &self,
        batch_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceBatch> {
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/batches",
                    &[batch_id],
                    "gateway.retrieve_batch",
                )?,
                "GatewayInferenceBatchSchema",
                options,
            )
            .await
    }
    pub async fn cancel_batch(
        &self,
        batch_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceBatch> {
        self.client
            .send_contract(
                self.request(
                    Method::POST,
                    "/batches",
                    &[batch_id, "cancel"],
                    "gateway.cancel_batch",
                )?,
                "GatewayInferenceBatchSchema",
                options,
            )
            .await
    }
    pub async fn get_batch_output(
        &self,
        batch_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<bytes::Bytes> {
        self.client
            .send_bytes(
                self.request(
                    Method::GET,
                    "/batches",
                    &[batch_id, "output"],
                    "gateway.get_batch_output",
                )?,
                options,
            )
            .await
    }
    pub async fn list_files(
        &self,
        query: &GatewayInferenceInputListFilesQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListFilesResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListFilesQuerySchema",
            &["purpose"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(Method::GET, "/files", &[], "gateway.list_files")?
                    .query(query),
                "GatewayInferenceListFilesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_paginated_fine_tuning_jobs(
        &self,
        query: &GatewayInferenceInputListPaginatedFineTuningJobsQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListPaginatedFineTuningJobsResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListPaginatedFineTuningJobsQuerySchema",
            &["after", "limit"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/fine_tuning/jobs",
                    &[],
                    "gateway.list_paginated_fine_tuning_jobs",
                )?
                .query(query),
                "GatewayInferenceListPaginatedFineTuningJobsResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_fine_tuning_events(
        &self,
        fine_tuning_job_id: &str,
        query: &GatewayInferenceInputListFineTuningEventsQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListFineTuningJobEventsResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListFineTuningEventsQuerySchema",
            &["after", "limit"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/fine_tuning/jobs",
                    &[fine_tuning_job_id, "events"],
                    "gateway.list_fine_tuning_events",
                )?
                .query(query),
                "GatewayInferenceListFineTuningJobEventsResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_fine_tuning_job_checkpoints(
        &self,
        fine_tuning_job_id: &str,
        query: &GatewayInferenceInputListFineTuningJobCheckpointsQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListFineTuningJobCheckpointsResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListFineTuningJobCheckpointsQuerySchema",
            &["after", "limit"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/fine_tuning/jobs",
                    &[fine_tuning_job_id, "checkpoints"],
                    "gateway.list_fine_tuning_job_checkpoints",
                )?
                .query(query),
                "GatewayInferenceListFineTuningJobCheckpointsResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_models(
        &self,
        query: &GatewayInferenceInputListModelsQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListModelsResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListModelsQuerySchema",
            &["ai_service", "provider", "limit", "offset", "sort", "order"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(Method::GET, "/models", &[], "gateway.list_models")?
                    .query(query),
                "GatewayInferenceListModelsResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn get_response(
        &self,
        response_id: &str,
        query: &GatewayInferenceInputGetResponseQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputGetResponseQuerySchema",
            &["include"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/responses",
                    &[response_id],
                    "gateway.get_response",
                )?
                .query(query),
                "GatewayInferenceResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_input_items(
        &self,
        response_id: &str,
        query: &GatewayInferenceInputListInputItemsQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceResponseItemList> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListInputItemsQuerySchema",
            &["limit", "order", "after", "before"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/responses",
                    &[response_id, "input_items"],
                    "gateway.list_input_items",
                )?
                .query(query),
                "GatewayInferenceResponseItemListSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_vector_stores(
        &self,
        query: &GatewayInferenceInputListVectorStoresQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListVectorStoresResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListVectorStoresQuerySchema",
            &["limit", "order", "after", "before"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/vector_stores",
                    &[],
                    "gateway.list_vector_stores",
                )?
                .query(query),
                "GatewayInferenceListVectorStoresResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_vector_store_files(
        &self,
        vector_store_id: &str,
        query: &GatewayInferenceInputListVectorStoreFilesQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListVectorStoreFilesResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListVectorStoreFilesQuerySchema",
            &["limit", "order", "after", "before", "filter"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/vector_stores",
                    &[vector_store_id, "files"],
                    "gateway.list_vector_store_files",
                )?
                .query(query),
                "GatewayInferenceListVectorStoreFilesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_files_in_vector_store_batch(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        query: &GatewayInferenceInputListFilesInVectorStoreBatchQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListVectorStoreFilesResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListFilesInVectorStoreBatchQuerySchema",
            &["limit", "order", "after", "before", "filter"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(
                    Method::GET,
                    "/vector_stores",
                    &[vector_store_id, "file_batches", batch_id, "files"],
                    "gateway.list_files_in_vector_store_batch",
                )?
                .query(query),
                "GatewayInferenceListVectorStoreFilesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_batches(
        &self,
        query: &GatewayInferenceInputListBatchesQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceListBatchesResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputListBatchesQuerySchema",
            &["after", "limit"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(Method::GET, "/batches", &[], "gateway.list_batches")?
                    .query(query),
                "GatewayInferenceListBatchesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn get_log(
        &self,
        log_id: &str,
        query: &GatewayInferenceInputGetLogQuery,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceLogObject> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayInferenceInputGetLogQuerySchema",
            &["path_format", "created_at", "type"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                self.request(Method::GET, "/logs", &[log_id], "gateway.get_log")?
                    .query(query),
                "GatewayInferenceLogObjectSchema",
                Some(&options),
            )
            .await
    }
}

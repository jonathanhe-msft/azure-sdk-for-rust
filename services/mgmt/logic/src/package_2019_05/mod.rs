#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn integration_account_agreements_client(&self) -> integration_account_agreements::Client {
        integration_account_agreements::Client(self.clone())
    }
    pub fn integration_account_assemblies_client(&self) -> integration_account_assemblies::Client {
        integration_account_assemblies::Client(self.clone())
    }
    pub fn integration_account_batch_configurations_client(&self) -> integration_account_batch_configurations::Client {
        integration_account_batch_configurations::Client(self.clone())
    }
    pub fn integration_account_certificates_client(&self) -> integration_account_certificates::Client {
        integration_account_certificates::Client(self.clone())
    }
    pub fn integration_account_maps_client(&self) -> integration_account_maps::Client {
        integration_account_maps::Client(self.clone())
    }
    pub fn integration_account_partners_client(&self) -> integration_account_partners::Client {
        integration_account_partners::Client(self.clone())
    }
    pub fn integration_account_schemas_client(&self) -> integration_account_schemas::Client {
        integration_account_schemas::Client(self.clone())
    }
    pub fn integration_account_sessions_client(&self) -> integration_account_sessions::Client {
        integration_account_sessions::Client(self.clone())
    }
    pub fn integration_accounts_client(&self) -> integration_accounts::Client {
        integration_accounts::Client(self.clone())
    }
    pub fn integration_service_environment_managed_api_operations_client(
        &self,
    ) -> integration_service_environment_managed_api_operations::Client {
        integration_service_environment_managed_api_operations::Client(self.clone())
    }
    pub fn integration_service_environment_managed_apis_client(&self) -> integration_service_environment_managed_apis::Client {
        integration_service_environment_managed_apis::Client(self.clone())
    }
    pub fn integration_service_environment_network_health_client(&self) -> integration_service_environment_network_health::Client {
        integration_service_environment_network_health::Client(self.clone())
    }
    pub fn integration_service_environment_skus_client(&self) -> integration_service_environment_skus::Client {
        integration_service_environment_skus::Client(self.clone())
    }
    pub fn integration_service_environments_client(&self) -> integration_service_environments::Client {
        integration_service_environments::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn workflow_run_action_repetitions_client(&self) -> workflow_run_action_repetitions::Client {
        workflow_run_action_repetitions::Client(self.clone())
    }
    pub fn workflow_run_action_repetitions_request_histories_client(&self) -> workflow_run_action_repetitions_request_histories::Client {
        workflow_run_action_repetitions_request_histories::Client(self.clone())
    }
    pub fn workflow_run_action_request_histories_client(&self) -> workflow_run_action_request_histories::Client {
        workflow_run_action_request_histories::Client(self.clone())
    }
    pub fn workflow_run_action_scope_repetitions_client(&self) -> workflow_run_action_scope_repetitions::Client {
        workflow_run_action_scope_repetitions::Client(self.clone())
    }
    pub fn workflow_run_actions_client(&self) -> workflow_run_actions::Client {
        workflow_run_actions::Client(self.clone())
    }
    pub fn workflow_run_operations_client(&self) -> workflow_run_operations::Client {
        workflow_run_operations::Client(self.clone())
    }
    pub fn workflow_runs_client(&self) -> workflow_runs::Client {
        workflow_runs::Client(self.clone())
    }
    pub fn workflow_trigger_histories_client(&self) -> workflow_trigger_histories::Client {
        workflow_trigger_histories::Client(self.clone())
    }
    pub fn workflow_triggers_client(&self) -> workflow_triggers::Client {
        workflow_triggers::Client(self.clone())
    }
    pub fn workflow_version_triggers_client(&self) -> workflow_version_triggers::Client {
        workflow_version_triggers::Client(self.clone())
    }
    pub fn workflow_versions_client(&self) -> workflow_versions::Client {
        workflow_versions::Client(self.clone())
    }
    pub fn workflows_client(&self) -> workflows::Client {
        workflows::Client(self.clone())
    }
}
pub mod workflows {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of workflows by subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets a list of workflows by resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
            }
        }
        #[doc = "Creates or updates a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `workflow`: The workflow."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            workflow: impl Into<models::Workflow>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                workflow: workflow.into(),
            }
        }
        #[doc = "Updates a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
            }
        }
        #[doc = "Deletes a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
            }
        }
        #[doc = "Disables a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn disable(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> disable::RequestBuilder {
            disable::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
            }
        }
        #[doc = "Enables a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn enable(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> enable::RequestBuilder {
            enable::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
            }
        }
        #[doc = "Generates the upgraded definition for a workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `parameters`: Parameters for generating an upgraded definition."]
        pub fn generate_upgraded_definition(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            parameters: impl Into<models::GenerateUpgradedDefinitionParameters>,
        ) -> generate_upgraded_definition::RequestBuilder {
            generate_upgraded_definition::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Get the workflow callback Url."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `list_callback_url`: Which callback url to list."]
        pub fn list_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            list_callback_url: impl Into<models::GetCallbackUrlParameters>,
        ) -> list_callback_url::RequestBuilder {
            list_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                list_callback_url: list_callback_url.into(),
            }
        }
        #[doc = "Gets an OpenAPI definition for the workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn list_swagger(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> list_swagger::RequestBuilder {
            list_swagger::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
            }
        }
        #[doc = "Moves an existing workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `move_`: The workflow to move."]
        pub fn move_(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            move_: impl Into<models::WorkflowReference>,
        ) -> move_::RequestBuilder {
            move_::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                move_: move_.into(),
            }
        }
        #[doc = "Regenerates the callback URL access key for request triggers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `key_type`: The access key type."]
        pub fn regenerate_access_key(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            key_type: impl Into<models::RegenerateActionParameter>,
        ) -> regenerate_access_key::RequestBuilder {
            regenerate_access_key::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                key_type: key_type.into(),
            }
        }
        #[doc = "Validates the workflow."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `validate`: The workflow."]
        pub fn validate_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            validate: impl Into<models::Workflow>,
        ) -> validate_by_resource_group::RequestBuilder {
            validate_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                validate: validate.into(),
            }
        }
        #[doc = "Validates the workflow definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `location`: The workflow location."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `validate`: The workflow."]
        pub fn validate_by_location(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            location: impl Into<String>,
            workflow_name: impl Into<String>,
            validate: impl Into<models::Workflow>,
        ) -> validate_by_location::RequestBuilder {
            validate_by_location::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                location: location.into(),
                workflow_name: workflow_name.into(),
                validate: validate.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: State, Trigger, and ReferencedResourceId."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Logic/workflows",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: State, Trigger, and ReferencedResourceId."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Workflow> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Workflow = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::Workflow>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Workflow> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Workflow = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) workflow: models::Workflow,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.workflow)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::Workflow>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Workflow> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Workflow = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::Workflow>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod disable {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/disable",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod enable {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/enable",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod generate_upgraded_definition {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Object> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Object = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) parameters: models::GenerateUpgradedDefinitionParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/generateUpgradedDefinition",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::Object>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) list_callback_url: models::GetCallbackUrlParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/listCallbackUrl",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.list_callback_url)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_swagger {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Object> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Object = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/listSwagger",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::Object>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod move_ {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) move_: models::WorkflowReference,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/move",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.move_)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod regenerate_access_key {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) key_type: models::RegenerateActionParameter,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/regenerateAccessKey",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.key_type)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod validate_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) validate: models::Workflow,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/validate",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.validate)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod validate_by_location {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) location: String,
            pub(crate) workflow_name: String,
            pub(crate) validate: models::Workflow,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/locations/{}/workflows/{}/validate",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.location,
                            &this.workflow_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.validate)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod workflow_versions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of workflow versions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                top: None,
            }
        }
        #[doc = "Gets a workflow version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `version_id`: The workflow versionId."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            version_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                version_id: version_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowVersionListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowVersionListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowVersionListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/versions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowVersion> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowVersion = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) version_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/versions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.version_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowVersion>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_triggers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of workflow triggers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets a workflow trigger."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
            }
        }
        #[doc = "Resets a workflow trigger."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn reset(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> reset::RequestBuilder {
            reset::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
            }
        }
        #[doc = "Runs a workflow trigger."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn run(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> run::RequestBuilder {
            run::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
            }
        }
        #[doc = "Get the trigger schema as JSON."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn get_schema_json(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> get_schema_json::RequestBuilder {
            get_schema_json::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
            }
        }
        #[doc = "Sets the state of a workflow trigger."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        #[doc = "* `set_state`: The workflow trigger state."]
        pub fn set_state(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
            set_state: impl Into<models::SetTriggerStateActionDefinition>,
        ) -> set_state::RequestBuilder {
            set_state::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
                set_state: set_state.into(),
            }
        }
        #[doc = "Get the callback URL for a workflow trigger."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn list_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> list_callback_url::RequestBuilder {
            list_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowTriggerListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTrigger> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTrigger = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTrigger>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod reset {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/reset",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod run {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/run",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_schema_json {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::JsonSchema> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::JsonSchema = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/schemas/json",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::JsonSchema>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod set_state {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
            pub(crate) set_state: models::SetTriggerStateActionDefinition,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/setState",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.set_state)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/listCallbackUrl",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_version_triggers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the callback url for a trigger of a workflow version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `version_id`: The workflow versionId."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn list_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            version_id: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> list_callback_url::RequestBuilder {
            list_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                version_id: version_id.into(),
                trigger_name: trigger_name.into(),
                parameters: None,
            }
        }
    }
    pub mod list_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) version_id: String,
            pub(crate) trigger_name: String,
            pub(crate) parameters: Option<models::GetCallbackUrlParameters>,
        }
        impl RequestBuilder {
            #[doc = "The callback URL parameters."]
            pub fn parameters(mut self, parameters: impl Into<models::GetCallbackUrlParameters>) -> Self {
                self.parameters = Some(parameters.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/versions/{}/triggers/{}/listCallbackUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . version_id , & this . trigger_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = if let Some(parameters) = &this.parameters {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(parameters)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_trigger_histories {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of workflow trigger histories."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets a workflow trigger history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        #[doc = "* `history_name`: The workflow trigger history name. Corresponds to the run name for triggers that resulted in a run."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
            history_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
                history_name: history_name.into(),
            }
        }
        #[doc = "Resubmits a workflow run based on the trigger history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `trigger_name`: The workflow trigger name."]
        #[doc = "* `history_name`: The workflow trigger history name. Corresponds to the run name for triggers that resulted in a run."]
        pub fn resubmit(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            trigger_name: impl Into<String>,
            history_name: impl Into<String>,
        ) -> resubmit::RequestBuilder {
            resubmit::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                trigger_name: trigger_name.into(),
                history_name: history_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerHistoryListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerHistoryListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: Status, StartTime, and ClientTrackingId."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowTriggerHistoryListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/histories",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerHistory> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerHistory = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
            pub(crate) history_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/histories/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.trigger_name,
                            &this.history_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerHistory>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod resubmit {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) trigger_name: String,
            pub(crate) history_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/triggers/{}/histories/{}/resubmit" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . trigger_name , & this . history_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod workflow_runs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of workflow runs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets a workflow run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
            }
        }
        #[doc = "Cancels a workflow run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        pub fn cancel(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
        ) -> cancel::RequestBuilder {
            cancel::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: Status, StartTime, and ClientTrackingId."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowRunListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRun> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRun = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.run_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRun>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod cancel {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.run_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod workflow_run_actions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of workflow run actions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets a workflow run action."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
            }
        }
        #[doc = "Lists a workflow run expression trace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        pub fn list_expression_traces(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
        ) -> list_expression_traces::RequestBuilder {
            list_expression_traces::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunActionListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunActionListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: Status."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::WorkflowRunActionListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.run_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunAction> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunAction = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.run_name,
                            &this.action_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRunAction>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_expression_traces {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ExpressionTraces> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ExpressionTraces = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/listExpressionTraces" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::ExpressionTraces>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_run_action_repetitions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all of a workflow run action repetitions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
            }
        }
        #[doc = "Get a workflow run action repetition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        #[doc = "* `repetition_name`: The workflow repetition."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
            repetition_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
                repetition_name: repetition_name.into(),
            }
        }
        #[doc = "Lists a workflow run expression trace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        #[doc = "* `repetition_name`: The workflow repetition."]
        pub fn list_expression_traces(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
            repetition_name: impl Into<String>,
        ) -> list_expression_traces::RequestBuilder {
            list_expression_traces::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
                repetition_name: repetition_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunActionRepetitionDefinitionCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunActionRepetitionDefinitionCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/repetitions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.run_name,
                            &this.action_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRunActionRepetitionDefinitionCollection>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunActionRepetitionDefinition> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunActionRepetitionDefinition = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
            pub(crate) repetition_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/repetitions/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name , & this . repetition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRunActionRepetitionDefinition>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_expression_traces {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ExpressionTraces> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ExpressionTraces = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
            pub(crate) repetition_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/repetitions/{}/listExpressionTraces" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name , & this . repetition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::ExpressionTraces>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_run_action_repetitions_request_histories {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List a workflow run repetition request history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        #[doc = "* `repetition_name`: The workflow repetition."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
            repetition_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
                repetition_name: repetition_name.into(),
            }
        }
        #[doc = "Gets a workflow run repetition request history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        #[doc = "* `repetition_name`: The workflow repetition."]
        #[doc = "* `request_history_name`: The request history name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
            repetition_name: impl Into<String>,
            request_history_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
                repetition_name: repetition_name.into(),
                request_history_name: request_history_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RequestHistoryListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RequestHistoryListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
            pub(crate) repetition_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::RequestHistoryListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/repetitions/{}/requestHistories" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name , & this . repetition_name)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RequestHistory> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RequestHistory = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
            pub(crate) repetition_name: String,
            pub(crate) request_history_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/repetitions/{}/requestHistories/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name , & this . repetition_name , & this . request_history_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::RequestHistory>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_run_action_request_histories {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List a workflow run request history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
            }
        }
        #[doc = "Gets a workflow run request history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        #[doc = "* `request_history_name`: The request history name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
            request_history_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
                request_history_name: request_history_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RequestHistoryListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RequestHistoryListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::RequestHistoryListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/requestHistories" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RequestHistory> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RequestHistory = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
            pub(crate) request_history_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/requestHistories/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name , & this . request_history_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::RequestHistory>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_run_action_scope_repetitions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the workflow run action scoped repetitions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
            }
        }
        #[doc = "Get a workflow run action scoped repetition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `action_name`: The workflow action name."]
        #[doc = "* `repetition_name`: The workflow repetition."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            action_name: impl Into<String>,
            repetition_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                action_name: action_name.into(),
                repetition_name: repetition_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunActionRepetitionDefinitionCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunActionRepetitionDefinitionCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/scopeRepetitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRunActionRepetitionDefinitionCollection>>
            {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRunActionRepetitionDefinition> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRunActionRepetitionDefinition = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) action_name: String,
            pub(crate) repetition_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/actions/{}/scopeRepetitions/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workflow_name , & this . run_name , & this . action_name , & this . repetition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRunActionRepetitionDefinition>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod workflow_run_operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets an operation for a run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `workflow_name`: The workflow name."]
        #[doc = "* `run_name`: The workflow run name."]
        #[doc = "* `operation_id`: The workflow operation id."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workflow_name: impl Into<String>,
            run_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workflow_name: workflow_name.into(),
                run_name: run_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowRun> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowRun = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workflow_name: String,
            pub(crate) run_name: String,
            pub(crate) operation_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}/runs/{}/operations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.workflow_name,
                            &this.run_name,
                            &this.operation_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowRun>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_accounts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration accounts by subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                top: None,
            }
        }
        #[doc = "Gets a list of integration accounts by resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
            }
        }
        #[doc = "Gets an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `integration_account`: The integration account."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            integration_account: impl Into<models::IntegrationAccount>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                integration_account: integration_account.into(),
            }
        }
        #[doc = "Updates an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `integration_account`: The integration account."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            integration_account: impl Into<models::IntegrationAccount>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                integration_account: integration_account.into(),
            }
        }
        #[doc = "Deletes an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
            }
        }
        #[doc = "Gets the integration account callback URL."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `parameters`: The callback URL parameters."]
        pub fn list_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            parameters: impl Into<models::GetCallbackUrlParameters>,
        ) -> list_callback_url::RequestBuilder {
            list_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Gets the integration account's Key Vault keys."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `list_key_vault_keys`: The key vault parameters."]
        pub fn list_key_vault_keys(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            list_key_vault_keys: impl Into<models::ListKeyVaultKeysDefinition>,
        ) -> list_key_vault_keys::RequestBuilder {
            list_key_vault_keys::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                list_key_vault_keys: list_key_vault_keys.into(),
            }
        }
        #[doc = "Logs the integration account's tracking events."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `log_tracking_events`: The callback URL parameters."]
        pub fn log_tracking_events(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            log_tracking_events: impl Into<models::TrackingEventsDefinition>,
        ) -> log_tracking_events::RequestBuilder {
            log_tracking_events::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                log_tracking_events: log_tracking_events.into(),
            }
        }
        #[doc = "Regenerates the integration account access key."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `regenerate_access_key`: The access key type."]
        pub fn regenerate_access_key(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            regenerate_access_key: impl Into<models::RegenerateActionParameter>,
        ) -> regenerate_access_key::RequestBuilder {
            regenerate_access_key::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                regenerate_access_key: regenerate_access_key.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Logic/integrationAccounts",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccount> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccount = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccount>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccount> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccount = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) integration_account: models::IntegrationAccount,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.integration_account)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccount>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccount> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccount = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) integration_account: models::IntegrationAccount,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.integration_account)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccount>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) parameters: models::GetCallbackUrlParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/listCallbackUrl",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::CallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_key_vault_keys {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::KeyVaultKeyCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::KeyVaultKeyCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) list_key_vault_keys: models::ListKeyVaultKeysDefinition,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/listKeyVaultKeys",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.list_key_vault_keys)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::KeyVaultKeyCollection>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod log_tracking_events {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) log_tracking_events: models::TrackingEventsDefinition,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/logTrackingEvents",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.log_tracking_events)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod regenerate_access_key {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccount> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccount = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) regenerate_access_key: models::RegenerateActionParameter,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/regenerateAccessKey",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.regenerate_access_key)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccount>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_account_assemblies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the assemblies for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
            }
        }
        #[doc = "Get an assembly for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `assembly_artifact_name`: The assembly artifact name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            assembly_artifact_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                assembly_artifact_name: assembly_artifact_name.into(),
            }
        }
        #[doc = "Create or update an assembly for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `assembly_artifact_name`: The assembly artifact name."]
        #[doc = "* `assembly_artifact`: The assembly artifact."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            assembly_artifact_name: impl Into<String>,
            assembly_artifact: impl Into<models::AssemblyDefinition>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                assembly_artifact_name: assembly_artifact_name.into(),
                assembly_artifact: assembly_artifact.into(),
            }
        }
        #[doc = "Delete an assembly for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `assembly_artifact_name`: The assembly artifact name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            assembly_artifact_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                assembly_artifact_name: assembly_artifact_name.into(),
            }
        }
        #[doc = "Get the content callback url for an integration account assembly."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `assembly_artifact_name`: The assembly artifact name."]
        pub fn list_content_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            assembly_artifact_name: impl Into<String>,
        ) -> list_content_callback_url::RequestBuilder {
            list_content_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                assembly_artifact_name: assembly_artifact_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AssemblyCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AssemblyCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/assemblies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::AssemblyCollection>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AssemblyDefinition> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AssemblyDefinition = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) assembly_artifact_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/assemblies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.assembly_artifact_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::AssemblyDefinition>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AssemblyDefinition> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AssemblyDefinition = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) assembly_artifact_name: String,
            pub(crate) assembly_artifact: models::AssemblyDefinition,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/assemblies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.assembly_artifact_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.assembly_artifact)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::AssemblyDefinition>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) assembly_artifact_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/assemblies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.assembly_artifact_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_content_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) assembly_artifact_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/assemblies/{}/listContentCallbackUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . integration_account_name , & this . assembly_artifact_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_account_batch_configurations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the batch configurations for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
            }
        }
        #[doc = "Get a batch configuration for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `batch_configuration_name`: The batch configuration name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            batch_configuration_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                batch_configuration_name: batch_configuration_name.into(),
            }
        }
        #[doc = "Create or update a batch configuration for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `batch_configuration_name`: The batch configuration name."]
        #[doc = "* `batch_configuration`: The batch configuration."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            batch_configuration_name: impl Into<String>,
            batch_configuration: impl Into<models::BatchConfiguration>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                batch_configuration_name: batch_configuration_name.into(),
                batch_configuration: batch_configuration.into(),
            }
        }
        #[doc = "Delete a batch configuration for an integration account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `batch_configuration_name`: The batch configuration name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            batch_configuration_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                batch_configuration_name: batch_configuration_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchConfigurationCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchConfigurationCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/batchConfigurations",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::BatchConfigurationCollection>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchConfiguration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchConfiguration = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) batch_configuration_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/batchConfigurations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.batch_configuration_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::BatchConfiguration>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchConfiguration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchConfiguration = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) batch_configuration_name: String,
            pub(crate) batch_configuration: models::BatchConfiguration,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/batchConfigurations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.batch_configuration_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.batch_configuration)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::BatchConfiguration>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) batch_configuration_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/batchConfigurations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.batch_configuration_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod integration_account_schemas {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration account schemas."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an integration account schema."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `schema_name`: The integration account schema name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            schema_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                schema_name: schema_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account schema."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `schema_name`: The integration account schema name."]
        #[doc = "* `schema`: The integration account schema."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            schema_name: impl Into<String>,
            schema: impl Into<models::IntegrationAccountSchema>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                schema_name: schema_name.into(),
                schema: schema.into(),
            }
        }
        #[doc = "Deletes an integration account schema."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `schema_name`: The integration account schema name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            schema_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                schema_name: schema_name.into(),
            }
        }
        #[doc = "Get the content callback url."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `schema_name`: The integration account schema name."]
        pub fn list_content_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            schema_name: impl Into<String>,
            list_content_callback_url: impl Into<models::GetCallbackUrlParameters>,
        ) -> list_content_callback_url::RequestBuilder {
            list_content_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                schema_name: schema_name.into(),
                list_content_callback_url: list_content_callback_url.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountSchemaListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountSchemaListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: SchemaType."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountSchemaListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/schemas",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountSchema> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountSchema = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) schema_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/schemas/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.schema_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountSchema>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountSchema> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountSchema = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) schema_name: String,
            pub(crate) schema: models::IntegrationAccountSchema,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/schemas/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.schema_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.schema)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountSchema>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) schema_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/schemas/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.schema_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_content_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) schema_name: String,
            pub(crate) list_content_callback_url: models::GetCallbackUrlParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/schemas/{}/listContentCallbackUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . integration_account_name , & this . schema_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.list_content_callback_url)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_account_maps {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration account maps."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an integration account map."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `map_name`: The integration account map name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            map_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                map_name: map_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account map. If the map is larger than 4 MB, you need to store the map in an Azure blob and use the blob's Shared Access Signature (SAS) URL as the 'contentLink' property value."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `map_name`: The integration account map name."]
        #[doc = "* `map`: The integration account map."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            map_name: impl Into<String>,
            map: impl Into<models::IntegrationAccountMap>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                map_name: map_name.into(),
                map: map.into(),
            }
        }
        #[doc = "Deletes an integration account map."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `map_name`: The integration account map name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            map_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                map_name: map_name.into(),
            }
        }
        #[doc = "Get the content callback url."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `map_name`: The integration account map name."]
        pub fn list_content_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            map_name: impl Into<String>,
            list_content_callback_url: impl Into<models::GetCallbackUrlParameters>,
        ) -> list_content_callback_url::RequestBuilder {
            list_content_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                map_name: map_name.into(),
                list_content_callback_url: list_content_callback_url.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountMapListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountMapListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: MapType."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountMapListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/maps",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountMap> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountMap = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) map_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/maps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.map_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountMap>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountMap> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountMap = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) map_name: String,
            pub(crate) map: models::IntegrationAccountMap,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/maps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.map_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.map)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountMap>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) map_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/maps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.map_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_content_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) map_name: String,
            pub(crate) list_content_callback_url: models::GetCallbackUrlParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/maps/{}/listContentCallbackUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . integration_account_name , & this . map_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.list_content_callback_url)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_account_partners {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration account partners."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an integration account partner."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `partner_name`: The integration account partner name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            partner_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                partner_name: partner_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account partner."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `partner_name`: The integration account partner name."]
        #[doc = "* `partner`: The integration account partner."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            partner_name: impl Into<String>,
            partner: impl Into<models::IntegrationAccountPartner>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                partner_name: partner_name.into(),
                partner: partner.into(),
            }
        }
        #[doc = "Deletes an integration account partner."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `partner_name`: The integration account partner name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            partner_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                partner_name: partner_name.into(),
            }
        }
        #[doc = "Get the content callback url."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `partner_name`: The integration account partner name."]
        pub fn list_content_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            partner_name: impl Into<String>,
            list_content_callback_url: impl Into<models::GetCallbackUrlParameters>,
        ) -> list_content_callback_url::RequestBuilder {
            list_content_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                partner_name: partner_name.into(),
                list_content_callback_url: list_content_callback_url.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountPartnerListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountPartnerListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: PartnerType."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountPartnerListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/partners",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountPartner> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountPartner = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) partner_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/partners/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.partner_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountPartner>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountPartner> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountPartner = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) partner_name: String,
            pub(crate) partner: models::IntegrationAccountPartner,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/partners/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.partner_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.partner)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountPartner>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) partner_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/partners/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.partner_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_content_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) partner_name: String,
            pub(crate) list_content_callback_url: models::GetCallbackUrlParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/partners/{}/listContentCallbackUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . integration_account_name , & this . partner_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.list_content_callback_url)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_account_agreements {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration account agreements."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an integration account agreement."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `agreement_name`: The integration account agreement name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            agreement_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                agreement_name: agreement_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account agreement."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `agreement_name`: The integration account agreement name."]
        #[doc = "* `agreement`: The integration account agreement."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            agreement_name: impl Into<String>,
            agreement: impl Into<models::IntegrationAccountAgreement>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                agreement_name: agreement_name.into(),
                agreement: agreement.into(),
            }
        }
        #[doc = "Deletes an integration account agreement."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `agreement_name`: The integration account agreement name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            agreement_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                agreement_name: agreement_name.into(),
            }
        }
        #[doc = "Get the content callback url."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `agreement_name`: The integration account agreement name."]
        pub fn list_content_callback_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            agreement_name: impl Into<String>,
            list_content_callback_url: impl Into<models::GetCallbackUrlParameters>,
        ) -> list_content_callback_url::RequestBuilder {
            list_content_callback_url::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                agreement_name: agreement_name.into(),
                list_content_callback_url: list_content_callback_url.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountAgreementListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountAgreementListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: AgreementType."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountAgreementListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/agreements",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountAgreement> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountAgreement = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) agreement_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/agreements/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.agreement_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountAgreement>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountAgreement> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountAgreement = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) agreement_name: String,
            pub(crate) agreement: models::IntegrationAccountAgreement,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/agreements/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.agreement_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.agreement)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountAgreement>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) agreement_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/agreements/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.agreement_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_content_callback_url {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkflowTriggerCallbackUrl> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::WorkflowTriggerCallbackUrl = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) agreement_name: String,
            pub(crate) list_content_callback_url: models::GetCallbackUrlParameters,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/agreements/{}/listContentCallbackUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . integration_account_name , & this . agreement_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.list_content_callback_url)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::WorkflowTriggerCallbackUrl>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_account_certificates {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration account certificates."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                top: None,
            }
        }
        #[doc = "Gets an integration account certificate."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `certificate_name`: The integration account certificate name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            certificate_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                certificate_name: certificate_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account certificate."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `certificate_name`: The integration account certificate name."]
        #[doc = "* `certificate`: The integration account certificate."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            certificate_name: impl Into<String>,
            certificate: impl Into<models::IntegrationAccountCertificate>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                certificate_name: certificate_name.into(),
                certificate: certificate.into(),
            }
        }
        #[doc = "Deletes an integration account certificate."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `certificate_name`: The integration account certificate name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            certificate_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                certificate_name: certificate_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountCertificateListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountCertificateListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountCertificateListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/certificates",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountCertificate> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountCertificate = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) certificate_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/certificates/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.certificate_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountCertificate>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountCertificate> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountCertificate = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) certificate_name: String,
            pub(crate) certificate: models::IntegrationAccountCertificate,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/certificates/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.certificate_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.certificate)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountCertificate>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) certificate_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/certificates/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.certificate_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod integration_account_sessions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration account sessions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an integration account session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `session_name`: The integration account session name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            session_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                session_name: session_name.into(),
            }
        }
        #[doc = "Creates or updates an integration account session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `session_name`: The integration account session name."]
        #[doc = "* `session`: The integration account session."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            session_name: impl Into<String>,
            session: impl Into<models::IntegrationAccountSession>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                session_name: session_name.into(),
                session: session.into(),
            }
        }
        #[doc = "Deletes an integration account session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group_name`: The resource group name."]
        #[doc = "* `integration_account_name`: The integration account name."]
        #[doc = "* `session_name`: The integration account session name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            integration_account_name: impl Into<String>,
            session_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                integration_account_name: integration_account_name.into(),
                session_name: session_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountSessionListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountSessionListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. Options for filters include: ChangedTime."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationAccountSessionListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/sessions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountSession> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountSession = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) session_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/sessions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.session_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountSession>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationAccountSession> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationAccountSession = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) session_name: String,
            pub(crate) session: models::IntegrationAccountSession,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/sessions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.session_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.session)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationAccountSession>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) integration_account_name: String,
            pub(crate) session_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationAccounts/{}/sessions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.integration_account_name,
                            &this.session_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod integration_service_environments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration service environments by subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                top: None,
            }
        }
        #[doc = "Gets a list of integration service environments by resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                top: None,
            }
        }
        #[doc = "Gets an integration service environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
            }
        }
        #[doc = "Creates or updates an integration service environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        #[doc = "* `integration_service_environment`: The integration service environment."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
            integration_service_environment: impl Into<models::IntegrationServiceEnvironment>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
                integration_service_environment: integration_service_environment.into(),
            }
        }
        #[doc = "Updates an integration service environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        #[doc = "* `integration_service_environment`: The integration service environment."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
            integration_service_environment: impl Into<models::IntegrationServiceEnvironment>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
                integration_service_environment: integration_service_environment.into(),
            }
        }
        #[doc = "Deletes an integration service environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
            }
        }
        #[doc = "Restarts an integration service environment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        pub fn restart(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
        ) -> restart::RequestBuilder {
            restart::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationServiceEnvironmentListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Logic/integrationServiceEnvironments",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The number of items to be included in the result."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationServiceEnvironmentListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationServiceEnvironment>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
            pub(crate) integration_service_environment: models::IntegrationServiceEnvironment,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.integration_service_environment)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationServiceEnvironment>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
            pub(crate) integration_service_environment: models::IntegrationServiceEnvironment,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.integration_service_environment)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationServiceEnvironment>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod restart {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/restart",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod integration_service_environment_skus {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of integration service environment Skus."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentSkuList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentSkuList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::IntegrationServiceEnvironmentSkuList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/skus",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod integration_service_environment_network_health {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the integration service environment network health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentNetworkHealth> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentNetworkHealth = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/health/network" , this . client . endpoint () , & this . subscription_id , & this . resource_group , & this . integration_service_environment_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationServiceEnvironmentNetworkHealth>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod integration_service_environment_managed_apis {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the integration service environment managed Apis."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
            }
        }
        #[doc = "Gets the integration service environment managed Api."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group name."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        #[doc = "* `api_name`: The api name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
            api_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
                api_name: api_name.into(),
            }
        }
        #[doc = "Puts the integration service environment managed Api."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group name."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        #[doc = "* `api_name`: The api name."]
        #[doc = "* `integration_service_environment_managed_api`: The integration service environment managed api."]
        pub fn put(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
            api_name: impl Into<String>,
            integration_service_environment_managed_api: impl Into<models::IntegrationServiceEnvironmentManagedApi>,
        ) -> put::RequestBuilder {
            put::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
                api_name: api_name.into(),
                integration_service_environment_managed_api: integration_service_environment_managed_api.into(),
            }
        }
        #[doc = "Deletes the integration service environment managed Api."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        #[doc = "* `api_name`: The api name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
            api_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
                api_name: api_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentManagedApiListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentManagedApiListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(
                self,
            ) -> azure_core::Pageable<models::IntegrationServiceEnvironmentManagedApiListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/managedApis",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group,
                            &this.integration_service_environment_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentManagedApi> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentManagedApi = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
            pub(crate) api_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/managedApis/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group , & this . integration_service_environment_name , & this . api_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationServiceEnvironmentManagedApi>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod put {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IntegrationServiceEnvironmentManagedApi> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IntegrationServiceEnvironmentManagedApi = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
            pub(crate) api_name: String,
            pub(crate) integration_service_environment_managed_api: models::IntegrationServiceEnvironmentManagedApi,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/managedApis/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group , & this . integration_service_environment_name , & this . api_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.integration_service_environment_managed_api)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<models::IntegrationServiceEnvironmentManagedApi>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
            pub(crate) api_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/managedApis/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group , & this . integration_service_environment_name , & this . api_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod integration_service_environment_managed_api_operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the managed Api operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id."]
        #[doc = "* `resource_group`: The resource group."]
        #[doc = "* `integration_service_environment_name`: The integration service environment name."]
        #[doc = "* `api_name`: The api name."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            integration_service_environment_name: impl Into<String>,
            api_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                integration_service_environment_name: integration_service_environment_name.into(),
                api_name: api_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ApiOperationListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ApiOperationListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) integration_service_environment_name: String,
            pub(crate) api_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ApiOperationListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Logic/integrationServiceEnvironments/{}/managedApis/{}/apiOperations" , this . client . endpoint () , & this . subscription_id , & this . resource_group , & this . integration_service_environment_name , & this . api_name)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available Logic REST API operations."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::OperationListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Logic/operations", this.client.endpoint(),))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}

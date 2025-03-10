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
    pub fn hyper_v_cluster_client(&self) -> hyper_v_cluster::Client {
        hyper_v_cluster::Client(self.clone())
    }
    pub fn hyper_v_host_client(&self) -> hyper_v_host::Client {
        hyper_v_host::Client(self.clone())
    }
    pub fn hyper_v_jobs_client(&self) -> hyper_v_jobs::Client {
        hyper_v_jobs::Client(self.clone())
    }
    pub fn hyper_v_machines_client(&self) -> hyper_v_machines::Client {
        hyper_v_machines::Client(self.clone())
    }
    pub fn hyper_v_operations_status_client(&self) -> hyper_v_operations_status::Client {
        hyper_v_operations_status::Client(self.clone())
    }
    pub fn hyper_v_run_as_accounts_client(&self) -> hyper_v_run_as_accounts::Client {
        hyper_v_run_as_accounts::Client(self.clone())
    }
    pub fn hyper_v_sites_client(&self) -> hyper_v_sites::Client {
        hyper_v_sites::Client(self.clone())
    }
    pub fn jobs_client(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn machines_client(&self) -> machines::Client {
        machines::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn run_as_accounts_client(&self) -> run_as_accounts::Client {
        run_as_accounts::Client(self.clone())
    }
    pub fn sites_client(&self) -> sites::Client {
        sites::Client(self.clone())
    }
    pub fn v_center_client(&self) -> v_center::Client {
        v_center::Client(self.clone())
    }
    pub fn v_mware_operations_status_client(&self) -> v_mware_operations_status::Client {
        v_mware_operations_status::Client(self.clone())
    }
}
pub mod hyper_v_cluster {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get a Hyper-V cluster."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `cluster_name`: Cluster ARM name."]
        pub fn get_cluster(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> get_cluster::RequestBuilder {
            get_cluster::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "Method to create or update a cluster in site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `cluster_name`: Cluster ARM name."]
        #[doc = "* `body`: Put cluster body."]
        pub fn put_cluster(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            cluster_name: impl Into<String>,
            body: impl Into<models::HyperVCluster>,
        ) -> put_cluster::RequestBuilder {
            put_cluster::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                cluster_name: cluster_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to get all clusters in a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_clusters_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_clusters_in_site::RequestBuilder {
            get_all_clusters_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                filter: None,
            }
        }
    }
    pub mod get_cluster {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVCluster> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVCluster = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/clusters/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.cluster_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVCluster>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod put_cluster {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
            pub(crate) cluster_name: String,
            pub(crate) body: models::HyperVCluster,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/clusters/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.cluster_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_all_clusters_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVClusterCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVClusterCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::HyperVClusterCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/clusters",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
}
pub mod hyper_v_host {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get a Hyper-V host."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `host_name`: Host ARM name."]
        pub fn get_host(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            host_name: impl Into<String>,
        ) -> get_host::RequestBuilder {
            get_host::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                host_name: host_name.into(),
            }
        }
        #[doc = "Method to create or update a host in site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `host_name`: Host ARM name."]
        #[doc = "* `body`: Put host body."]
        pub fn put_host(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            host_name: impl Into<String>,
            body: impl Into<models::HyperVHost>,
        ) -> put_host::RequestBuilder {
            put_host::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                host_name: host_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to get all hosts in a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_hosts_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_hosts_in_site::RequestBuilder {
            get_all_hosts_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                filter: None,
            }
        }
    }
    pub mod get_host {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVHost> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVHost = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) host_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/hosts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.host_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVHost>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod put_host {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
            pub(crate) host_name: String,
            pub(crate) body: models::HyperVHost,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/hosts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.host_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_all_hosts_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVHostCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVHostCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::HyperVHostCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/hosts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
}
pub mod hyper_v_jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `job_name`: Job ARM name."]
        pub fn get_job(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            job_name: impl Into<String>,
        ) -> get_job::RequestBuilder {
            get_job::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                job_name: job_name.into(),
            }
        }
        #[doc = "Method to get all jobs in a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_jobs_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_jobs_in_site::RequestBuilder {
            get_all_jobs_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
    }
    pub mod get_job {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVJob = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) job_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.job_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVJob>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_jobs_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVJobCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVJobCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::HyperVJobCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
pub mod hyper_v_machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `machine_name`: Machine ARM name."]
        pub fn get_machine(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> get_machine::RequestBuilder {
            get_machine::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                machine_name: machine_name.into(),
            }
        }
        #[doc = "Method to get machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_machines_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_machines_in_site::RequestBuilder {
            get_all_machines_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                filter: None,
                top: None,
                continuation_token: None,
                total_record_count: None,
            }
        }
    }
    pub mod get_machine {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVMachine> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVMachine = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/machines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVMachine>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_machines_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVMachineCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVMachineCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) total_record_count: Option<i32>,
        }
        impl RequestBuilder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Optional parameter for continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Total count of machines in the given site."]
            pub fn total_record_count(mut self, total_record_count: i32) -> Self {
                self.total_record_count = Some(total_record_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::HyperVMachineCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/machines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
                                if let Some(total_record_count) = &this.total_record_count {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("totalRecordCount", &total_record_count.to_string());
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
}
pub mod hyper_v_operations_status {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `operation_status_name`: Operation status ARM name."]
        pub fn get_operation_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            operation_status_name: impl Into<String>,
        ) -> get_operation_status::RequestBuilder {
            get_operation_status::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                operation_status_name: operation_status_name.into(),
            }
        }
    }
    pub mod get_operation_status {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationStatus> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationStatus = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) operation_status_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/operationsStatus/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.operation_status_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::OperationStatus>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod hyper_v_run_as_accounts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get run as account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `account_name`: Run as account ARM name."]
        pub fn get_run_as_account(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> get_run_as_account::RequestBuilder {
            get_run_as_account::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Method to get run as accounts."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_run_as_accounts_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_run_as_accounts_in_site::RequestBuilder {
            get_all_run_as_accounts_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
    }
    pub mod get_run_as_account {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVRunAsAccount> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVRunAsAccount = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) account_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/runAsAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVRunAsAccount>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_run_as_accounts_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVRunAsAccountCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVRunAsAccountCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::HyperVRunAsAccountCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/runAsAccounts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
pub mod hyper_v_sites {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_site::RequestBuilder {
            get_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to create or update a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `body`: Body with site details."]
        pub fn put_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            body: impl Into<models::HyperVSite>,
        ) -> put_site::RequestBuilder {
            put_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to patch an existing site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `body`: Body with site details."]
        pub fn patch_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            body: impl Into<models::HyperVSite>,
        ) -> patch_site::RequestBuilder {
            patch_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to delete a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn delete_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> delete_site::RequestBuilder {
            delete_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to refresh a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn refresh_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> refresh_site::RequestBuilder {
            refresh_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to get site health summary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_site_health_summary(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_site_health_summary::RequestBuilder {
            get_site_health_summary::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to get site usage."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_site_usage(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_site_usage::RequestBuilder {
            get_site_usage::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
    }
    pub mod get_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVSite> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVSite = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVSite>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod put_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVSite> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVSite = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) body: models::HyperVSite,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVSite>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod patch_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVSite> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVSite = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) body: models::HyperVSite,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVSite>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_site {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod refresh_site {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/refresh",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_site_health_summary {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SiteHealthSummaryCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SiteHealthSummaryCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::SiteHealthSummaryCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/healthSummary",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod get_site_usage {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperVSiteUsage> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperVSiteUsage = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/HyperVSites/{}/summary",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::HyperVSiteUsage>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `job_name`: Job ARM name."]
        pub fn get_job(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            job_name: impl Into<String>,
        ) -> get_job::RequestBuilder {
            get_job::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                job_name: job_name.into(),
            }
        }
        #[doc = "Method to get all jobs in a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_jobs_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_jobs_in_site::RequestBuilder {
            get_all_jobs_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
    }
    pub mod get_job {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareJob = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) job_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.job_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareJob>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_jobs_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareJobCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareJobCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::VMwareJobCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
pub mod machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `machine_name`: Machine ARM name."]
        pub fn get_machine(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> get_machine::RequestBuilder {
            get_machine::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                machine_name: machine_name.into(),
            }
        }
        #[doc = "Method to get machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_machines_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_machines_in_site::RequestBuilder {
            get_all_machines_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                filter: None,
                top: None,
                continuation_token: None,
                total_record_count: None,
            }
        }
        #[doc = "Method to stop a machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `machine_name`: Machine ARM name."]
        pub fn stop_machine(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> stop_machine::RequestBuilder {
            stop_machine::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                machine_name: machine_name.into(),
            }
        }
        #[doc = "Method to start a machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `machine_name`: Machine ARM name."]
        pub fn start_machine(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> start_machine::RequestBuilder {
            start_machine::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                machine_name: machine_name.into(),
            }
        }
    }
    pub mod get_machine {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareMachine> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareMachine = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/machines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareMachine>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_machines_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareMachineCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareMachineCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) total_record_count: Option<i32>,
        }
        impl RequestBuilder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Optional parameter for continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Total count of machines in the given site."]
            pub fn total_record_count(mut self, total_record_count: i32) -> Self {
                self.total_record_count = Some(total_record_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::VMwareMachineCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/machines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
                                if let Some(total_record_count) = &this.total_record_count {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("totalRecordCount", &total_record_count.to_string());
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
    pub mod stop_machine {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
            pub(crate) machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/machines/{}/stop",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod start_machine {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
            pub(crate) machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/machines/{}/start",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
pub mod run_as_accounts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get run as account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `account_name`: Run as account ARM name."]
        pub fn get_run_as_account(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> get_run_as_account::RequestBuilder {
            get_run_as_account::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Method to get run as accounts."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_run_as_accounts_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_run_as_accounts_in_site::RequestBuilder {
            get_all_run_as_accounts_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
    }
    pub mod get_run_as_account {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareRunAsAccount> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareRunAsAccount = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) account_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/runAsAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareRunAsAccount>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_run_as_accounts_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareRunAsAccountCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareRunAsAccountCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::VMwareRunAsAccountCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/runAsAccounts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
pub mod sites {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_site::RequestBuilder {
            get_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to create or update a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `body`: Body with site details."]
        pub fn put_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            body: impl Into<models::VMwareSite>,
        ) -> put_site::RequestBuilder {
            put_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to update an existing site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `body`: Body with site details."]
        pub fn patch_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            body: impl Into<models::VMwareSite>,
        ) -> patch_site::RequestBuilder {
            patch_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to delete a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn delete_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> delete_site::RequestBuilder {
            delete_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to refresh a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn refresh_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> refresh_site::RequestBuilder {
            refresh_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to get site health summary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_site_health_summary(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_site_health_summary::RequestBuilder {
            get_site_health_summary::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
        #[doc = "Method to get site usage/summary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_site_usage(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_site_usage::RequestBuilder {
            get_site_usage::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
            }
        }
    }
    pub mod get_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareSite> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareSite = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareSite>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod put_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareSite> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareSite = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) body: models::VMwareSite,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareSite>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod patch_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareSite> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareSite = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) body: models::VMwareSite,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareSite>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_site {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod refresh_site {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/refresh",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_site_health_summary {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SiteHealthSummaryCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SiteHealthSummaryCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::SiteHealthSummaryCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/healthSummary",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod get_site_usage {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VMwareSiteUsage> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VMwareSiteUsage = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/summary",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VMwareSiteUsage>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod v_center {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get a vCenter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `vcenter_name`: VCenter ARM name."]
        pub fn get_v_center(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            vcenter_name: impl Into<String>,
        ) -> get_v_center::RequestBuilder {
            get_v_center::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                vcenter_name: vcenter_name.into(),
            }
        }
        #[doc = "Method to create or update a vCenter in site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `vcenter_name`: VCenter ARM name."]
        #[doc = "* `body`: Put vCenter body."]
        pub fn put_v_center(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            vcenter_name: impl Into<String>,
            body: impl Into<models::VCenter>,
        ) -> put_v_center::RequestBuilder {
            put_v_center::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                vcenter_name: vcenter_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Method to delete vCenter in site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `vcenter_name`: VCenter ARM name."]
        pub fn delete_v_center(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            vcenter_name: impl Into<String>,
        ) -> delete_v_center::RequestBuilder {
            delete_v_center::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                vcenter_name: vcenter_name.into(),
            }
        }
        #[doc = "Method to get all vCenters in a site."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        pub fn get_all_v_centers_in_site(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
        ) -> get_all_v_centers_in_site::RequestBuilder {
            get_all_v_centers_in_site::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                filter: None,
            }
        }
    }
    pub mod get_v_center {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VCenter> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VCenter = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) vcenter_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/vCenters/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.vcenter_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::VCenter>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod put_v_center {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
            pub(crate) vcenter_name: String,
            pub(crate) body: models::VCenter,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/vCenters/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.vcenter_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod delete_v_center {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) site_name: String,
            pub(crate) vcenter_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/vCenters/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.vcenter_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_all_v_centers_in_site {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VCenterCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VCenterCollection = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::VCenterCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/vCenters",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
}
pub mod v_mware_operations_status {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `site_name`: Site name."]
        #[doc = "* `operation_status_name`: Operation status ARM name."]
        pub fn get_operation_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            site_name: impl Into<String>,
            operation_status_name: impl Into<String>,
        ) -> get_operation_status::RequestBuilder {
            get_operation_status::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                site_name: site_name.into(),
                operation_status_name: operation_status_name.into(),
            }
        }
    }
    pub mod get_operation_status {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationStatus> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationStatus = serde_json::from_slice(&bytes)?;
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
            pub(crate) site_name: String,
            pub(crate) operation_status_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OffAzure/VMwareSites/{}/operationsStatus/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.site_name,
                            &this.operation_status_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::OperationStatus>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of operations supported in the API."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationResultList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationResultList = serde_json::from_slice(&bytes)?;
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
            pub fn into_stream(self) -> azure_core::Pageable<models::OperationResultList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.OffAzure/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-01-01");
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

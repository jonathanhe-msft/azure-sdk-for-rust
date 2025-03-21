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
    pub fn append_blob_client(&self) -> append_blob::Client {
        append_blob::Client(self.clone())
    }
    pub fn blob_client(&self) -> blob::Client {
        blob::Client(self.clone())
    }
    pub fn block_blob_client(&self) -> block_blob::Client {
        block_blob::Client(self.clone())
    }
    pub fn container_client(&self) -> container::Client {
        container::Client(self.clone())
    }
    pub fn page_blob_client(&self) -> page_blob::Client {
        page_blob::Client(self.clone())
    }
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
}
pub mod service {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "gets the properties of a storage account's Blob service, including properties for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules."]
        pub fn get_properties(&self) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Sets properties for a storage account's Blob service endpoint, including properties for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_service_properties`: The StorageService properties."]
        pub fn set_properties(
            &self,
            storage_service_properties: impl Into<models::StorageServiceProperties>,
        ) -> set_properties::RequestBuilder {
            set_properties::RequestBuilder {
                client: self.0.clone(),
                storage_service_properties: storage_service_properties.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Retrieves statistics related to replication for the Blob service. It is only available on the secondary location endpoint when read-access geo-redundant replication is enabled for the storage account."]
        pub fn get_statistics(&self) -> get_statistics::RequestBuilder {
            get_statistics::RequestBuilder {
                client: self.0.clone(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The List Containers Segment operation returns a list of the containers under the specified account"]
        pub fn list_containers_segment(&self) -> list_containers_segment::RequestBuilder {
            list_containers_segment::RequestBuilder {
                client: self.0.clone(),
                prefix: None,
                marker: None,
                maxresults: None,
                include: Vec::new(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Retrieves a user delegation key for the Blob service. This is only a valid operation when using bearer token authentication."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `key_info`: Key information"]
        pub fn get_user_delegation_key(&self, key_info: impl Into<models::KeyInfo>) -> get_user_delegation_key::RequestBuilder {
            get_user_delegation_key::RequestBuilder {
                client: self.0.clone(),
                key_info: key_info.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Returns the sku name and account kind "]
        pub fn get_account_info(&self) -> get_account_info::RequestBuilder {
            get_account_info::RequestBuilder { client: self.0.clone() }
        }
        #[doc = "The Batch operation allows multiple API calls to be embedded into a single HTTP request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Initial data"]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `content_type`: Required. The value of this header must be multipart/mixed with a batch boundary. Example header value: multipart/mixed; boundary=batch_<GUID>"]
        pub fn submit_batch(
            &self,
            body: impl Into<serde_json::Value>,
            content_length: i64,
            content_type: impl Into<String>,
        ) -> submit_batch::RequestBuilder {
            submit_batch::RequestBuilder {
                client: self.0.clone(),
                body: body.into(),
                content_length,
                content_type: content_type.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Filter Blobs operation enables callers to list blobs across all containers whose tags match a given search expression.  Filter blobs searches across all containers within a storage account but can be scoped within the expression to a single container."]
        pub fn filter_blobs(&self) -> filter_blobs::RequestBuilder {
            filter_blobs::RequestBuilder {
                client: self.0.clone(),
                timeout: None,
                x_ms_client_request_id: None,
                where_: None,
                marker: None,
                maxresults: None,
            }
        }
    }
    pub mod get_properties {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StorageServiceProperties> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StorageServiceProperties = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::StorageServiceProperties>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod set_properties {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_service_properties: models::StorageServiceProperties,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.storage_service_properties)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_statistics {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StorageServiceStats> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StorageServiceStats = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=stats", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::StorageServiceStats>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_containers_segment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListContainersSegmentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListContainersSegmentResponse = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) prefix: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filters the results to return only containers whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Include this parameter to specify that the container's metadata be returned as part of the response body."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListContainersSegmentResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/?comp=list", this.client.endpoint(),))?;
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
                                req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                                    req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod get_user_delegation_key {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::UserDelegationKey> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::UserDelegationKey = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) key_info: models::KeyInfo,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=userdelegationkey", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.key_info)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::UserDelegationKey>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_account_info {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=account&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod submit_batch {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The media type of the body of the response. For batch requests, this is multipart/mixed; boundary=batchresponse_GUID"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) body: serde_json::Value,
            pub(crate) content_length: i64,
            pub(crate) content_type: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?comp=batch", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.insert_header("content-length", &this.content_length.to_string());
                        req.insert_header("content-type", &this.content_type);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<bytes::Bytes>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod filter_blobs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::FilterBlobSegment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::FilterBlobSegment = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) where_: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Filters the results to return only to return only blobs whose tags match the specified expression."]
            pub fn where_(mut self, where_: impl Into<String>) -> Self {
                self.where_ = Some(where_.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?comp=blobs", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(where_) = &this.where_ {
                            req.url_mut().query_pairs_mut().append_pair("where", where_);
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(maxresults) = &this.maxresults {
                            req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::FilterBlobSegment>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod container {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "returns all user-defined metadata and system properties for the specified container. The data returned does not include the container's list of blobs"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn get_properties(&self, container_name: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "creates a new container under the specified account. If the container with the same name already exists, the operation fails"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn create(&self, container_name: impl Into<String>) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_blob_public_access: None,
                x_ms_client_request_id: None,
                x_ms_default_encryption_scope: None,
                x_ms_deny_encryption_scope_override: None,
            }
        }
        #[doc = "operation marks the specified container for deletion. The container and any blobs contained within it are later deleted during garbage collection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn delete(&self, container_name: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "operation sets one or more user-defined name-value pairs for the specified container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn set_metadata(&self, container_name: impl Into<String>) -> set_metadata::RequestBuilder {
            set_metadata::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_meta: None,
                if_modified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "gets the permissions for the specified container. The permissions indicate whether container data may be accessed publicly."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn get_access_policy(&self, container_name: impl Into<String>) -> get_access_policy::RequestBuilder {
            get_access_policy::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "sets the permissions for the specified container. The permissions indicate whether blobs in a container may be accessed publicly."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn set_access_policy(&self, container_name: impl Into<String>) -> set_access_policy::RequestBuilder {
            set_access_policy::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                container_acl: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_blob_public_access: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Restores a previously-deleted container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn restore(&self, container_name: impl Into<String>) -> restore::RequestBuilder {
            restore::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_deleted_container_name: None,
                x_ms_deleted_container_version: None,
            }
        }
        #[doc = "Renames an existing container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `x_ms_source_container_name`: Required.  Specifies the name of the container to rename."]
        pub fn rename(&self, container_name: impl Into<String>, x_ms_source_container_name: impl Into<String>) -> rename::RequestBuilder {
            rename::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                x_ms_source_container_name: x_ms_source_container_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_source_lease_id: None,
            }
        }
        #[doc = "The Batch operation allows multiple API calls to be embedded into a single HTTP request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `body`: Initial data"]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `content_type`: Required. The value of this header must be multipart/mixed with a batch boundary. Example header value: multipart/mixed; boundary=batch_<GUID>"]
        pub fn submit_batch(
            &self,
            container_name: impl Into<String>,
            body: impl Into<serde_json::Value>,
            content_length: i64,
            content_type: impl Into<String>,
        ) -> submit_batch::RequestBuilder {
            submit_batch::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                body: body.into(),
                content_length,
                content_type: content_type.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Filter Blobs operation enables callers to list blobs in a container whose tags match a given search expression.  Filter blobs searches within the given container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn filter_blobs(&self, container_name: impl Into<String>) -> filter_blobs::RequestBuilder {
            filter_blobs::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
                where_: None,
                marker: None,
                maxresults: None,
            }
        }
        #[doc = "[Update] establishes and manages a lock on a container for delete operations. The lock duration can be 15 to 60 seconds, or can be infinite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn acquire_lease(
            &self,
            container_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
        ) -> acquire_lease::RequestBuilder {
            acquire_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_proposed_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] establishes and manages a lock on a container for delete operations. The lock duration can be 15 to 60 seconds, or can be infinite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn release_lease(
            &self,
            container_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> release_lease::RequestBuilder {
            release_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] establishes and manages a lock on a container for delete operations. The lock duration can be 15 to 60 seconds, or can be infinite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn renew_lease(
            &self,
            container_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> renew_lease::RequestBuilder {
            renew_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] establishes and manages a lock on a container for delete operations. The lock duration can be 15 to 60 seconds, or can be infinite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn break_lease(&self, container_name: impl Into<String>, x_ms_lease_action: impl Into<String>) -> break_lease::RequestBuilder {
            break_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_break_period: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] establishes and manages a lock on a container for delete operations. The lock duration can be 15 to 60 seconds, or can be infinite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_proposed_lease_id`: Proposed lease ID, in a GUID string format. The Blob service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
        pub fn change_lease(
            &self,
            container_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_proposed_lease_id: impl Into<String>,
        ) -> change_lease::RequestBuilder {
            change_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_proposed_lease_id: x_ms_proposed_lease_id.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The List Blobs operation returns a list of the blobs under the specified container"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn list_blob_flat_segment(&self, container_name: impl Into<String>) -> list_blob_flat_segment::RequestBuilder {
            list_blob_flat_segment::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                prefix: None,
                marker: None,
                maxresults: None,
                include: Vec::new(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The List Blobs operation returns a list of the blobs under the specified container"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `delimiter`: When the request includes this parameter, the operation returns a BlobPrefix element in the response body that acts as a placeholder for all blobs whose names begin with the same substring up to the appearance of the delimiter character. The delimiter may be a single character or a string."]
        pub fn list_blob_hierarchy_segment(
            &self,
            container_name: impl Into<String>,
            delimiter: impl Into<String>,
        ) -> list_blob_hierarchy_segment::RequestBuilder {
            list_blob_hierarchy_segment::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                delimiter: delimiter.into(),
                prefix: None,
                marker: None,
                maxresults: None,
                include: Vec::new(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Returns the sku name and account kind "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        pub fn get_account_info(&self, container_name: impl Into<String>) -> get_account_info::RequestBuilder {
            get_account_info::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
            }
        }
    }
    pub mod get_properties {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/{}?restype=container", this.client.endpoint(), &this.container_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_blob_public_access: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_default_encryption_scope: Option<String>,
            pub(crate) x_ms_deny_encryption_scope_override: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Specifies whether data in the container may be accessed publicly and the level of access"]
            pub fn x_ms_blob_public_access(mut self, x_ms_blob_public_access: impl Into<String>) -> Self {
                self.x_ms_blob_public_access = Some(x_ms_blob_public_access.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Version 2019-07-07 and later.  Specifies the default encryption scope to set on the container and use for all future writes."]
            pub fn x_ms_default_encryption_scope(mut self, x_ms_default_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_default_encryption_scope = Some(x_ms_default_encryption_scope.into());
                self
            }
            #[doc = "Optional.  Version 2019-07-07 and newer.  If true, prevents any request from specifying a different encryption scope than the scope set on the container."]
            pub fn x_ms_deny_encryption_scope_override(mut self, x_ms_deny_encryption_scope_override: bool) -> Self {
                self.x_ms_deny_encryption_scope_override = Some(x_ms_deny_encryption_scope_override);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/{}?restype=container", this.client.endpoint(), &this.container_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_blob_public_access) = &this.x_ms_blob_public_access {
                            req.insert_header("x-ms-blob-public-access", x_ms_blob_public_access);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_default_encryption_scope) = &this.x_ms_default_encryption_scope {
                            req.insert_header("x-ms-default-encryption-scope", x_ms_default_encryption_scope);
                        }
                        if let Some(x_ms_deny_encryption_scope_override) = &this.x_ms_deny_encryption_scope_override {
                            req.insert_header(
                                "x-ms-deny-encryption-scope-override",
                                &x_ms_deny_encryption_scope_override.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/{}?restype=container", this.client.endpoint(), &this.container_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_metadata {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=metadata",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_access_policy {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SignedIdentifiers> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SignedIdentifiers = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Indicated whether data in the container may be accessed publicly and the level of access"]
            pub fn x_ms_blob_public_access(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-blob-public-access"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=acl",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::SignedIdentifiers>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod set_access_policy {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) container_acl: Option<models::SignedIdentifiers>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_public_access: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "the acls for the container"]
            pub fn container_acl(mut self, container_acl: impl Into<models::SignedIdentifiers>) -> Self {
                self.container_acl = Some(container_acl.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specifies whether data in the container may be accessed publicly and the level of access"]
            pub fn x_ms_blob_public_access(mut self, x_ms_blob_public_access: impl Into<String>) -> Self {
                self.x_ms_blob_public_access = Some(x_ms_blob_public_access.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=acl",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = if let Some(container_acl) = &this.container_acl {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(container_acl)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_public_access) = &this.x_ms_blob_public_access {
                            req.insert_header("x-ms-blob-public-access", x_ms_blob_public_access);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod restore {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_deleted_container_name: Option<String>,
            pub(crate) x_ms_deleted_container_version: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Version 2019-12-12 and later.  Specifies the name of the deleted container to restore."]
            pub fn x_ms_deleted_container_name(mut self, x_ms_deleted_container_name: impl Into<String>) -> Self {
                self.x_ms_deleted_container_name = Some(x_ms_deleted_container_name.into());
                self
            }
            #[doc = "Optional.  Version 2019-12-12 and later.  Specifies the version of the deleted container to restore."]
            pub fn x_ms_deleted_container_version(mut self, x_ms_deleted_container_version: impl Into<String>) -> Self {
                self.x_ms_deleted_container_version = Some(x_ms_deleted_container_version.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=undelete",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_deleted_container_name) = &this.x_ms_deleted_container_name {
                            req.insert_header("x-ms-deleted-container-name", x_ms_deleted_container_name);
                        }
                        if let Some(x_ms_deleted_container_version) = &this.x_ms_deleted_container_version {
                            req.insert_header("x-ms-deleted-container-version", x_ms_deleted_container_version);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod rename {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) x_ms_source_container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_source_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "A lease ID for the source path. If specified, the source path must have an active lease and the lease ID must match."]
            pub fn x_ms_source_lease_id(mut self, x_ms_source_lease_id: impl Into<String>) -> Self {
                self.x_ms_source_lease_id = Some(x_ms_source_lease_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=rename",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("x-ms-source-container-name", &this.x_ms_source_container_name);
                        if let Some(x_ms_source_lease_id) = &this.x_ms_source_lease_id {
                            req.insert_header("x-ms-source-lease-id", x_ms_source_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod submit_batch {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The media type of the body of the response. For batch requests, this is multipart/mixed; boundary=batchresponse_GUID"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) body: serde_json::Value,
            pub(crate) content_length: i64,
            pub(crate) content_type: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=batch",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.insert_header("content-length", &this.content_length.to_string());
                        req.insert_header("content-type", &this.content_type);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<bytes::Bytes>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod filter_blobs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::FilterBlobSegment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::FilterBlobSegment = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) where_: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Filters the results to return only to return only blobs whose tags match the specified expression."]
            pub fn where_(mut self, where_: impl Into<String>) -> Self {
                self.where_ = Some(where_.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=blobs",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(where_) = &this.where_ {
                            req.url_mut().query_pairs_mut().append_pair("where", where_);
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(maxresults) = &this.maxresults {
                            req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::FilterBlobSegment>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod acquire_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A non-infinite lease can be between 15 and 60 seconds. A lease duration cannot be changed using renew or change."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i64) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The Blob service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?comp=lease&restype=container&acquire",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod release_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?comp=lease&restype=container&release",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod renew_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?comp=lease&restype=container&renew",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod break_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_break_period: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "For a break operation, proposed duration the lease should continue before it is broken, in seconds, between 0 and 60. This break period is only used if it is shorter than the time remaining on the lease. If longer, the time remaining on the lease is used. A new lease will not be available before the break period has expired, but the lease may be held for longer than the break period. If this header does not appear with a break operation, a fixed-duration lease breaks after the remaining lease period elapses, and an infinite lease breaks immediately."]
            pub fn x_ms_lease_break_period(mut self, x_ms_lease_break_period: i64) -> Self {
                self.x_ms_lease_break_period = Some(x_ms_lease_break_period);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?comp=lease&restype=container&break",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_break_period) = &this.x_ms_lease_break_period {
                            req.insert_header("x-ms-lease-break-period", &x_ms_lease_break_period.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod change_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_proposed_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?comp=lease&restype=container&change",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        req.insert_header("x-ms-proposed-lease-id", &this.x_ms_proposed_lease_id);
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_blob_flat_segment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListBlobsFlatSegmentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListBlobsFlatSegmentResponse = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The media type of the body of the response. For List Blobs this is 'application/xml'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) prefix: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filters the results to return only containers whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListBlobsFlatSegmentResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=list&flat",
                            this.client.endpoint(),
                            &this.container_name
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
                                req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                                    req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod list_blob_hierarchy_segment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListBlobsHierarchySegmentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListBlobsHierarchySegmentResponse = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The media type of the body of the response. For List Blobs this is 'application/xml'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) delimiter: String,
            pub(crate) prefix: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filters the results to return only containers whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListBlobsHierarchySegmentResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=container&comp=list&hierarchy",
                            this.client.endpoint(),
                            &this.container_name
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
                                req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                let delimiter = &this.delimiter;
                                req.url_mut().query_pairs_mut().append_pair("delimiter", delimiter);
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                                    req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod get_account_info {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=account&comp=properties",
                            this.client.endpoint(),
                            &this.container_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod blob {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The Download operation reads or downloads a blob from the system, including its metadata and properties. You can also call Download to read a snapshot."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn download(&self, container_name: impl Into<String>, blob: impl Into<String>) -> download::RequestBuilder {
            download::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                snapshot: None,
                versionid: None,
                timeout: None,
                x_ms_range: None,
                x_ms_lease_id: None,
                x_ms_range_get_content_md5: None,
                x_ms_range_get_content_crc64: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "If the storage account's soft delete feature is disabled then, when a blob is deleted, it is permanently removed from the storage account. If the storage account's soft delete feature is enabled, then, when a blob is deleted, it is marked for deletion and becomes inaccessible immediately. However, the blob service retains the blob or snapshot for the number of days specified by the DeleteRetentionPolicy section of [Storage service properties] (Set-Blob-Service-Properties.md). After the specified number of days has passed, the blob's data is permanently removed from the storage account. Note that you continue to be charged for the soft-deleted blob's storage until it is permanently removed. Use the List Blobs API and specify the \"include=deleted\" query parameter to discover which blobs and snapshots have been soft deleted. You can then use the Undelete Blob API to restore a soft-deleted blob. All other operations on a soft-deleted blob or snapshot causes the service to return an HTTP status code of 404 (ResourceNotFound)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn delete(&self, container_name: impl Into<String>, blob: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                snapshot: None,
                versionid: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_delete_snapshots: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
                deletetype: None,
            }
        }
        #[doc = "The Get Properties operation returns all user-defined metadata, standard HTTP properties, and system properties for the blob. It does not return the content of the blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn get_properties(&self, container_name: impl Into<String>, blob: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                snapshot: None,
                versionid: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Undelete a blob that was previously soft deleted"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn undelete(&self, container_name: impl Into<String>, blob: impl Into<String>) -> undelete::RequestBuilder {
            undelete::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Sets the time a blob will expire and be deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_expiry_option`: Required. Indicates mode of the expiry time"]
        pub fn set_expiry(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_expiry_option: impl Into<String>,
        ) -> set_expiry::RequestBuilder {
            set_expiry::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_expiry_option: x_ms_expiry_option.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_expiry_time: None,
            }
        }
        #[doc = "The Set HTTP Headers operation sets system properties on the blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn set_http_headers(&self, container_name: impl Into<String>, blob: impl Into<String>) -> set_http_headers::RequestBuilder {
            set_http_headers::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_blob_cache_control: None,
                x_ms_blob_content_type: None,
                x_ms_blob_content_md5: None,
                x_ms_blob_content_encoding: None,
                x_ms_blob_content_language: None,
                x_ms_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_blob_content_disposition: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Set Immutability Policy operation sets the immutability policy on the blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn set_immutability_policy(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
        ) -> set_immutability_policy::RequestBuilder {
            set_immutability_policy::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_client_request_id: None,
                if_unmodified_since: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
            }
        }
        #[doc = "The Delete Immutability Policy operation deletes the immutability policy on the blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn delete_immutability_policy(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
        ) -> delete_immutability_policy::RequestBuilder {
            delete_immutability_policy::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Set Legal Hold operation sets a legal hold on the blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_legal_hold`: Specified if a legal hold should be set on the blob."]
        pub fn set_legal_hold(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_legal_hold: bool,
        ) -> set_legal_hold::RequestBuilder {
            set_legal_hold::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_legal_hold,
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Set Blob Metadata operation sets user-defined metadata for the specified blob as one or more name-value pairs"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn set_metadata(&self, container_name: impl Into<String>, blob: impl Into<String>) -> set_metadata::RequestBuilder {
            set_metadata::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease Blob operation establishes and manages a lock on a blob for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn acquire_lease(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
        ) -> acquire_lease::RequestBuilder {
            acquire_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_proposed_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease Blob operation establishes and manages a lock on a blob for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn release_lease(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> release_lease::RequestBuilder {
            release_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease Blob operation establishes and manages a lock on a blob for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn renew_lease(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> renew_lease::RequestBuilder {
            renew_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease Blob operation establishes and manages a lock on a blob for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_proposed_lease_id`: Proposed lease ID, in a GUID string format. The Blob service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
        pub fn change_lease(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_proposed_lease_id: impl Into<String>,
        ) -> change_lease::RequestBuilder {
            change_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_proposed_lease_id: x_ms_proposed_lease_id.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease Blob operation establishes and manages a lock on a blob for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn break_lease(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
        ) -> break_lease::RequestBuilder {
            break_lease::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_break_period: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Create Snapshot operation creates a read-only snapshot of a blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn create_snapshot(&self, container_name: impl Into<String>, blob: impl Into<String>) -> create_snapshot::RequestBuilder {
            create_snapshot::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Start Copy From URL operation copies a blob or an internet resource to a new blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_copy_source`: Specifies the name of the source page blob snapshot. This value is a URL of up to 2 KB in length that specifies a page blob snapshot. The value should be URL-encoded as it would appear in a request URI. The source blob must either be public or must be authenticated via a shared access signature."]
        pub fn start_copy_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
        ) -> start_copy_from_url::RequestBuilder {
            start_copy_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_access_tier: None,
                x_ms_rehydrate_priority: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                x_ms_source_if_tags: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
                x_ms_tags: None,
                x_ms_seal_blob: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
                x_ms_legal_hold: None,
            }
        }
        #[doc = "The Copy From URL operation copies a blob or an internet resource to a new blob. It will not return a response until the copy is complete."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_requires_sync`: This header indicates that this is a synchronous Copy Blob From URL instead of a Asynchronous Copy Blob."]
        #[doc = "* `x_ms_copy_source`: Specifies the name of the source page blob snapshot. This value is a URL of up to 2 KB in length that specifies a page blob snapshot. The value should be URL-encoded as it would appear in a request URI. The source blob must either be public or must be authenticated via a shared access signature."]
        pub fn copy_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_requires_sync: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
        ) -> copy_from_url::RequestBuilder {
            copy_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_requires_sync: x_ms_requires_sync.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_access_tier: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
                x_ms_source_content_md5: None,
                x_ms_tags: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
                x_ms_legal_hold: None,
                x_ms_copy_source_authorization: None,
                x_ms_encryption_scope: None,
                x_ms_copy_source_tag_option: None,
            }
        }
        #[doc = "The Abort Copy From URL operation aborts a pending Copy From URL operation, and leaves a destination blob with zero length and full metadata."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_copy_action`: Copy action."]
        pub fn abort_copy_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_copy_action: impl Into<String>,
        ) -> abort_copy_from_url::RequestBuilder {
            abort_copy_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_copy_action: x_ms_copy_action.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Set Tier operation sets the tier on a blob. The operation is allowed on a page blob in a premium storage account and on a block blob in a blob storage account (locally redundant storage only). A premium page blob's tier determines the allowed size, IOPS, and bandwidth of the blob. A block blob's tier determines Hot/Cool/Archive storage type. This operation does not update the blob's ETag."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_access_tier`: Indicates the tier to be set on the blob."]
        pub fn set_tier(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_access_tier: impl Into<String>,
        ) -> set_tier::RequestBuilder {
            set_tier::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_access_tier: x_ms_access_tier.into(),
                snapshot: None,
                versionid: None,
                timeout: None,
                x_ms_rehydrate_priority: None,
                x_ms_client_request_id: None,
                x_ms_lease_id: None,
                x_ms_if_tags: None,
            }
        }
        #[doc = "Returns the sku name and account kind "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn get_account_info(&self, container_name: impl Into<String>, blob: impl Into<String>) -> get_account_info::RequestBuilder {
            get_account_info::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
            }
        }
        #[doc = "The Query operation enables users to select/project on blob data by providing simple query expressions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn query(&self, container_name: impl Into<String>, blob: impl Into<String>) -> query::RequestBuilder {
            query::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                query_request: None,
                snapshot: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Get Tags operation enables users to get the tags associated with a blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn get_tags(&self, container_name: impl Into<String>, blob: impl Into<String>) -> get_tags::RequestBuilder {
            get_tags::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_client_request_id: None,
                snapshot: None,
                versionid: None,
                x_ms_if_tags: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "The Set Tags operation enables users to set tags on a blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn set_tags(&self, container_name: impl Into<String>, blob: impl Into<String>) -> set_tags::RequestBuilder {
            set_tags::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                versionid: None,
                content_md5: None,
                x_ms_content_crc64: None,
                x_ms_client_request_id: None,
                x_ms_if_tags: None,
                x_ms_lease_id: None,
                tags: None,
            }
        }
    }
    pub mod download {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "Optional. Only valid when Object Replication is enabled for the storage container and on the destination blob of the replication."]
            pub fn x_ms_or_policy_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-or-policy-id"))
            }
            #[doc = "Optional. Only valid when Object Replication is enabled for the storage container and on the source blob of the replication. When retrieving this header, it will return the header with the policy id and rule id (e.g. x-ms-or-policyid_ruleid), and the value will be the status of the replication (e.g. complete, failed)."]
            pub fn x_ms_or(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-or"))
            }
            #[doc = "The number of bytes present in the response body."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "The media type of the body of the response. For Download Blob this is 'application/octet-stream'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "Indicates the range of bytes returned in the event that the client requested a subset of the blob by setting the 'Range' request header."]
            pub fn content_range(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-range"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "If the blob has an MD5 hash and this operation is to read the full blob, this response header is returned so that the client can check for message content integrity."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "This header returns the value that was specified for the Content-Encoding request header"]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "This header is returned if it was previously specified for the blob."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "This header returns the value that was specified for the 'x-ms-blob-content-disposition' header. The Content-Disposition response header field conveys additional information about how to process the response payload, and also can be used to attach additional metadata. For example, if set to attachment, it indicates that the user-agent should not display the response, but instead show a Save As dialog with a filename other than the blob name specified."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "This header returns the value that was specified for the Content-Language request header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "The current sequence number for a page blob. This header is not returned for block blobs or append blobs"]
            pub fn x_ms_blob_sequence_number(&self) -> azure_core::Result<i64> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-sequence-number"))
            }
            #[doc = "The blob's type."]
            pub fn x_ms_blob_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-blob-type"))
            }
            #[doc = "Conclusion time of the last attempted Copy Blob operation where this blob was the destination blob. This value can specify the time of a completed, aborted, or failed copy attempt. This header does not appear if a copy is pending, if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List."]
            pub fn x_ms_copy_completion_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-completion-time"))?,
                )
            }
            #[doc = "Only appears when x-ms-copy-status is failed or pending. Describes the cause of the last fatal or non-fatal copy operation failure. This header does not appear if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List"]
            pub fn x_ms_copy_status_description(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status-description"))
            }
            #[doc = "String identifier for this copy operation. Use with Get Blob Properties to check the status of this copy operation, or pass to Abort Copy Blob to abort a pending copy."]
            pub fn x_ms_copy_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-id"))
            }
            #[doc = "Contains the number of bytes copied and the total bytes in the source in the last attempted Copy Blob operation where this blob was the destination blob. Can show between 0 and Content-Length bytes copied. This header does not appear if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List"]
            pub fn x_ms_copy_progress(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-progress"))
            }
            #[doc = "URL up to 2 KB in length that specifies the source blob or file used in the last attempted Copy Blob operation where this blob was the destination blob. This header does not appear if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List."]
            pub fn x_ms_copy_source(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-source"))
            }
            #[doc = "State of the copy operation identified by x-ms-copy-id."]
            pub fn x_ms_copy_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status"))
            }
            #[doc = "When a blob is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the blob."]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The current lease status of the blob."]
            pub fn x_ms_lease_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-status"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A DateTime value returned by the service that uniquely identifies the blob. The value of this header indicates the blob version, and may be used in subsequent requests to access this version of the blob."]
            pub fn x_ms_version_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version-id"))
            }
            #[doc = "The value of this header indicates whether version of this blob is a current version, see also x-ms-version-id header."]
            pub fn x_ms_is_current_version(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-is-current-version"))
            }
            #[doc = "Indicates that the service supports requests for partial blob content."]
            pub fn accept_ranges(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("accept-ranges"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The number of committed blocks present in the blob. This header is returned only for append blobs."]
            pub fn x_ms_blob_committed_block_count(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-committed-block-count"))
            }
            #[doc = "The value of this header is set to true if the blob data and application metadata are completely encrypted using the specified algorithm. Otherwise, the value is set to false (when the blob is unencrypted, or if only parts of the blob/application metadata are encrypted)."]
            pub fn x_ms_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-server-encrypted"))
            }
            #[doc = "The SHA-256 hash of the encryption key used to encrypt the blob. This header is only returned when the blob was encrypted with a customer-provided key."]
            pub fn x_ms_encryption_key_sha256(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-key-sha256"))
            }
            #[doc = "Returns the name of the encryption scope used to encrypt the blob contents and application metadata.  Note that the absence of this header implies use of the default account encryption scope."]
            pub fn x_ms_encryption_scope(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-scope"))
            }
            #[doc = "If the blob has a MD5 hash, and if request contains range header (Range or x-ms-range), this response header is returned with the value of the whole blob's MD5 value. This value may or may not be equal to the value returned in Content-MD5 header, with the latter calculated from the requested range"]
            pub fn x_ms_blob_content_md5(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-blob-content-md5"))
            }
            #[doc = "The number of tags associated with the blob"]
            pub fn x_ms_tag_count(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("x-ms-tag-count"))
            }
            #[doc = "If this blob has been sealed"]
            pub fn x_ms_blob_sealed(&self) -> azure_core::Result<bool> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-sealed"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the blob was last read or written to"]
            pub fn x_ms_last_access_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-last-access-time"))?,
                )
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the blob immutability policy will expire."]
            pub fn x_ms_immutability_policy_until_date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-immutability-policy-until-date"))?,
                )
            }
            #[doc = "Indicates immutability policy mode."]
            pub fn x_ms_immutability_policy_mode(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-immutability-policy-mode"))
            }
            #[doc = "Indicates if a legal hold is present on the blob."]
            pub fn x_ms_legal_hold(&self) -> azure_core::Result<bool> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("x-ms-legal-hold"))
            }
            #[doc = "If the request is to read a specified range and the x-ms-range-get-content-crc64 is set to true, then the request returns a crc64 for the range, as long as the range size is less than or equal to 4 MB. If both x-ms-range-get-content-crc64 & x-ms-range-get-content-md5 is specified in the same request, it will fail with 400(Bad Request)"]
            pub fn x_ms_content_crc64(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-content-crc64"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) versionid: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_range_get_content_md5: Option<bool>,
            pub(crate) x_ms_range_get_content_crc64: Option<bool>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate on. It's for service version 2019-10-10 and newer."]
            pub fn versionid(mut self, versionid: impl Into<String>) -> Self {
                self.versionid = Some(versionid.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Return only the bytes of the blob in the specified range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "When set to true and specified together with the Range, the service returns the MD5 hash for the range, as long as the range is less than or equal to 4 MB in size."]
            pub fn x_ms_range_get_content_md5(mut self, x_ms_range_get_content_md5: bool) -> Self {
                self.x_ms_range_get_content_md5 = Some(x_ms_range_get_content_md5);
                self
            }
            #[doc = "When set to true and specified together with the Range, the service returns the CRC64 hash for the range, as long as the range is less than or equal to 4 MB in size."]
            pub fn x_ms_range_get_content_crc64(mut self, x_ms_range_get_content_crc64: bool) -> Self {
                self.x_ms_range_get_content_crc64 = Some(x_ms_range_get_content_crc64);
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}/{}", this.client.endpoint(), &this.container_name, &this.blob))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        if let Some(versionid) = &this.versionid {
                            req.url_mut().query_pairs_mut().append_pair("versionid", versionid);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_range_get_content_md5) = &this.x_ms_range_get_content_md5 {
                            req.insert_header("x-ms-range-get-content-md5", &x_ms_range_get_content_md5.to_string());
                        }
                        if let Some(x_ms_range_get_content_crc64) = &this.x_ms_range_get_content_crc64 {
                            req.insert_header("x-ms-range-get-content-crc64", &x_ms_range_get_content_crc64.to_string());
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<bytes::Bytes>> {
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
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) versionid: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_delete_snapshots: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) deletetype: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate on. It's for service version 2019-10-10 and newer."]
            pub fn versionid(mut self, versionid: impl Into<String>) -> Self {
                self.versionid = Some(versionid.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Required if the blob has associated snapshots. Specify one of the following two options: include: Delete the base blob and all of its snapshots. only: Delete only the blob's snapshots and not the blob itself"]
            pub fn x_ms_delete_snapshots(mut self, x_ms_delete_snapshots: impl Into<String>) -> Self {
                self.x_ms_delete_snapshots = Some(x_ms_delete_snapshots.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Only possible value is 'permanent', which specifies to permanently delete a blob if blob soft delete is enabled."]
            pub fn deletetype(mut self, deletetype: impl Into<String>) -> Self {
                self.deletetype = Some(deletetype.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}/{}", this.client.endpoint(), &this.container_name, &this.blob))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        if let Some(versionid) = &this.versionid {
                            req.url_mut().query_pairs_mut().append_pair("versionid", versionid);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_delete_snapshots) = &this.x_ms_delete_snapshots {
                            req.insert_header("x-ms-delete-snapshots", x_ms_delete_snapshots);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(deletetype) = &this.deletetype {
                            req.url_mut().query_pairs_mut().append_pair("deletetype", deletetype);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_properties {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) versionid: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate on. It's for service version 2019-10-10 and newer."]
            pub fn versionid(mut self, versionid: impl Into<String>) -> Self {
                self.versionid = Some(versionid.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}/{}", this.client.endpoint(), &this.container_name, &this.blob))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        if let Some(versionid) = &this.versionid {
                            req.url_mut().query_pairs_mut().append_pair("versionid", versionid);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod undelete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=undelete",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_expiry {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_expiry_option: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_expiry_time: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The time to set the blob to expiry"]
            pub fn x_ms_expiry_time(mut self, x_ms_expiry_time: impl Into<String>) -> Self {
                self.x_ms_expiry_time = Some(x_ms_expiry_time.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=expiry",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("x-ms-expiry-option", &this.x_ms_expiry_option);
                        if let Some(x_ms_expiry_time) = &this.x_ms_expiry_time {
                            req.insert_header("x-ms-expiry-time", x_ms_expiry_time);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_http_headers {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_blob_cache_control: Option<String>,
            pub(crate) x_ms_blob_content_type: Option<String>,
            pub(crate) x_ms_blob_content_md5: Option<String>,
            pub(crate) x_ms_blob_content_encoding: Option<String>,
            pub(crate) x_ms_blob_content_language: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_blob_content_disposition: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_cache_control(mut self, x_ms_blob_cache_control: impl Into<String>) -> Self {
                self.x_ms_blob_cache_control = Some(x_ms_blob_cache_control.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_type(mut self, x_ms_blob_content_type: impl Into<String>) -> Self {
                self.x_ms_blob_content_type = Some(x_ms_blob_content_type.into());
                self
            }
            #[doc = "Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks were validated when each was uploaded."]
            pub fn x_ms_blob_content_md5(mut self, x_ms_blob_content_md5: impl Into<String>) -> Self {
                self.x_ms_blob_content_md5 = Some(x_ms_blob_content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_encoding(mut self, x_ms_blob_content_encoding: impl Into<String>) -> Self {
                self.x_ms_blob_content_encoding = Some(x_ms_blob_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_language(mut self, x_ms_blob_content_language: impl Into<String>) -> Self {
                self.x_ms_blob_content_language = Some(x_ms_blob_content_language.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_blob_content_disposition(mut self, x_ms_blob_content_disposition: impl Into<String>) -> Self {
                self.x_ms_blob_content_disposition = Some(x_ms_blob_content_disposition.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=properties&SetHTTPHeaders",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_blob_cache_control) = &this.x_ms_blob_cache_control {
                            req.insert_header("x-ms-blob-cache-control", x_ms_blob_cache_control);
                        }
                        if let Some(x_ms_blob_content_type) = &this.x_ms_blob_content_type {
                            req.insert_header("x-ms-blob-content-type", x_ms_blob_content_type);
                        }
                        if let Some(x_ms_blob_content_md5) = &this.x_ms_blob_content_md5 {
                            req.insert_header("x-ms-blob-content-md5", x_ms_blob_content_md5);
                        }
                        if let Some(x_ms_blob_content_encoding) = &this.x_ms_blob_content_encoding {
                            req.insert_header("x-ms-blob-content-encoding", x_ms_blob_content_encoding);
                        }
                        if let Some(x_ms_blob_content_language) = &this.x_ms_blob_content_language {
                            req.insert_header("x-ms-blob-content-language", x_ms_blob_content_language);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_blob_content_disposition) = &this.x_ms_blob_content_disposition {
                            req.insert_header("x-ms-blob-content-disposition", x_ms_blob_content_disposition);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_immutability_policy {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=immutabilityPolicies",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod delete_immutability_policy {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=immutabilityPolicies",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_legal_hold {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_legal_hold: bool,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=legalhold",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("x-ms-legal-hold", &this.x_ms_legal_hold.to_string());
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_metadata {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=metadata",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod acquire_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A non-infinite lease can be between 15 and 60 seconds. A lease duration cannot be changed using renew or change."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i64) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The Blob service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=lease&acquire",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod release_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=lease&release",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod renew_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=lease&renew",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod change_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_proposed_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=lease&change",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        req.insert_header("x-ms-proposed-lease-id", &this.x_ms_proposed_lease_id);
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod break_lease {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_break_period: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "For a break operation, proposed duration the lease should continue before it is broken, in seconds, between 0 and 60. This break period is only used if it is shorter than the time remaining on the lease. If longer, the time remaining on the lease is used. A new lease will not be available before the break period has expired, but the lease may be held for longer than the break period. If this header does not appear with a break operation, a fixed-duration lease breaks after the remaining lease period elapses, and an infinite lease breaks immediately."]
            pub fn x_ms_lease_break_period(mut self, x_ms_lease_break_period: i64) -> Self {
                self.x_ms_lease_break_period = Some(x_ms_lease_break_period);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=lease&break",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_break_period) = &this.x_ms_lease_break_period {
                            req.insert_header("x-ms-lease-break-period", &x_ms_lease_break_period.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_snapshot {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=snapshot",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod start_copy_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_rehydrate_priority: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) x_ms_source_if_tags: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_seal_blob: Option<bool>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
            pub(crate) x_ms_legal_hold: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Optional. Indicates the tier to be set on the blob."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Optional: Indicates the priority with which to rehydrate an archived blob."]
            pub fn x_ms_rehydrate_priority(mut self, x_ms_rehydrate_priority: impl Into<String>) -> Self {
                self.x_ms_rehydrate_priority = Some(x_ms_rehydrate_priority.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_tags(mut self, x_ms_source_if_tags: impl Into<String>) -> Self {
                self.x_ms_source_if_tags = Some(x_ms_source_if_tags.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Overrides the sealed state of the destination blob.  Service version 2019-12-12 and newer."]
            pub fn x_ms_seal_blob(mut self, x_ms_seal_blob: bool) -> Self {
                self.x_ms_seal_blob = Some(x_ms_seal_blob);
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Specified if a legal hold should be set on the blob."]
            pub fn x_ms_legal_hold(mut self, x_ms_legal_hold: bool) -> Self {
                self.x_ms_legal_hold = Some(x_ms_legal_hold);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=copy",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(x_ms_rehydrate_priority) = &this.x_ms_rehydrate_priority {
                            req.insert_header("x-ms-rehydrate-priority", x_ms_rehydrate_priority);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(x_ms_source_if_tags) = &this.x_ms_source_if_tags {
                            req.insert_header("x-ms-source-if-tags", x_ms_source_if_tags);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        if let Some(x_ms_seal_blob) = &this.x_ms_seal_blob {
                            req.insert_header("x-ms-seal-blob", &x_ms_seal_blob.to_string());
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        if let Some(x_ms_legal_hold) = &this.x_ms_legal_hold {
                            req.insert_header("x-ms-legal-hold", &x_ms_legal_hold.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod copy_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_requires_sync: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_source_content_md5: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
            pub(crate) x_ms_legal_hold: Option<bool>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_copy_source_tag_option: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Optional. Indicates the tier to be set on the blob."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specify the md5 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_md5(mut self, x_ms_source_content_md5: impl Into<String>) -> Self {
                self.x_ms_source_content_md5 = Some(x_ms_source_content_md5.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Specified if a legal hold should be set on the blob."]
            pub fn x_ms_legal_hold(mut self, x_ms_legal_hold: bool) -> Self {
                self.x_ms_legal_hold = Some(x_ms_legal_hold);
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Optional, default 'replace'.  Indicates if source tags should be copied or replaced with the tags specified by x-ms-tags."]
            pub fn x_ms_copy_source_tag_option(mut self, x_ms_copy_source_tag_option: impl Into<String>) -> Self {
                self.x_ms_copy_source_tag_option = Some(x_ms_copy_source_tag_option.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=copy&sync",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-requires-sync", &this.x_ms_requires_sync);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_source_content_md5) = &this.x_ms_source_content_md5 {
                            req.insert_header("x-ms-source-content-md5", x_ms_source_content_md5);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        if let Some(x_ms_legal_hold) = &this.x_ms_legal_hold {
                            req.insert_header("x-ms-legal-hold", &x_ms_legal_hold.to_string());
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_copy_source_tag_option) = &this.x_ms_copy_source_tag_option {
                            req.insert_header("x-ms-copy-source-tag-option", x_ms_copy_source_tag_option);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod abort_copy_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_copy_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=copy&copyid",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-copy-action", &this.x_ms_copy_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_tier {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_access_tier: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) versionid: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_rehydrate_priority: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate on. It's for service version 2019-10-10 and newer."]
            pub fn versionid(mut self, versionid: impl Into<String>) -> Self {
                self.versionid = Some(versionid.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional: Indicates the priority with which to rehydrate an archived blob."]
            pub fn x_ms_rehydrate_priority(mut self, x_ms_rehydrate_priority: impl Into<String>) -> Self {
                self.x_ms_rehydrate_priority = Some(x_ms_rehydrate_priority.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=tier",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        if let Some(versionid) = &this.versionid {
                            req.url_mut().query_pairs_mut().append_pair("versionid", versionid);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-access-tier", &this.x_ms_access_tier);
                        if let Some(x_ms_rehydrate_priority) = &this.x_ms_rehydrate_priority {
                            req.insert_header("x-ms-rehydrate-priority", x_ms_rehydrate_priority);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_account_info {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=account&comp=properties",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod query {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "The number of bytes present in the response body."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "The media type of the body of the response. For Download Blob this is 'application/octet-stream'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "Indicates the range of bytes returned in the event that the client requested a subset of the blob by setting the 'Range' request header."]
            pub fn content_range(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-range"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "If the blob has an MD5 hash and this operation is to read the full blob, this response header is returned so that the client can check for message content integrity."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "This header returns the value that was specified for the Content-Encoding request header"]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "This header is returned if it was previously specified for the blob."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "This header returns the value that was specified for the 'x-ms-blob-content-disposition' header. The Content-Disposition response header field conveys additional information about how to process the response payload, and also can be used to attach additional metadata. For example, if set to attachment, it indicates that the user-agent should not display the response, but instead show a Save As dialog with a filename other than the blob name specified."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "This header returns the value that was specified for the Content-Language request header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "The current sequence number for a page blob. This header is not returned for block blobs or append blobs"]
            pub fn x_ms_blob_sequence_number(&self) -> azure_core::Result<i64> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-sequence-number"))
            }
            #[doc = "The blob's type."]
            pub fn x_ms_blob_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-blob-type"))
            }
            #[doc = "Conclusion time of the last attempted Copy Blob operation where this blob was the destination blob. This value can specify the time of a completed, aborted, or failed copy attempt. This header does not appear if a copy is pending, if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List."]
            pub fn x_ms_copy_completion_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-completion-time"))?,
                )
            }
            #[doc = "Only appears when x-ms-copy-status is failed or pending. Describes the cause of the last fatal or non-fatal copy operation failure. This header does not appear if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List"]
            pub fn x_ms_copy_status_description(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status-description"))
            }
            #[doc = "String identifier for this copy operation. Use with Get Blob Properties to check the status of this copy operation, or pass to Abort Copy Blob to abort a pending copy."]
            pub fn x_ms_copy_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-id"))
            }
            #[doc = "Contains the number of bytes copied and the total bytes in the source in the last attempted Copy Blob operation where this blob was the destination blob. Can show between 0 and Content-Length bytes copied. This header does not appear if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List"]
            pub fn x_ms_copy_progress(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-progress"))
            }
            #[doc = "URL up to 2 KB in length that specifies the source blob or file used in the last attempted Copy Blob operation where this blob was the destination blob. This header does not appear if this blob has never been the destination in a Copy Blob operation, or if this blob has been modified after a concluded Copy Blob operation using Set Blob Properties, Put Blob, or Put Block List."]
            pub fn x_ms_copy_source(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-source"))
            }
            #[doc = "State of the copy operation identified by x-ms-copy-id."]
            pub fn x_ms_copy_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status"))
            }
            #[doc = "When a blob is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the blob."]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The current lease status of the blob."]
            pub fn x_ms_lease_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-status"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "Indicates that the service supports requests for partial blob content."]
            pub fn accept_ranges(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("accept-ranges"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The number of committed blocks present in the blob. This header is returned only for append blobs."]
            pub fn x_ms_blob_committed_block_count(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-committed-block-count"))
            }
            #[doc = "The value of this header is set to true if the blob data and application metadata are completely encrypted using the specified algorithm. Otherwise, the value is set to false (when the blob is unencrypted, or if only parts of the blob/application metadata are encrypted)."]
            pub fn x_ms_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-server-encrypted"))
            }
            #[doc = "The SHA-256 hash of the encryption key used to encrypt the blob. This header is only returned when the blob was encrypted with a customer-provided key."]
            pub fn x_ms_encryption_key_sha256(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-key-sha256"))
            }
            #[doc = "Returns the name of the encryption scope used to encrypt the blob contents and application metadata.  Note that the absence of this header implies use of the default account encryption scope."]
            pub fn x_ms_encryption_scope(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-scope"))
            }
            #[doc = "If the blob has a MD5 hash, and if request contains range header (Range or x-ms-range), this response header is returned with the value of the whole blob's MD5 value. This value may or may not be equal to the value returned in Content-MD5 header, with the latter calculated from the requested range"]
            pub fn x_ms_blob_content_md5(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-blob-content-md5"))
            }
            #[doc = "If the request is to read a specified range and the x-ms-range-get-content-crc64 is set to true, then the request returns a crc64 for the range, as long as the range size is less than or equal to 4 MB. If both x-ms-range-get-content-crc64 and x-ms-range-get-content-md5 is specified in the same request, it will fail with 400(Bad Request)"]
            pub fn x_ms_content_crc64(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-content-crc64"))
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) query_request: Option<models::QueryRequest>,
            pub(crate) snapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "the query request"]
            pub fn query_request(mut self, query_request: impl Into<models::QueryRequest>) -> Self {
                self.query_request = Some(query_request.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=query",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let req_body = if let Some(query_request) = &this.query_request {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(query_request)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<bytes::Bytes>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_tags {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BlobTags> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BlobTags = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) snapshot: Option<String>,
            pub(crate) versionid: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate on. It's for service version 2019-10-10 and newer."]
            pub fn versionid(mut self, versionid: impl Into<String>) -> Self {
                self.versionid = Some(versionid.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=tags",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        if let Some(versionid) = &this.versionid {
                            req.url_mut().query_pairs_mut().append_pair("versionid", versionid);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::BlobTags>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod set_tags {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) versionid: Option<String>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_content_crc64: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) tags: Option<models::BlobTags>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate on. It's for service version 2019-10-10 and newer."]
            pub fn versionid(mut self, versionid: impl Into<String>) -> Self {
                self.versionid = Some(versionid.into());
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Specify the transactional crc64 for the body, to be validated by the service."]
            pub fn x_ms_content_crc64(mut self, x_ms_content_crc64: impl Into<String>) -> Self {
                self.x_ms_content_crc64 = Some(x_ms_content_crc64.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Blob tags"]
            pub fn tags(mut self, tags: impl Into<models::BlobTags>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=tags",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(versionid) = &this.versionid {
                            req.url_mut().query_pairs_mut().append_pair("versionid", versionid);
                        }
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_content_crc64) = &this.x_ms_content_crc64 {
                            req.insert_header("x-ms-content-crc64", x_ms_content_crc64);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = if let Some(tags) = &this.tags {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(tags)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod page_blob {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The Create operation creates a new page blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_blob_type`: Specifies the type of blob to create: block blob, page blob, or append blob."]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `x_ms_blob_content_length`: This header specifies the maximum size for the page blob, up to 1 TB. The page blob size must be aligned to a 512-byte boundary."]
        pub fn create(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_blob_type: impl Into<String>,
            content_length: i64,
            x_ms_blob_content_length: i64,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_blob_type: x_ms_blob_type.into(),
                content_length,
                x_ms_blob_content_length,
                timeout: None,
                x_ms_access_tier: None,
                x_ms_blob_content_type: None,
                x_ms_blob_content_encoding: None,
                x_ms_blob_content_language: None,
                x_ms_blob_content_md5: None,
                x_ms_blob_cache_control: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_blob_content_disposition: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_blob_sequence_number: None,
                x_ms_client_request_id: None,
                x_ms_tags: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
                x_ms_legal_hold: None,
            }
        }
        #[doc = "The Upload Pages operation writes a range of pages to a page blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_page_write`: Required. You may specify one of the following options:\n  - Update: Writes the bytes specified by the request body into the specified range. The Range and Content-Length headers must match to perform the update.\n  - Clear: Clears the specified range and releases the space used in storage for that range. To clear a range, set the Content-Length header to zero, and the Range header to a value that indicates the range to clear, up to maximum blob size."]
        #[doc = "* `body`: Initial data"]
        #[doc = "* `content_length`: The length of the request."]
        pub fn upload_pages(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_page_write: impl Into<String>,
            body: impl Into<serde_json::Value>,
            content_length: i64,
        ) -> upload_pages::RequestBuilder {
            upload_pages::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_page_write: x_ms_page_write.into(),
                body: body.into(),
                content_length,
                content_md5: None,
                x_ms_content_crc64: None,
                timeout: None,
                x_ms_range: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_if_sequence_number_le: None,
                x_ms_if_sequence_number_lt: None,
                x_ms_if_sequence_number_eq: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Clear Pages operation clears a set of pages from a page blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_page_write`: Required. You may specify one of the following options:\n  - Update: Writes the bytes specified by the request body into the specified range. The Range and Content-Length headers must match to perform the update.\n  - Clear: Clears the specified range and releases the space used in storage for that range. To clear a range, set the Content-Length header to zero, and the Range header to a value that indicates the range to clear, up to maximum blob size."]
        #[doc = "* `content_length`: The length of the request."]
        pub fn clear_pages(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_page_write: impl Into<String>,
            content_length: i64,
        ) -> clear_pages::RequestBuilder {
            clear_pages::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_page_write: x_ms_page_write.into(),
                content_length,
                timeout: None,
                x_ms_range: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_if_sequence_number_le: None,
                x_ms_if_sequence_number_lt: None,
                x_ms_if_sequence_number_eq: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Upload Pages operation writes a range of pages to a page blob where the contents are read from a URL"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_page_write`: Required. You may specify one of the following options:\n  - Update: Writes the bytes specified by the request body into the specified range. The Range and Content-Length headers must match to perform the update.\n  - Clear: Clears the specified range and releases the space used in storage for that range. To clear a range, set the Content-Length header to zero, and the Range header to a value that indicates the range to clear, up to maximum blob size."]
        #[doc = "* `x_ms_copy_source`: Specify a URL to the copy source."]
        #[doc = "* `x_ms_source_range`: Bytes of source data in the specified range. The length of this range should match the ContentLength header and x-ms-range/Range destination range header."]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `x_ms_range`: The range of bytes to which the source range would be written. The range should be 512 aligned and range-end is required."]
        pub fn upload_pages_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_page_write: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
            x_ms_source_range: impl Into<String>,
            content_length: i64,
            x_ms_range: impl Into<String>,
        ) -> upload_pages_from_url::RequestBuilder {
            upload_pages_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_page_write: x_ms_page_write.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                x_ms_source_range: x_ms_source_range.into(),
                content_length,
                x_ms_range: x_ms_range.into(),
                x_ms_source_content_md5: None,
                x_ms_source_content_crc64: None,
                timeout: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_lease_id: None,
                x_ms_if_sequence_number_le: None,
                x_ms_if_sequence_number_lt: None,
                x_ms_if_sequence_number_eq: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                x_ms_client_request_id: None,
                x_ms_copy_source_authorization: None,
            }
        }
        #[doc = "The Get Page Ranges operation returns the list of valid page ranges for a page blob or snapshot of a page blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn get_page_ranges(&self, container_name: impl Into<String>, blob: impl Into<String>) -> get_page_ranges::RequestBuilder {
            get_page_ranges::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                snapshot: None,
                timeout: None,
                x_ms_range: None,
                x_ms_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
                marker: None,
                maxresults: None,
            }
        }
        #[doc = "The Get Page Ranges Diff operation returns the list of valid page ranges for a page blob that were changed between target blob and previous snapshot."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn get_page_ranges_diff(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
        ) -> get_page_ranges_diff::RequestBuilder {
            get_page_ranges_diff::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                snapshot: None,
                timeout: None,
                prevsnapshot: None,
                x_ms_previous_snapshot_url: None,
                x_ms_range: None,
                x_ms_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
                marker: None,
                maxresults: None,
            }
        }
        #[doc = "Resize the Blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_blob_content_length`: This header specifies the maximum size for the page blob, up to 1 TB. The page blob size must be aligned to a 512-byte boundary."]
        pub fn resize(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_blob_content_length: i64,
        ) -> resize::RequestBuilder {
            resize::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_blob_content_length,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Update the sequence number of the blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_sequence_number_action`: Required if the x-ms-blob-sequence-number header is set for the request. This property applies to page blobs only. This property indicates how the service should modify the blob's sequence number"]
        pub fn update_sequence_number(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_sequence_number_action: impl Into<String>,
        ) -> update_sequence_number::RequestBuilder {
            update_sequence_number::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_sequence_number_action: x_ms_sequence_number_action.into(),
                timeout: None,
                x_ms_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_blob_sequence_number: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Copy Incremental operation copies a snapshot of the source page blob to a destination page blob. The snapshot is copied such that only the differential changes between the previously copied snapshot are transferred to the destination. The copied snapshots are complete copies of the original snapshot and can be read or copied from as usual. This API is supported since REST version 2016-05-31."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_copy_source`: Specifies the name of the source page blob snapshot. This value is a URL of up to 2 KB in length that specifies a page blob snapshot. The value should be URL-encoded as it would appear in a request URI. The source blob must either be public or must be authenticated via a shared access signature."]
        pub fn copy_incremental(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
        ) -> copy_incremental::RequestBuilder {
            copy_incremental::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_blob_type: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_blob_content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_blob_content_type: Option<String>,
            pub(crate) x_ms_blob_content_encoding: Option<String>,
            pub(crate) x_ms_blob_content_language: Option<String>,
            pub(crate) x_ms_blob_content_md5: Option<String>,
            pub(crate) x_ms_blob_cache_control: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_content_disposition: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_blob_sequence_number: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
            pub(crate) x_ms_legal_hold: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Indicates the tier to be set on the page blob."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_type(mut self, x_ms_blob_content_type: impl Into<String>) -> Self {
                self.x_ms_blob_content_type = Some(x_ms_blob_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_encoding(mut self, x_ms_blob_content_encoding: impl Into<String>) -> Self {
                self.x_ms_blob_content_encoding = Some(x_ms_blob_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_language(mut self, x_ms_blob_content_language: impl Into<String>) -> Self {
                self.x_ms_blob_content_language = Some(x_ms_blob_content_language.into());
                self
            }
            #[doc = "Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks were validated when each was uploaded."]
            pub fn x_ms_blob_content_md5(mut self, x_ms_blob_content_md5: impl Into<String>) -> Self {
                self.x_ms_blob_content_md5 = Some(x_ms_blob_content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_cache_control(mut self, x_ms_blob_cache_control: impl Into<String>) -> Self {
                self.x_ms_blob_cache_control = Some(x_ms_blob_cache_control.into());
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_blob_content_disposition(mut self, x_ms_blob_content_disposition: impl Into<String>) -> Self {
                self.x_ms_blob_content_disposition = Some(x_ms_blob_content_disposition.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Set for page blobs only. The sequence number is a user-controlled value that you can use to track requests. The value of the sequence number must be between 0 and 2^63 - 1."]
            pub fn x_ms_blob_sequence_number(mut self, x_ms_blob_sequence_number: i64) -> Self {
                self.x_ms_blob_sequence_number = Some(x_ms_blob_sequence_number);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Specified if a legal hold should be set on the blob."]
            pub fn x_ms_legal_hold(mut self, x_ms_legal_hold: bool) -> Self {
                self.x_ms_legal_hold = Some(x_ms_legal_hold);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?PageBlob",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-blob-type", &this.x_ms_blob_type);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(x_ms_blob_content_type) = &this.x_ms_blob_content_type {
                            req.insert_header("x-ms-blob-content-type", x_ms_blob_content_type);
                        }
                        if let Some(x_ms_blob_content_encoding) = &this.x_ms_blob_content_encoding {
                            req.insert_header("x-ms-blob-content-encoding", x_ms_blob_content_encoding);
                        }
                        if let Some(x_ms_blob_content_language) = &this.x_ms_blob_content_language {
                            req.insert_header("x-ms-blob-content-language", x_ms_blob_content_language);
                        }
                        if let Some(x_ms_blob_content_md5) = &this.x_ms_blob_content_md5 {
                            req.insert_header("x-ms-blob-content-md5", x_ms_blob_content_md5);
                        }
                        if let Some(x_ms_blob_cache_control) = &this.x_ms_blob_cache_control {
                            req.insert_header("x-ms-blob-cache-control", x_ms_blob_cache_control);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_content_disposition) = &this.x_ms_blob_content_disposition {
                            req.insert_header("x-ms-blob-content-disposition", x_ms_blob_content_disposition);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("x-ms-blob-content-length", &this.x_ms_blob_content_length.to_string());
                        if let Some(x_ms_blob_sequence_number) = &this.x_ms_blob_sequence_number {
                            req.insert_header("x-ms-blob-sequence-number", &x_ms_blob_sequence_number.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        if let Some(x_ms_legal_hold) = &this.x_ms_legal_hold {
                            req.insert_header("x-ms-legal-hold", &x_ms_legal_hold.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod upload_pages {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_page_write: String,
            pub(crate) body: serde_json::Value,
            pub(crate) content_length: i64,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_content_crc64: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_if_sequence_number_le: Option<i64>,
            pub(crate) x_ms_if_sequence_number_lt: Option<i64>,
            pub(crate) x_ms_if_sequence_number_eq: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Specify the transactional crc64 for the body, to be validated by the service."]
            pub fn x_ms_content_crc64(mut self, x_ms_content_crc64: impl Into<String>) -> Self {
                self.x_ms_content_crc64 = Some(x_ms_content_crc64.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Return only the bytes of the blob in the specified range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has a sequence number less than or equal to the specified."]
            pub fn x_ms_if_sequence_number_le(mut self, x_ms_if_sequence_number_le: i64) -> Self {
                self.x_ms_if_sequence_number_le = Some(x_ms_if_sequence_number_le);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has a sequence number less than the specified."]
            pub fn x_ms_if_sequence_number_lt(mut self, x_ms_if_sequence_number_lt: i64) -> Self {
                self.x_ms_if_sequence_number_lt = Some(x_ms_if_sequence_number_lt);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has the specified sequence number."]
            pub fn x_ms_if_sequence_number_eq(mut self, x_ms_if_sequence_number_eq: i64) -> Self {
                self.x_ms_if_sequence_number_eq = Some(x_ms_if_sequence_number_eq);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=page&update",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-page-write", &this.x_ms_page_write);
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_content_crc64) = &this.x_ms_content_crc64 {
                            req.insert_header("x-ms-content-crc64", x_ms_content_crc64);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_if_sequence_number_le) = &this.x_ms_if_sequence_number_le {
                            req.insert_header("x-ms-if-sequence-number-le", &x_ms_if_sequence_number_le.to_string());
                        }
                        if let Some(x_ms_if_sequence_number_lt) = &this.x_ms_if_sequence_number_lt {
                            req.insert_header("x-ms-if-sequence-number-lt", &x_ms_if_sequence_number_lt.to_string());
                        }
                        if let Some(x_ms_if_sequence_number_eq) = &this.x_ms_if_sequence_number_eq {
                            req.insert_header("x-ms-if-sequence-number-eq", &x_ms_if_sequence_number_eq.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod clear_pages {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_page_write: String,
            pub(crate) content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_if_sequence_number_le: Option<i64>,
            pub(crate) x_ms_if_sequence_number_lt: Option<i64>,
            pub(crate) x_ms_if_sequence_number_eq: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Return only the bytes of the blob in the specified range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has a sequence number less than or equal to the specified."]
            pub fn x_ms_if_sequence_number_le(mut self, x_ms_if_sequence_number_le: i64) -> Self {
                self.x_ms_if_sequence_number_le = Some(x_ms_if_sequence_number_le);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has a sequence number less than the specified."]
            pub fn x_ms_if_sequence_number_lt(mut self, x_ms_if_sequence_number_lt: i64) -> Self {
                self.x_ms_if_sequence_number_lt = Some(x_ms_if_sequence_number_lt);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has the specified sequence number."]
            pub fn x_ms_if_sequence_number_eq(mut self, x_ms_if_sequence_number_eq: i64) -> Self {
                self.x_ms_if_sequence_number_eq = Some(x_ms_if_sequence_number_eq);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=page&clear",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-page-write", &this.x_ms_page_write);
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_if_sequence_number_le) = &this.x_ms_if_sequence_number_le {
                            req.insert_header("x-ms-if-sequence-number-le", &x_ms_if_sequence_number_le.to_string());
                        }
                        if let Some(x_ms_if_sequence_number_lt) = &this.x_ms_if_sequence_number_lt {
                            req.insert_header("x-ms-if-sequence-number-lt", &x_ms_if_sequence_number_lt.to_string());
                        }
                        if let Some(x_ms_if_sequence_number_eq) = &this.x_ms_if_sequence_number_eq {
                            req.insert_header("x-ms-if-sequence-number-eq", &x_ms_if_sequence_number_eq.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod upload_pages_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_page_write: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) x_ms_source_range: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_range: String,
            pub(crate) x_ms_source_content_md5: Option<String>,
            pub(crate) x_ms_source_content_crc64: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_if_sequence_number_le: Option<i64>,
            pub(crate) x_ms_if_sequence_number_lt: Option<i64>,
            pub(crate) x_ms_if_sequence_number_eq: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Specify the md5 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_md5(mut self, x_ms_source_content_md5: impl Into<String>) -> Self {
                self.x_ms_source_content_md5 = Some(x_ms_source_content_md5.into());
                self
            }
            #[doc = "Specify the crc64 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_crc64(mut self, x_ms_source_content_crc64: impl Into<String>) -> Self {
                self.x_ms_source_content_crc64 = Some(x_ms_source_content_crc64.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has a sequence number less than or equal to the specified."]
            pub fn x_ms_if_sequence_number_le(mut self, x_ms_if_sequence_number_le: i64) -> Self {
                self.x_ms_if_sequence_number_le = Some(x_ms_if_sequence_number_le);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has a sequence number less than the specified."]
            pub fn x_ms_if_sequence_number_lt(mut self, x_ms_if_sequence_number_lt: i64) -> Self {
                self.x_ms_if_sequence_number_lt = Some(x_ms_if_sequence_number_lt);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has the specified sequence number."]
            pub fn x_ms_if_sequence_number_eq(mut self, x_ms_if_sequence_number_eq: i64) -> Self {
                self.x_ms_if_sequence_number_eq = Some(x_ms_if_sequence_number_eq);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=page&update&fromUrl",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-page-write", &this.x_ms_page_write);
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        req.insert_header("x-ms-source-range", &this.x_ms_source_range);
                        if let Some(x_ms_source_content_md5) = &this.x_ms_source_content_md5 {
                            req.insert_header("x-ms-source-content-md5", x_ms_source_content_md5);
                        }
                        if let Some(x_ms_source_content_crc64) = &this.x_ms_source_content_crc64 {
                            req.insert_header("x-ms-source-content-crc64", x_ms_source_content_crc64);
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-range", &this.x_ms_range);
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_if_sequence_number_le) = &this.x_ms_if_sequence_number_le {
                            req.insert_header("x-ms-if-sequence-number-le", &x_ms_if_sequence_number_le.to_string());
                        }
                        if let Some(x_ms_if_sequence_number_lt) = &this.x_ms_if_sequence_number_lt {
                            req.insert_header("x-ms-if-sequence-number-lt", &x_ms_if_sequence_number_lt.to_string());
                        }
                        if let Some(x_ms_if_sequence_number_eq) = &this.x_ms_if_sequence_number_eq {
                            req.insert_header("x-ms-if-sequence-number-eq", &x_ms_if_sequence_number_eq.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_page_ranges {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageList = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The size of the blob in bytes."]
            pub fn x_ms_blob_content_length(&self) -> azure_core::Result<i64> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-content-length"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Return only the bytes of the blob in the specified range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=pagelist",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
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
                                req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                                if let Some(snapshot) = &this.snapshot {
                                    req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(x_ms_range) = &this.x_ms_range {
                                    req.insert_header("x-ms-range", x_ms_range);
                                }
                                if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                                    req.insert_header("x-ms-lease-id", x_ms_lease_id);
                                }
                                if let Some(if_modified_since) = &this.if_modified_since {
                                    req.insert_header("if-modified-since", &if_modified_since.to_string());
                                }
                                if let Some(if_unmodified_since) = &this.if_unmodified_since {
                                    req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                                }
                                if let Some(if_match) = &this.if_match {
                                    req.insert_header("if-match", if_match);
                                }
                                if let Some(if_none_match) = &this.if_none_match {
                                    req.insert_header("if-none-match", if_none_match);
                                }
                                if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                                    req.insert_header("x-ms-if-tags", x_ms_if_tags);
                                }
                                if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                                    req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
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
    pub mod get_page_ranges_diff {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageList = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The size of the blob in bytes."]
            pub fn x_ms_blob_content_length(&self) -> azure_core::Result<i64> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-content-length"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) prevsnapshot: Option<String>,
            pub(crate) x_ms_previous_snapshot_url: Option<String>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional in version 2015-07-08 and newer. The prevsnapshot parameter is a DateTime value that specifies that the response will contain only pages that were changed between target blob and previous snapshot. Changed pages include both updated and cleared pages. The target blob may be a snapshot, as long as the snapshot specified by prevsnapshot is the older of the two. Note that incremental snapshots are currently supported only for blobs created on or after January 1, 2016."]
            pub fn prevsnapshot(mut self, prevsnapshot: impl Into<String>) -> Self {
                self.prevsnapshot = Some(prevsnapshot.into());
                self
            }
            #[doc = "Optional. This header is only supported in service versions 2019-04-19 and after and specifies the URL of a previous snapshot of the target blob. The response will only contain pages that were changed between the target blob and its previous snapshot."]
            pub fn x_ms_previous_snapshot_url(mut self, x_ms_previous_snapshot_url: impl Into<String>) -> Self {
                self.x_ms_previous_snapshot_url = Some(x_ms_previous_snapshot_url.into());
                self
            }
            #[doc = "Return only the bytes of the blob in the specified range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of containers to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=pagelist&diff",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
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
                                req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                                if let Some(snapshot) = &this.snapshot {
                                    req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(prevsnapshot) = &this.prevsnapshot {
                                    req.url_mut().query_pairs_mut().append_pair("prevsnapshot", prevsnapshot);
                                }
                                if let Some(x_ms_previous_snapshot_url) = &this.x_ms_previous_snapshot_url {
                                    req.insert_header("x-ms-previous-snapshot-url", x_ms_previous_snapshot_url);
                                }
                                if let Some(x_ms_range) = &this.x_ms_range {
                                    req.insert_header("x-ms-range", x_ms_range);
                                }
                                if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                                    req.insert_header("x-ms-lease-id", x_ms_lease_id);
                                }
                                if let Some(if_modified_since) = &this.if_modified_since {
                                    req.insert_header("if-modified-since", &if_modified_since.to_string());
                                }
                                if let Some(if_unmodified_since) = &this.if_unmodified_since {
                                    req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                                }
                                if let Some(if_match) = &this.if_match {
                                    req.insert_header("if-match", if_match);
                                }
                                if let Some(if_none_match) = &this.if_none_match {
                                    req.insert_header("if-none-match", if_none_match);
                                }
                                if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                                    req.insert_header("x-ms-if-tags", x_ms_if_tags);
                                }
                                if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                                    req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
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
    pub mod resize {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_blob_content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=properties&Resize",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("x-ms-blob-content-length", &this.x_ms_blob_content_length.to_string());
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod update_sequence_number {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_sequence_number_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_blob_sequence_number: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Set for page blobs only. The sequence number is a user-controlled value that you can use to track requests. The value of the sequence number must be between 0 and 2^63 - 1."]
            pub fn x_ms_blob_sequence_number(mut self, x_ms_blob_sequence_number: i64) -> Self {
                self.x_ms_blob_sequence_number = Some(x_ms_blob_sequence_number);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=properties&UpdateSequenceNumber",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("x-ms-sequence-number-action", &this.x_ms_sequence_number_action);
                        if let Some(x_ms_blob_sequence_number) = &this.x_ms_blob_sequence_number {
                            req.insert_header("x-ms-blob-sequence-number", &x_ms_blob_sequence_number.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod copy_incremental {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=incrementalcopy",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod append_blob {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The Create Append Blob operation creates a new append blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_blob_type`: Specifies the type of blob to create: block blob, page blob, or append blob."]
        #[doc = "* `content_length`: The length of the request."]
        pub fn create(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_blob_type: impl Into<String>,
            content_length: i64,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_blob_type: x_ms_blob_type.into(),
                content_length,
                timeout: None,
                x_ms_blob_content_type: None,
                x_ms_blob_content_encoding: None,
                x_ms_blob_content_language: None,
                x_ms_blob_content_md5: None,
                x_ms_blob_cache_control: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_blob_content_disposition: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
                x_ms_tags: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
                x_ms_legal_hold: None,
            }
        }
        #[doc = "The Append Block operation commits a new block of data to the end of an existing append blob. The Append Block operation is permitted only if the blob was created with x-ms-blob-type set to AppendBlob. Append Block is supported only on version 2015-02-21 version or later."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `body`: Initial data"]
        #[doc = "* `content_length`: The length of the request."]
        pub fn append_block(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            body: impl Into<serde_json::Value>,
            content_length: i64,
        ) -> append_block::RequestBuilder {
            append_block::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                body: body.into(),
                content_length,
                timeout: None,
                content_md5: None,
                x_ms_content_crc64: None,
                x_ms_lease_id: None,
                x_ms_blob_condition_maxsize: None,
                x_ms_blob_condition_appendpos: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Append Block operation commits a new block of data to the end of an existing append blob where the contents are read from a source url. The Append Block operation is permitted only if the blob was created with x-ms-blob-type set to AppendBlob. Append Block is supported only on version 2015-02-21 version or later."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_copy_source`: Specify a URL to the copy source."]
        #[doc = "* `content_length`: The length of the request."]
        pub fn append_block_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
            content_length: i64,
        ) -> append_block_from_url::RequestBuilder {
            append_block_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                content_length,
                x_ms_source_range: None,
                x_ms_source_content_md5: None,
                x_ms_source_content_crc64: None,
                timeout: None,
                content_md5: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_lease_id: None,
                x_ms_blob_condition_maxsize: None,
                x_ms_blob_condition_appendpos: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                x_ms_client_request_id: None,
                x_ms_copy_source_authorization: None,
            }
        }
        #[doc = "The Seal operation seals the Append Blob to make it read-only. Seal is supported only on version 2019-12-12 version or later."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        pub fn seal(&self, container_name: impl Into<String>, blob: impl Into<String>) -> seal::RequestBuilder {
            seal::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_lease_id: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_blob_condition_appendpos: None,
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_blob_type: String,
            pub(crate) content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_blob_content_type: Option<String>,
            pub(crate) x_ms_blob_content_encoding: Option<String>,
            pub(crate) x_ms_blob_content_language: Option<String>,
            pub(crate) x_ms_blob_content_md5: Option<String>,
            pub(crate) x_ms_blob_cache_control: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_content_disposition: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
            pub(crate) x_ms_legal_hold: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_type(mut self, x_ms_blob_content_type: impl Into<String>) -> Self {
                self.x_ms_blob_content_type = Some(x_ms_blob_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_encoding(mut self, x_ms_blob_content_encoding: impl Into<String>) -> Self {
                self.x_ms_blob_content_encoding = Some(x_ms_blob_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_language(mut self, x_ms_blob_content_language: impl Into<String>) -> Self {
                self.x_ms_blob_content_language = Some(x_ms_blob_content_language.into());
                self
            }
            #[doc = "Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks were validated when each was uploaded."]
            pub fn x_ms_blob_content_md5(mut self, x_ms_blob_content_md5: impl Into<String>) -> Self {
                self.x_ms_blob_content_md5 = Some(x_ms_blob_content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_cache_control(mut self, x_ms_blob_cache_control: impl Into<String>) -> Self {
                self.x_ms_blob_cache_control = Some(x_ms_blob_cache_control.into());
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_blob_content_disposition(mut self, x_ms_blob_content_disposition: impl Into<String>) -> Self {
                self.x_ms_blob_content_disposition = Some(x_ms_blob_content_disposition.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Specified if a legal hold should be set on the blob."]
            pub fn x_ms_legal_hold(mut self, x_ms_legal_hold: bool) -> Self {
                self.x_ms_legal_hold = Some(x_ms_legal_hold);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?AppendBlob",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-blob-type", &this.x_ms_blob_type);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(x_ms_blob_content_type) = &this.x_ms_blob_content_type {
                            req.insert_header("x-ms-blob-content-type", x_ms_blob_content_type);
                        }
                        if let Some(x_ms_blob_content_encoding) = &this.x_ms_blob_content_encoding {
                            req.insert_header("x-ms-blob-content-encoding", x_ms_blob_content_encoding);
                        }
                        if let Some(x_ms_blob_content_language) = &this.x_ms_blob_content_language {
                            req.insert_header("x-ms-blob-content-language", x_ms_blob_content_language);
                        }
                        if let Some(x_ms_blob_content_md5) = &this.x_ms_blob_content_md5 {
                            req.insert_header("x-ms-blob-content-md5", x_ms_blob_content_md5);
                        }
                        if let Some(x_ms_blob_cache_control) = &this.x_ms_blob_cache_control {
                            req.insert_header("x-ms-blob-cache-control", x_ms_blob_cache_control);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_content_disposition) = &this.x_ms_blob_content_disposition {
                            req.insert_header("x-ms-blob-content-disposition", x_ms_blob_content_disposition);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        if let Some(x_ms_legal_hold) = &this.x_ms_legal_hold {
                            req.insert_header("x-ms-legal-hold", &x_ms_legal_hold.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod append_block {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) body: serde_json::Value,
            pub(crate) content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_content_crc64: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_condition_maxsize: Option<i64>,
            pub(crate) x_ms_blob_condition_appendpos: Option<i64>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Specify the transactional crc64 for the body, to be validated by the service."]
            pub fn x_ms_content_crc64(mut self, x_ms_content_crc64: impl Into<String>) -> Self {
                self.x_ms_content_crc64 = Some(x_ms_content_crc64.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional conditional header. The max length in bytes permitted for the append blob. If the Append Block operation would cause the blob to exceed that limit or if the blob size is already greater than the value specified in this header, the request will fail with MaxBlobSizeConditionNotMet error (HTTP status code 412 - Precondition Failed)."]
            pub fn x_ms_blob_condition_maxsize(mut self, x_ms_blob_condition_maxsize: i64) -> Self {
                self.x_ms_blob_condition_maxsize = Some(x_ms_blob_condition_maxsize);
                self
            }
            #[doc = "Optional conditional header, used only for the Append Block operation. A number indicating the byte offset to compare. Append Block will succeed only if the append position is equal to this number. If it is not, the request will fail with the AppendPositionConditionNotMet error (HTTP status code 412 - Precondition Failed)."]
            pub fn x_ms_blob_condition_appendpos(mut self, x_ms_blob_condition_appendpos: i64) -> Self {
                self.x_ms_blob_condition_appendpos = Some(x_ms_blob_condition_appendpos);
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=appendblock",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_content_crc64) = &this.x_ms_content_crc64 {
                            req.insert_header("x-ms-content-crc64", x_ms_content_crc64);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_condition_maxsize) = &this.x_ms_blob_condition_maxsize {
                            req.insert_header("x-ms-blob-condition-maxsize", &x_ms_blob_condition_maxsize.to_string());
                        }
                        if let Some(x_ms_blob_condition_appendpos) = &this.x_ms_blob_condition_appendpos {
                            req.insert_header("x-ms-blob-condition-appendpos", &x_ms_blob_condition_appendpos.to_string());
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod append_block_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_source_range: Option<String>,
            pub(crate) x_ms_source_content_md5: Option<String>,
            pub(crate) x_ms_source_content_crc64: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_condition_maxsize: Option<i64>,
            pub(crate) x_ms_blob_condition_appendpos: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Bytes of source data in the specified range."]
            pub fn x_ms_source_range(mut self, x_ms_source_range: impl Into<String>) -> Self {
                self.x_ms_source_range = Some(x_ms_source_range.into());
                self
            }
            #[doc = "Specify the md5 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_md5(mut self, x_ms_source_content_md5: impl Into<String>) -> Self {
                self.x_ms_source_content_md5 = Some(x_ms_source_content_md5.into());
                self
            }
            #[doc = "Specify the crc64 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_crc64(mut self, x_ms_source_content_crc64: impl Into<String>) -> Self {
                self.x_ms_source_content_crc64 = Some(x_ms_source_content_crc64.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional conditional header. The max length in bytes permitted for the append blob. If the Append Block operation would cause the blob to exceed that limit or if the blob size is already greater than the value specified in this header, the request will fail with MaxBlobSizeConditionNotMet error (HTTP status code 412 - Precondition Failed)."]
            pub fn x_ms_blob_condition_maxsize(mut self, x_ms_blob_condition_maxsize: i64) -> Self {
                self.x_ms_blob_condition_maxsize = Some(x_ms_blob_condition_maxsize);
                self
            }
            #[doc = "Optional conditional header, used only for the Append Block operation. A number indicating the byte offset to compare. Append Block will succeed only if the append position is equal to this number. If it is not, the request will fail with the AppendPositionConditionNotMet error (HTTP status code 412 - Precondition Failed)."]
            pub fn x_ms_blob_condition_appendpos(mut self, x_ms_blob_condition_appendpos: i64) -> Self {
                self.x_ms_blob_condition_appendpos = Some(x_ms_blob_condition_appendpos);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=appendblock&fromUrl",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_source_range) = &this.x_ms_source_range {
                            req.insert_header("x-ms-source-range", x_ms_source_range);
                        }
                        if let Some(x_ms_source_content_md5) = &this.x_ms_source_content_md5 {
                            req.insert_header("x-ms-source-content-md5", x_ms_source_content_md5);
                        }
                        if let Some(x_ms_source_content_crc64) = &this.x_ms_source_content_crc64 {
                            req.insert_header("x-ms-source-content-crc64", x_ms_source_content_crc64);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_condition_maxsize) = &this.x_ms_blob_condition_maxsize {
                            req.insert_header("x-ms-blob-condition-maxsize", &x_ms_blob_condition_maxsize.to_string());
                        }
                        if let Some(x_ms_blob_condition_appendpos) = &this.x_ms_blob_condition_appendpos {
                            req.insert_header("x-ms-blob-condition-appendpos", &x_ms_blob_condition_appendpos.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod seal {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_blob_condition_appendpos: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Optional conditional header, used only for the Append Block operation. A number indicating the byte offset to compare. Append Block will succeed only if the append position is equal to this number. If it is not, the request will fail with the AppendPositionConditionNotMet error (HTTP status code 412 - Precondition Failed)."]
            pub fn x_ms_blob_condition_appendpos(mut self, x_ms_blob_condition_appendpos: i64) -> Self {
                self.x_ms_blob_condition_appendpos = Some(x_ms_blob_condition_appendpos);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=seal",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_blob_condition_appendpos) = &this.x_ms_blob_condition_appendpos {
                            req.insert_header("x-ms-blob-condition-appendpos", &x_ms_blob_condition_appendpos.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod block_blob {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The Upload Block Blob operation updates the content of an existing block blob. Updating an existing block blob overwrites any existing metadata on the blob. Partial updates are not supported with Put Blob; the content of the existing blob is overwritten with the content of the new blob. To perform a partial update of the content of a block blob, use the Put Block List operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_blob_type`: Specifies the type of blob to create: block blob, page blob, or append blob."]
        #[doc = "* `body`: Initial data"]
        #[doc = "* `content_length`: The length of the request."]
        pub fn upload(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_blob_type: impl Into<String>,
            body: impl Into<serde_json::Value>,
            content_length: i64,
        ) -> upload::RequestBuilder {
            upload::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_blob_type: x_ms_blob_type.into(),
                body: body.into(),
                content_length,
                timeout: None,
                content_md5: None,
                x_ms_blob_content_type: None,
                x_ms_blob_content_encoding: None,
                x_ms_blob_content_language: None,
                x_ms_blob_content_md5: None,
                x_ms_blob_cache_control: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_blob_content_disposition: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_access_tier: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
                x_ms_tags: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
                x_ms_legal_hold: None,
            }
        }
        #[doc = "The Put Blob from URL operation creates a new Block Blob where the contents of the blob are read from a given URL.  This API is supported beginning with the 2020-04-08 version. Partial updates are not supported with Put Blob from URL; the content of an existing blob is overwritten with the content of the new blob.  To perform partial updates to a block blob’s contents using a source URL, use the Put Block from URL API in conjunction with Put Block List."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `x_ms_blob_type`: Specifies the type of blob to create: block blob, page blob, or append blob."]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `x_ms_copy_source`: Specifies the name of the source page blob snapshot. This value is a URL of up to 2 KB in length that specifies a page blob snapshot. The value should be URL-encoded as it would appear in a request URI. The source blob must either be public or must be authenticated via a shared access signature."]
        pub fn put_blob_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            x_ms_blob_type: impl Into<String>,
            content_length: i64,
            x_ms_copy_source: impl Into<String>,
        ) -> put_blob_from_url::RequestBuilder {
            put_blob_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                x_ms_blob_type: x_ms_blob_type.into(),
                content_length,
                x_ms_copy_source: x_ms_copy_source.into(),
                timeout: None,
                content_md5: None,
                x_ms_blob_content_type: None,
                x_ms_blob_content_encoding: None,
                x_ms_blob_content_language: None,
                x_ms_blob_content_md5: None,
                x_ms_blob_cache_control: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_blob_content_disposition: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_access_tier: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                x_ms_source_if_tags: None,
                x_ms_client_request_id: None,
                x_ms_source_content_md5: None,
                x_ms_tags: None,
                x_ms_copy_source_blob_properties: None,
                x_ms_copy_source_authorization: None,
                x_ms_copy_source_tag_option: None,
            }
        }
        #[doc = "The Stage Block operation creates a new block to be committed as part of a blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `blockid`: A valid Base64 string value that identifies the block. Prior to encoding, the string must be less than or equal to 64 bytes in size. For a given blob, the length of the value specified for the blockid parameter must be the same size for each block."]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `body`: Initial data"]
        pub fn stage_block(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            blockid: impl Into<String>,
            content_length: i64,
            body: impl Into<serde_json::Value>,
        ) -> stage_block::RequestBuilder {
            stage_block::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                blockid: blockid.into(),
                content_length,
                body: body.into(),
                content_md5: None,
                x_ms_content_crc64: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Stage Block operation creates a new block to be committed as part of a blob where the contents are read from a URL."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `blockid`: A valid Base64 string value that identifies the block. Prior to encoding, the string must be less than or equal to 64 bytes in size. For a given blob, the length of the value specified for the blockid parameter must be the same size for each block."]
        #[doc = "* `content_length`: The length of the request."]
        #[doc = "* `x_ms_copy_source`: Specify a URL to the copy source."]
        pub fn stage_block_from_url(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            blockid: impl Into<String>,
            content_length: i64,
            x_ms_copy_source: impl Into<String>,
        ) -> stage_block_from_url::RequestBuilder {
            stage_block_from_url::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                blockid: blockid.into(),
                content_length,
                x_ms_copy_source: x_ms_copy_source.into(),
                x_ms_source_range: None,
                x_ms_source_content_md5: None,
                x_ms_source_content_crc64: None,
                timeout: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_lease_id: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                x_ms_client_request_id: None,
                x_ms_copy_source_authorization: None,
            }
        }
        #[doc = "The Get Block List operation retrieves the list of blocks that have been uploaded as part of a block blob"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `blocklisttype`: Specifies whether to return the list of committed blocks, the list of uncommitted blocks, or both lists together."]
        pub fn get_block_list(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            blocklisttype: impl Into<String>,
        ) -> get_block_list::RequestBuilder {
            get_block_list::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                blocklisttype: blocklisttype.into(),
                snapshot: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Commit Block List operation writes a blob by specifying the list of block IDs that make up the blob. In order to be written as part of a blob, a block must have been successfully written to the server in a prior Put Block operation. You can call Put Block List to update a blob by uploading only those blocks that have changed, then committing the new and existing blocks together. You can do this by specifying whether to commit a block from the committed block list or from the uncommitted block list, or to commit the most recently uploaded version of the block, whichever list it may belong to."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `container_name`: The container name."]
        #[doc = "* `blob`: The blob name."]
        #[doc = "* `blocks`: Blob Blocks."]
        pub fn commit_block_list(
            &self,
            container_name: impl Into<String>,
            blob: impl Into<String>,
            blocks: impl Into<models::BlockLookupList>,
        ) -> commit_block_list::RequestBuilder {
            commit_block_list::RequestBuilder {
                client: self.0.clone(),
                container_name: container_name.into(),
                blob: blob.into(),
                blocks: blocks.into(),
                timeout: None,
                x_ms_blob_cache_control: None,
                x_ms_blob_content_type: None,
                x_ms_blob_content_encoding: None,
                x_ms_blob_content_language: None,
                x_ms_blob_content_md5: None,
                content_md5: None,
                x_ms_content_crc64: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_blob_content_disposition: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
                x_ms_encryption_scope: None,
                x_ms_access_tier: None,
                if_modified_since: None,
                if_unmodified_since: None,
                if_match: None,
                if_none_match: None,
                x_ms_if_tags: None,
                x_ms_client_request_id: None,
                x_ms_tags: None,
                x_ms_immutability_policy_until_date: None,
                x_ms_immutability_policy_mode: None,
                x_ms_legal_hold: None,
            }
        }
    }
    pub mod upload {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_blob_type: String,
            pub(crate) body: serde_json::Value,
            pub(crate) content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_blob_content_type: Option<String>,
            pub(crate) x_ms_blob_content_encoding: Option<String>,
            pub(crate) x_ms_blob_content_language: Option<String>,
            pub(crate) x_ms_blob_content_md5: Option<String>,
            pub(crate) x_ms_blob_cache_control: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_content_disposition: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
            pub(crate) x_ms_legal_hold: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_type(mut self, x_ms_blob_content_type: impl Into<String>) -> Self {
                self.x_ms_blob_content_type = Some(x_ms_blob_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_encoding(mut self, x_ms_blob_content_encoding: impl Into<String>) -> Self {
                self.x_ms_blob_content_encoding = Some(x_ms_blob_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_language(mut self, x_ms_blob_content_language: impl Into<String>) -> Self {
                self.x_ms_blob_content_language = Some(x_ms_blob_content_language.into());
                self
            }
            #[doc = "Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks were validated when each was uploaded."]
            pub fn x_ms_blob_content_md5(mut self, x_ms_blob_content_md5: impl Into<String>) -> Self {
                self.x_ms_blob_content_md5 = Some(x_ms_blob_content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_cache_control(mut self, x_ms_blob_cache_control: impl Into<String>) -> Self {
                self.x_ms_blob_cache_control = Some(x_ms_blob_cache_control.into());
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_blob_content_disposition(mut self, x_ms_blob_content_disposition: impl Into<String>) -> Self {
                self.x_ms_blob_content_disposition = Some(x_ms_blob_content_disposition.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Optional. Indicates the tier to be set on the blob."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Specified if a legal hold should be set on the blob."]
            pub fn x_ms_legal_hold(mut self, x_ms_legal_hold: bool) -> Self {
                self.x_ms_legal_hold = Some(x_ms_legal_hold);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?BlockBlob",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-blob-type", &this.x_ms_blob_type);
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(x_ms_blob_content_type) = &this.x_ms_blob_content_type {
                            req.insert_header("x-ms-blob-content-type", x_ms_blob_content_type);
                        }
                        if let Some(x_ms_blob_content_encoding) = &this.x_ms_blob_content_encoding {
                            req.insert_header("x-ms-blob-content-encoding", x_ms_blob_content_encoding);
                        }
                        if let Some(x_ms_blob_content_language) = &this.x_ms_blob_content_language {
                            req.insert_header("x-ms-blob-content-language", x_ms_blob_content_language);
                        }
                        if let Some(x_ms_blob_content_md5) = &this.x_ms_blob_content_md5 {
                            req.insert_header("x-ms-blob-content-md5", x_ms_blob_content_md5);
                        }
                        if let Some(x_ms_blob_cache_control) = &this.x_ms_blob_cache_control {
                            req.insert_header("x-ms-blob-cache-control", x_ms_blob_cache_control);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_content_disposition) = &this.x_ms_blob_content_disposition {
                            req.insert_header("x-ms-blob-content-disposition", x_ms_blob_content_disposition);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        if let Some(x_ms_legal_hold) = &this.x_ms_legal_hold {
                            req.insert_header("x-ms-legal-hold", &x_ms_legal_hold.to_string());
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod put_blob_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) x_ms_blob_type: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_copy_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_blob_content_type: Option<String>,
            pub(crate) x_ms_blob_content_encoding: Option<String>,
            pub(crate) x_ms_blob_content_language: Option<String>,
            pub(crate) x_ms_blob_content_md5: Option<String>,
            pub(crate) x_ms_blob_cache_control: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_content_disposition: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) x_ms_source_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_source_content_md5: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_copy_source_blob_properties: Option<bool>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
            pub(crate) x_ms_copy_source_tag_option: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_type(mut self, x_ms_blob_content_type: impl Into<String>) -> Self {
                self.x_ms_blob_content_type = Some(x_ms_blob_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_encoding(mut self, x_ms_blob_content_encoding: impl Into<String>) -> Self {
                self.x_ms_blob_content_encoding = Some(x_ms_blob_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_language(mut self, x_ms_blob_content_language: impl Into<String>) -> Self {
                self.x_ms_blob_content_language = Some(x_ms_blob_content_language.into());
                self
            }
            #[doc = "Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks were validated when each was uploaded."]
            pub fn x_ms_blob_content_md5(mut self, x_ms_blob_content_md5: impl Into<String>) -> Self {
                self.x_ms_blob_content_md5 = Some(x_ms_blob_content_md5.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_cache_control(mut self, x_ms_blob_cache_control: impl Into<String>) -> Self {
                self.x_ms_blob_cache_control = Some(x_ms_blob_cache_control.into());
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_blob_content_disposition(mut self, x_ms_blob_content_disposition: impl Into<String>) -> Self {
                self.x_ms_blob_content_disposition = Some(x_ms_blob_content_disposition.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Optional. Indicates the tier to be set on the blob."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_tags(mut self, x_ms_source_if_tags: impl Into<String>) -> Self {
                self.x_ms_source_if_tags = Some(x_ms_source_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specify the md5 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_md5(mut self, x_ms_source_content_md5: impl Into<String>) -> Self {
                self.x_ms_source_content_md5 = Some(x_ms_source_content_md5.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Optional, default is true.  Indicates if properties from the source blob should be copied."]
            pub fn x_ms_copy_source_blob_properties(mut self, x_ms_copy_source_blob_properties: bool) -> Self {
                self.x_ms_copy_source_blob_properties = Some(x_ms_copy_source_blob_properties);
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "Optional, default 'replace'.  Indicates if source tags should be copied or replaced with the tags specified by x-ms-tags."]
            pub fn x_ms_copy_source_tag_option(mut self, x_ms_copy_source_tag_option: impl Into<String>) -> Self {
                self.x_ms_copy_source_tag_option = Some(x_ms_copy_source_tag_option.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?BlockBlob&fromUrl",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        req.insert_header("x-ms-blob-type", &this.x_ms_blob_type);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(x_ms_blob_content_type) = &this.x_ms_blob_content_type {
                            req.insert_header("x-ms-blob-content-type", x_ms_blob_content_type);
                        }
                        if let Some(x_ms_blob_content_encoding) = &this.x_ms_blob_content_encoding {
                            req.insert_header("x-ms-blob-content-encoding", x_ms_blob_content_encoding);
                        }
                        if let Some(x_ms_blob_content_language) = &this.x_ms_blob_content_language {
                            req.insert_header("x-ms-blob-content-language", x_ms_blob_content_language);
                        }
                        if let Some(x_ms_blob_content_md5) = &this.x_ms_blob_content_md5 {
                            req.insert_header("x-ms-blob-content-md5", x_ms_blob_content_md5);
                        }
                        if let Some(x_ms_blob_cache_control) = &this.x_ms_blob_cache_control {
                            req.insert_header("x-ms-blob-cache-control", x_ms_blob_cache_control);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_content_disposition) = &this.x_ms_blob_content_disposition {
                            req.insert_header("x-ms-blob-content-disposition", x_ms_blob_content_disposition);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(x_ms_source_if_tags) = &this.x_ms_source_if_tags {
                            req.insert_header("x-ms-source-if-tags", x_ms_source_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_source_content_md5) = &this.x_ms_source_content_md5 {
                            req.insert_header("x-ms-source-content-md5", x_ms_source_content_md5);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_copy_source_blob_properties) = &this.x_ms_copy_source_blob_properties {
                            req.insert_header("x-ms-copy-source-blob-properties", &x_ms_copy_source_blob_properties.to_string());
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        if let Some(x_ms_copy_source_tag_option) = &this.x_ms_copy_source_tag_option {
                            req.insert_header("x-ms-copy-source-tag-option", x_ms_copy_source_tag_option);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod stage_block {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) blockid: String,
            pub(crate) content_length: i64,
            pub(crate) body: serde_json::Value,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_content_crc64: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Specify the transactional crc64 for the body, to be validated by the service."]
            pub fn x_ms_content_crc64(mut self, x_ms_content_crc64: impl Into<String>) -> Self {
                self.x_ms_content_crc64 = Some(x_ms_content_crc64.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=block",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let blockid = &this.blockid;
                        req.url_mut().query_pairs_mut().append_pair("blockid", blockid);
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_content_crc64) = &this.x_ms_content_crc64 {
                            req.insert_header("x-ms-content-crc64", x_ms_content_crc64);
                        }
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod stage_block_from_url {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) blockid: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_copy_source: String,
            pub(crate) x_ms_source_range: Option<String>,
            pub(crate) x_ms_source_content_md5: Option<String>,
            pub(crate) x_ms_source_content_crc64: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Bytes of source data in the specified range."]
            pub fn x_ms_source_range(mut self, x_ms_source_range: impl Into<String>) -> Self {
                self.x_ms_source_range = Some(x_ms_source_range.into());
                self
            }
            #[doc = "Specify the md5 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_md5(mut self, x_ms_source_content_md5: impl Into<String>) -> Self {
                self.x_ms_source_content_md5 = Some(x_ms_source_content_md5.into());
                self
            }
            #[doc = "Specify the crc64 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_crc64(mut self, x_ms_source_content_crc64: impl Into<String>) -> Self {
                self.x_ms_source_content_crc64 = Some(x_ms_source_content_crc64.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=block&fromURL",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        let blockid = &this.blockid;
                        req.url_mut().query_pairs_mut().append_pair("blockid", blockid);
                        req.insert_header("content-length", &this.content_length.to_string());
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_source_range) = &this.x_ms_source_range {
                            req.insert_header("x-ms-source-range", x_ms_source_range);
                        }
                        if let Some(x_ms_source_content_md5) = &this.x_ms_source_content_md5 {
                            req.insert_header("x-ms-source-content-md5", x_ms_source_content_md5);
                        }
                        if let Some(x_ms_source_content_crc64) = &this.x_ms_source_content_crc64 {
                            req.insert_header("x-ms-source-content-crc64", x_ms_source_content_crc64);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_block_list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BlockList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BlockList = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The media type of the body of the response. For Get Block List this is 'application/xml'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "The size of the blob in bytes."]
            pub fn x_ms_blob_content_length(&self) -> azure_core::Result<i64> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-blob-content-length"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) blocklisttype: String,
            pub(crate) snapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more information on working with blob snapshots, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob\">Creating a Snapshot of a Blob.</a>"]
            pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
                self.snapshot = Some(snapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=blocklist",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(snapshot) = &this.snapshot {
                            req.url_mut().query_pairs_mut().append_pair("snapshot", snapshot);
                        }
                        let blocklisttype = &this.blocklisttype;
                        req.url_mut().query_pairs_mut().append_pair("blocklisttype", blocklisttype);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<models::BlockList>> {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod commit_block_list {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) container_name: String,
            pub(crate) blob: String,
            pub(crate) blocks: models::BlockLookupList,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_blob_cache_control: Option<String>,
            pub(crate) x_ms_blob_content_type: Option<String>,
            pub(crate) x_ms_blob_content_encoding: Option<String>,
            pub(crate) x_ms_blob_content_language: Option<String>,
            pub(crate) x_ms_blob_content_md5: Option<String>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_content_crc64: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_blob_content_disposition: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
            pub(crate) x_ms_encryption_scope: Option<String>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) x_ms_if_tags: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_tags: Option<String>,
            pub(crate) x_ms_immutability_policy_until_date: Option<time::OffsetDateTime>,
            pub(crate) x_ms_immutability_policy_mode: Option<String>,
            pub(crate) x_ms_legal_hold: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_cache_control(mut self, x_ms_blob_cache_control: impl Into<String>) -> Self {
                self.x_ms_blob_cache_control = Some(x_ms_blob_cache_control.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_type(mut self, x_ms_blob_content_type: impl Into<String>) -> Self {
                self.x_ms_blob_content_type = Some(x_ms_blob_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_encoding(mut self, x_ms_blob_content_encoding: impl Into<String>) -> Self {
                self.x_ms_blob_content_encoding = Some(x_ms_blob_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_blob_content_language(mut self, x_ms_blob_content_language: impl Into<String>) -> Self {
                self.x_ms_blob_content_language = Some(x_ms_blob_content_language.into());
                self
            }
            #[doc = "Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks were validated when each was uploaded."]
            pub fn x_ms_blob_content_md5(mut self, x_ms_blob_content_md5: impl Into<String>) -> Self {
                self.x_ms_blob_content_md5 = Some(x_ms_blob_content_md5.into());
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Specify the transactional crc64 for the body, to be validated by the service."]
            pub fn x_ms_content_crc64(mut self, x_ms_content_crc64: impl Into<String>) -> Self {
                self.x_ms_content_crc64 = Some(x_ms_content_crc64.into());
                self
            }
            #[doc = "Optional. Specifies a user-defined name-value pair associated with the blob. If no name-value pairs are specified, the operation will copy the metadata from the source blob or file to the destination blob. If one or more name-value pairs are specified, the destination blob is created with the specified metadata, and metadata is not copied from the source blob or file. Note that beginning with version 2009-09-19, metadata names must adhere to the naming rules for C# identifiers. See Naming and Referencing Containers, Blobs, and Metadata for more information."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_blob_content_disposition(mut self, x_ms_blob_content_disposition: impl Into<String>) -> Self {
                self.x_ms_blob_content_disposition = Some(x_ms_blob_content_disposition.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
                self
            }
            #[doc = "Optional. Version 2019-07-07 and later.  Specifies the name of the encryption scope to use to encrypt the data provided in the request. If not specified, encryption is performed with the default account encryption scope.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_scope(mut self, x_ms_encryption_scope: impl Into<String>) -> Self {
                self.x_ms_encryption_scope = Some(x_ms_encryption_scope.into());
                self
            }
            #[doc = "Optional. Indicates the tier to be set on the blob."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify a SQL where clause on blob tags to operate only on blobs with a matching value."]
            pub fn x_ms_if_tags(mut self, x_ms_if_tags: impl Into<String>) -> Self {
                self.x_ms_if_tags = Some(x_ms_if_tags.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional.  Used to set blob tags in various blob operations."]
            pub fn x_ms_tags(mut self, x_ms_tags: impl Into<String>) -> Self {
                self.x_ms_tags = Some(x_ms_tags.into());
                self
            }
            #[doc = "Specifies the date time when the blobs immutability policy is set to expire."]
            pub fn x_ms_immutability_policy_until_date(
                mut self,
                x_ms_immutability_policy_until_date: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.x_ms_immutability_policy_until_date = Some(x_ms_immutability_policy_until_date.into());
                self
            }
            #[doc = "Specifies the immutability policy mode to set on the blob."]
            pub fn x_ms_immutability_policy_mode(mut self, x_ms_immutability_policy_mode: impl Into<String>) -> Self {
                self.x_ms_immutability_policy_mode = Some(x_ms_immutability_policy_mode.into());
                self
            }
            #[doc = "Specified if a legal hold should be set on the blob."]
            pub fn x_ms_legal_hold(mut self, x_ms_legal_hold: bool) -> Self {
                self.x_ms_legal_hold = Some(x_ms_legal_hold);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=blocklist",
                            this.client.endpoint(),
                            &this.container_name,
                            &this.blob
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-04-10");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_blob_cache_control) = &this.x_ms_blob_cache_control {
                            req.insert_header("x-ms-blob-cache-control", x_ms_blob_cache_control);
                        }
                        if let Some(x_ms_blob_content_type) = &this.x_ms_blob_content_type {
                            req.insert_header("x-ms-blob-content-type", x_ms_blob_content_type);
                        }
                        if let Some(x_ms_blob_content_encoding) = &this.x_ms_blob_content_encoding {
                            req.insert_header("x-ms-blob-content-encoding", x_ms_blob_content_encoding);
                        }
                        if let Some(x_ms_blob_content_language) = &this.x_ms_blob_content_language {
                            req.insert_header("x-ms-blob-content-language", x_ms_blob_content_language);
                        }
                        if let Some(x_ms_blob_content_md5) = &this.x_ms_blob_content_md5 {
                            req.insert_header("x-ms-blob-content-md5", x_ms_blob_content_md5);
                        }
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_content_crc64) = &this.x_ms_content_crc64 {
                            req.insert_header("x-ms-content-crc64", x_ms_content_crc64);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_blob_content_disposition) = &this.x_ms_blob_content_disposition {
                            req.insert_header("x-ms-blob-content-disposition", x_ms_blob_content_disposition);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        if let Some(x_ms_encryption_scope) = &this.x_ms_encryption_scope {
                            req.insert_header("x-ms-encryption-scope", x_ms_encryption_scope);
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(x_ms_if_tags) = &this.x_ms_if_tags {
                            req.insert_header("x-ms-if-tags", x_ms_if_tags);
                        }
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.blocks)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_tags) = &this.x_ms_tags {
                            req.insert_header("x-ms-tags", x_ms_tags);
                        }
                        if let Some(x_ms_immutability_policy_until_date) = &this.x_ms_immutability_policy_until_date {
                            req.insert_header(
                                "x-ms-immutability-policy-until-date",
                                &x_ms_immutability_policy_until_date.to_string(),
                            );
                        }
                        if let Some(x_ms_immutability_policy_mode) = &this.x_ms_immutability_policy_mode {
                            req.insert_header("x-ms-immutability-policy-mode", x_ms_immutability_policy_mode);
                        }
                        if let Some(x_ms_legal_hold) = &this.x_ms_legal_hold {
                            req.insert_header("x-ms-legal-hold", &x_ms_legal_hold.to_string());
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}

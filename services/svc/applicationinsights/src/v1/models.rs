#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetricId {
    #[serde(rename = "requests/count")]
    RequestsCount,
    #[serde(rename = "requests/duration")]
    RequestsDuration,
    #[serde(rename = "requests/failed")]
    RequestsFailed,
    #[serde(rename = "users/count")]
    UsersCount,
    #[serde(rename = "users/authenticated")]
    UsersAuthenticated,
    #[serde(rename = "pageViews/count")]
    PageViewsCount,
    #[serde(rename = "pageViews/duration")]
    PageViewsDuration,
    #[serde(rename = "client/processingDuration")]
    ClientProcessingDuration,
    #[serde(rename = "client/receiveDuration")]
    ClientReceiveDuration,
    #[serde(rename = "client/networkDuration")]
    ClientNetworkDuration,
    #[serde(rename = "client/sendDuration")]
    ClientSendDuration,
    #[serde(rename = "client/totalDuration")]
    ClientTotalDuration,
    #[serde(rename = "dependencies/count")]
    DependenciesCount,
    #[serde(rename = "dependencies/failed")]
    DependenciesFailed,
    #[serde(rename = "dependencies/duration")]
    DependenciesDuration,
    #[serde(rename = "exceptions/count")]
    ExceptionsCount,
    #[serde(rename = "exceptions/browser")]
    ExceptionsBrowser,
    #[serde(rename = "exceptions/server")]
    ExceptionsServer,
    #[serde(rename = "sessions/count")]
    SessionsCount,
    #[serde(rename = "performanceCounters/requestExecutionTime")]
    PerformanceCountersRequestExecutionTime,
    #[serde(rename = "performanceCounters/requestsPerSecond")]
    PerformanceCountersRequestsPerSecond,
    #[serde(rename = "performanceCounters/requestsInQueue")]
    PerformanceCountersRequestsInQueue,
    #[serde(rename = "performanceCounters/memoryAvailableBytes")]
    PerformanceCountersMemoryAvailableBytes,
    #[serde(rename = "performanceCounters/exceptionsPerSecond")]
    PerformanceCountersExceptionsPerSecond,
    #[serde(rename = "performanceCounters/processCpuPercentage")]
    PerformanceCountersProcessCpuPercentage,
    #[serde(rename = "performanceCounters/processIOBytesPerSecond")]
    PerformanceCountersProcessIoBytesPerSecond,
    #[serde(rename = "performanceCounters/processPrivateBytes")]
    PerformanceCountersProcessPrivateBytes,
    #[serde(rename = "performanceCounters/processorCpuPercentage")]
    PerformanceCountersProcessorCpuPercentage,
    #[serde(rename = "availabilityResults/availabilityPercentage")]
    AvailabilityResultsAvailabilityPercentage,
    #[serde(rename = "availabilityResults/duration")]
    AvailabilityResultsDuration,
    #[serde(rename = "billing/telemetryCount")]
    BillingTelemetryCount,
    #[serde(rename = "customEvents/count")]
    CustomEventsCount,
}
pub type MetricsTimespan = String;
pub type MetricsAggregation = Vec<String>;
pub type MetricsInterval = String;
pub type MetricsSegment = Vec<String>;
pub type MetricsTop = i32;
pub type MetricsOrderBy = String;
pub type MetricsFilter = String;
pub type MetricsPostBody = Vec<MetricsPostBodySchema>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsPostBodySchema {
    pub id: String,
    pub parameters: metrics_post_body_schema::Parameters,
}
pub mod metrics_post_body_schema {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Parameters {
        #[serde(rename = "metricId")]
        pub metric_id: MetricId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timespan: Option<MetricsTimespan>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aggregation: Option<MetricsAggregation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<MetricsInterval>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub segment: Option<MetricsSegment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub top: Option<MetricsTop>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub orderby: Option<MetricsOrderBy>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<MetricsFilter>,
    }
}
pub type MetricsResults = Vec<serde_json::Value>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<MetricsResultInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsResultInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<MetricsSegmentInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsSegmentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<MetricsSegmentInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "$all")]
    U24all,
    #[serde(rename = "traces")]
    Traces,
    #[serde(rename = "customEvents")]
    CustomEvents,
    #[serde(rename = "pageViews")]
    PageViews,
    #[serde(rename = "browserTimings")]
    BrowserTimings,
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "dependencies")]
    Dependencies,
    #[serde(rename = "exceptions")]
    Exceptions,
    #[serde(rename = "availabilityResults")]
    AvailabilityResults,
    #[serde(rename = "performanceCounters")]
    PerformanceCounters,
    #[serde(rename = "customMetrics")]
    CustomMetrics,
}
pub type EventId = String;
pub type EventsTimespan = String;
pub type EventsFilter = String;
pub type EventsSearch = String;
pub type EventsOrderBy = String;
pub type EventsSelect = String;
pub type EventsSkip = i32;
pub type EventsTop = i32;
pub type EventsFormat = String;
pub type EventsCount = bool;
pub type EventsApply = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsResults {
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[serde(rename = "@ai.messages", default, skip_serializing_if = "Vec::is_empty")]
    pub ai_messages: Vec<ErrorInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventsResultData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsResult {
    #[serde(rename = "@ai.messages", default, skip_serializing_if = "Vec::is_empty")]
    pub ai_messages: Vec<ErrorInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<EventsResultData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsResultData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: EventType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "customDimensions", default, skip_serializing_if = "Option::is_none")]
    pub custom_dimensions: Option<events_result_data::CustomDimensions>,
    #[serde(rename = "customMeasurements", default, skip_serializing_if = "Option::is_none")]
    pub custom_measurements: Option<events_result_data::CustomMeasurements>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<EventsOperationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<EventsSessionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<EventsUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<EventsCloudInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<EventsAiInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<EventsApplicationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<EventsClientInfo>,
}
pub mod events_result_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CustomDimensions {
        #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
        pub additional_properties: Option<serde_json::Value>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CustomMeasurements {
        #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
        pub additional_properties: Option<serde_json::Value>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsTraceResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsTraceInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCustomEventResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCustomEventInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsPageViewResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsPageViewInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsBrowserTimingResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsBrowserTimingInfo {
    #[serde(rename = "urlPath", default, skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
    #[serde(rename = "urlHost", default, skip_serializing_if = "Option::is_none")]
    pub url_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "totalDuration", default, skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[serde(rename = "networkDuration", default, skip_serializing_if = "Option::is_none")]
    pub network_duration: Option<i64>,
    #[serde(rename = "sendDuration", default, skip_serializing_if = "Option::is_none")]
    pub send_duration: Option<i64>,
    #[serde(rename = "receiveDuration", default, skip_serializing_if = "Option::is_none")]
    pub receive_duration: Option<i64>,
    #[serde(rename = "processingDuration", default, skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsClientPerformanceInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsRequestResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsRequestInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsDependencyResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsDependencyInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsExceptionResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsExceptionInfo {
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<i64>,
    #[serde(rename = "problemId", default, skip_serializing_if = "Option::is_none")]
    pub problem_id: Option<String>,
    #[serde(rename = "handledAt", default, skip_serializing_if = "Option::is_none")]
    pub handled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "outerType", default, skip_serializing_if = "Option::is_none")]
    pub outer_type: Option<String>,
    #[serde(rename = "outerMethod", default, skip_serializing_if = "Option::is_none")]
    pub outer_method: Option<String>,
    #[serde(rename = "outerAssembly", default, skip_serializing_if = "Option::is_none")]
    pub outer_assembly: Option<String>,
    #[serde(rename = "outerMessage", default, skip_serializing_if = "Option::is_none")]
    pub outer_message: Option<String>,
    #[serde(rename = "innermostType", default, skip_serializing_if = "Option::is_none")]
    pub innermost_type: Option<String>,
    #[serde(rename = "innermostMessage", default, skip_serializing_if = "Option::is_none")]
    pub innermost_message: Option<String>,
    #[serde(rename = "innermostMethod", default, skip_serializing_if = "Option::is_none")]
    pub innermost_method: Option<String>,
    #[serde(rename = "innermostAssembly", default, skip_serializing_if = "Option::is_none")]
    pub innermost_assembly: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<EventsExceptionDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsExceptionDetail {
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<String>,
    #[serde(rename = "outerId", default, skip_serializing_if = "Option::is_none")]
    pub outer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parsedStack", default, skip_serializing_if = "Vec::is_empty")]
    pub parsed_stack: Vec<EventsExceptionDetailsParsedStack>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsExceptionDetailsParsedStack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsAvailabilityResultResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsAvailabilityResultInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsPerformanceCounterResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsPerformanceCounterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counter: Option<String>,
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCustomMetricResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCustomMetricInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "valueSum", default, skip_serializing_if = "Option::is_none")]
    pub value_sum: Option<f64>,
    #[serde(rename = "valueCount", default, skip_serializing_if = "Option::is_none")]
    pub value_count: Option<i32>,
    #[serde(rename = "valueMin", default, skip_serializing_if = "Option::is_none")]
    pub value_min: Option<f64>,
    #[serde(rename = "valueMax", default, skip_serializing_if = "Option::is_none")]
    pub value_max: Option<f64>,
    #[serde(rename = "valueStdDev", default, skip_serializing_if = "Option::is_none")]
    pub value_std_dev: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsOperationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "syntheticSource", default, skip_serializing_if = "Option::is_none")]
    pub synthetic_source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsSessionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsUserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "authenticatedId", default, skip_serializing_if = "Option::is_none")]
    pub authenticated_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCloudInfo {
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "roleInstance", default, skip_serializing_if = "Option::is_none")]
    pub role_instance: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsAiInfo {
    #[serde(rename = "iKey", default, skip_serializing_if = "Option::is_none")]
    pub i_key: Option<String>,
    #[serde(rename = "appName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "sdkVersion", default, skip_serializing_if = "Option::is_none")]
    pub sdk_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsApplicationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsClientInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
pub type QueryParam = String;
pub type QueryTimespan = String;
pub type ApplicationsParam = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryBody {
    pub query: QueryParam,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<QueryTimespan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<ApplicationsParam>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResults {
    pub tables: Vec<Table>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataResults {
    #[serde(rename = "tableGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub table_groups: Vec<MetadataTableGroup>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<MetadataTable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<MetadataFunction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<MetadataApplication>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataTableGroup {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataTable {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "timespanColumn", default, skip_serializing_if = "Option::is_none")]
    pub timespan_column: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataFunction {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub body: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataApplication {
    pub id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub name: String,
    pub region: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<String>,
    #[serde(rename = "tableGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub table_groups: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ErrorInfo>>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}

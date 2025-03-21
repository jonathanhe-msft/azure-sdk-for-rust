#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents an instance of an auto scale v-core resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleVCore {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Represents the SKU name and Azure pricing tier for auto scale v-core resource."]
    pub sku: AutoScaleVCoreSku,
    #[doc = "Properties of an auto scale v-core resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoScaleVCoreProperties>,
}
impl AutoScaleVCore {
    pub fn new(resource: Resource, sku: AutoScaleVCoreSku) -> Self {
        Self {
            resource,
            sku,
            properties: None,
        }
    }
}
#[doc = "An array of auto scale v-core resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleVCoreListResult {
    #[doc = "An array of auto scale v-core resources."]
    pub value: Vec<AutoScaleVCore>,
}
impl azure_core::Continuable for AutoScaleVCoreListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AutoScaleVCoreListResult {
    pub fn new(value: Vec<AutoScaleVCore>) -> Self {
        Self { value }
    }
}
#[doc = "An object that represents a set of mutable auto scale v-core resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleVCoreMutableProperties {
    #[doc = "The maximum capacity of an auto scale v-core resource."]
    #[serde(rename = "capacityLimit", default, skip_serializing_if = "Option::is_none")]
    pub capacity_limit: Option<i32>,
}
impl AutoScaleVCoreMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an auto scale v-core resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleVCoreProperties {
    #[serde(flatten)]
    pub auto_scale_v_core_mutable_properties: AutoScaleVCoreMutableProperties,
    #[doc = "The object ID of the capacity resource associated with the auto scale v-core resource."]
    #[serde(rename = "capacityObjectId", default, skip_serializing_if = "Option::is_none")]
    pub capacity_object_id: Option<String>,
    #[doc = "The current deployment state of an auto scale v-core resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<auto_scale_v_core_properties::ProvisioningState>,
}
impl AutoScaleVCoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_scale_v_core_properties {
    use super::*;
    #[doc = "The current deployment state of an auto scale v-core resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents the SKU name and Azure pricing tier for auto scale v-core resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleVCoreSku {
    #[doc = "Name of the SKU level."]
    pub name: String,
    #[doc = "The name of the Azure pricing tier to which the SKU applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<auto_scale_v_core_sku::Tier>,
    #[doc = "The capacity of an auto scale v-core resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl AutoScaleVCoreSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
        }
    }
}
pub mod auto_scale_v_core_sku {
    use super::*;
    #[doc = "The name of the Azure pricing tier to which the SKU applies."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        AutoScale,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoScale => serializer.serialize_unit_variant("Tier", 0u32, "AutoScale"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update request specification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleVCoreUpdateParameters {
    #[doc = "Represents the SKU name and Azure pricing tier for auto scale v-core resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AutoScaleVCoreSku>,
    #[doc = "Key-value pairs of additional provisioning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An object that represents a set of mutable auto scale v-core resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoScaleVCoreMutableProperties>,
}
impl AutoScaleVCoreUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the SKU name and Azure pricing tier for PowerBI Dedicated capacity resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacitySku {
    #[doc = "Name of the SKU level."]
    pub name: String,
    #[doc = "The name of the Azure pricing tier to which the SKU applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<capacity_sku::Tier>,
    #[doc = "The capacity of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl CapacitySku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
        }
    }
}
pub mod capacity_sku {
    use super::*;
    #[doc = "The name of the Azure pricing tier to which the SKU applies."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        #[serde(rename = "PBIE_Azure")]
        PbieAzure,
        Premium,
        AutoPremiumHost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PbieAzure => serializer.serialize_unit_variant("Tier", 0u32, "PBIE_Azure"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 1u32, "Premium"),
                Self::AutoPremiumHost => serializer.serialize_unit_variant("Tier", 2u32, "AutoPremiumHost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of capacity name request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckCapacityNameAvailabilityParameters {
    #[doc = "Name for checking availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of PowerBI dedicated."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckCapacityNameAvailabilityParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The checking result of capacity name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckCapacityNameAvailabilityResult {
    #[doc = "Indicator of availability of the capacity name."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason of unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed message of the request unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckCapacityNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An array of Dedicated capacities resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacities {
    #[doc = "An array of Dedicated capacities resources."]
    pub value: Vec<DedicatedCapacity>,
}
impl azure_core::Continuable for DedicatedCapacities {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DedicatedCapacities {
    pub fn new(value: Vec<DedicatedCapacity>) -> Self {
        Self { value }
    }
}
#[doc = "Represents an instance of a Dedicated Capacity resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedCapacity {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Represents the SKU name and Azure pricing tier for PowerBI Dedicated capacity resource."]
    pub sku: CapacitySku,
    #[doc = "Properties of Dedicated Capacity resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCapacityProperties>,
}
impl DedicatedCapacity {
    pub fn new(resource: Resource, sku: CapacitySku) -> Self {
        Self {
            resource,
            sku,
            properties: None,
        }
    }
}
#[doc = "An array of administrator user identities"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedCapacityAdministrators {
    #[doc = "An array of administrator user identities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}
impl DedicatedCapacityAdministrators {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents a set of mutable Dedicated capacity resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedCapacityMutableProperties {
    #[doc = "An array of administrator user identities"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<DedicatedCapacityAdministrators>,
    #[doc = "Specifies the generation of the Power BI Embedded capacity. If no value is specified, the default value 'Gen2' is used. [Learn More](https://docs.microsoft.com/power-bi/developer/embedded/power-bi-embedded-generation-2)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<dedicated_capacity_mutable_properties::Mode>,
    #[doc = "Tenant ID for the capacity. Used for creating Pro Plus capacity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Capacity name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl DedicatedCapacityMutableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dedicated_capacity_mutable_properties {
    use super::*;
    #[doc = "Specifies the generation of the Power BI Embedded capacity. If no value is specified, the default value 'Gen2' is used. [Learn More](https://docs.microsoft.com/power-bi/developer/embedded/power-bi-embedded-generation-2)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Gen1,
        Gen2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Gen1 => serializer.serialize_unit_variant("Mode", 0u32, "Gen1"),
                Self::Gen2 => serializer.serialize_unit_variant("Mode", 1u32, "Gen2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of Dedicated Capacity resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedCapacityProperties {
    #[serde(flatten)]
    pub dedicated_capacity_mutable_properties: DedicatedCapacityMutableProperties,
    #[doc = "The current state of PowerBI Dedicated resource. The state is to indicate more states outside of resource provisioning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<dedicated_capacity_properties::State>,
    #[doc = "The current deployment state of PowerBI Dedicated resource. The provisioningState is to indicate states for resource provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<dedicated_capacity_properties::ProvisioningState>,
}
impl DedicatedCapacityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dedicated_capacity_properties {
    use super::*;
    #[doc = "The current state of PowerBI Dedicated resource. The state is to indicate more states outside of resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Deleting => serializer.serialize_unit_variant("State", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("State", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("State", 2u32, "Failed"),
                Self::Paused => serializer.serialize_unit_variant("State", 3u32, "Paused"),
                Self::Suspended => serializer.serialize_unit_variant("State", 4u32, "Suspended"),
                Self::Provisioning => serializer.serialize_unit_variant("State", 5u32, "Provisioning"),
                Self::Updating => serializer.serialize_unit_variant("State", 6u32, "Updating"),
                Self::Suspending => serializer.serialize_unit_variant("State", 7u32, "Suspending"),
                Self::Pausing => serializer.serialize_unit_variant("State", 8u32, "Pausing"),
                Self::Resuming => serializer.serialize_unit_variant("State", 9u32, "Resuming"),
                Self::Preparing => serializer.serialize_unit_variant("State", 10u32, "Preparing"),
                Self::Scaling => serializer.serialize_unit_variant("State", 11u32, "Scaling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current deployment state of PowerBI Dedicated resource. The provisioningState is to indicate states for resource provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Paused => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Paused"),
                Self::Suspended => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Suspended"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Suspending => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Suspending"),
                Self::Pausing => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Pausing"),
                Self::Resuming => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Resuming"),
                Self::Preparing => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Preparing"),
                Self::Scaling => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Scaling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Provision request specification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedCapacityUpdateParameters {
    #[doc = "Represents the SKU name and Azure pricing tier for PowerBI Dedicated capacity resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<CapacitySku>,
    #[doc = "Key-value pairs of additional provisioning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An object that represents a set of mutable Dedicated capacity resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedCapacityMutableProperties>,
}
impl DedicatedCapacityUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response {
    use super::*;
    #[doc = "The error object"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The type of identity that created/modified the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("IdentityType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("IdentityType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("IdentityType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Log specification for exposing diagnostic logs to shoebox."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localizable name of the log"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration for the log"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric specification for exposing performance metrics to shoebox."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Metric name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localizable metric name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localizable description of metric"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit for the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Aggregation type for the metric"]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Pattern used to filter the metric"]
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[doc = "For describing multi dimensional metrics"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<serde_json::Value>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capacities REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional properties to expose performance metrics to shoebox."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<operation::Properties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.PowerBIDedicated."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: capacity, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: create, update, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Localized description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Additional properties to expose performance metrics to shoebox."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Service specification for exposing performance metrics to shoebox."]
        #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<ServiceSpecification>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result listing capacities. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of capacities supported by the Microsoft.PowerBIDedicated resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an instance of an PowerBI Dedicated resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "An identifier that represents the PowerBI Dedicated resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the PowerBI Dedicated resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the PowerBI Dedicated resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location of the PowerBI Dedicated resource."]
    pub location: String,
    #[doc = "Key-value pairs of additional resource provisioning properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
        }
    }
}
#[doc = "Service specification for exposing performance metrics to shoebox."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Metric specifications for exposing performance metrics to shoebox."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[doc = "Log specifications for exposing diagnostic logs to shoebox."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents SKU details for existing resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDetailsForExistingResource {
    #[doc = "The resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Represents the SKU name and Azure pricing tier for PowerBI Dedicated capacity resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<CapacitySku>,
}
impl SkuDetailsForExistingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents enumerating SKUs for existing resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuEnumerationForExistingResourceResult {
    #[doc = "The collection of available SKUs for existing resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDetailsForExistingResource>,
}
impl SkuEnumerationForExistingResourceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that represents enumerating SKUs for new resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuEnumerationForNewResourceResult {
    #[doc = "The collection of available SKUs for new resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapacitySku>,
}
impl SkuEnumerationForNewResourceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "An identifier for the identity that created the resource"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created/modified the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource creation (UTC)"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "An identifier for the identity that last modified the resource"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that created/modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}

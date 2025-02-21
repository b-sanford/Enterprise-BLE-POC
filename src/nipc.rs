// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSubscription {
    #[prost(string, optional, tag = "1")]
    pub device_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, optional, tag = "4")]
    pub ap_mac_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof = "data_subscription::Subscription", tags = "11, 12, 13, 14, 15")]
    pub subscription: ::core::option::Option<data_subscription::Subscription>,
}
/// Nested message and enum types in `DataSubscription`.
pub mod data_subscription {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BleSubscription {
        #[prost(string, optional, tag = "1")]
        pub service_uuid: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub characteristic_uuid: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BleAdvertisement {
        #[prost(string, tag = "1")]
        pub mac_address: ::prost::alloc::string::String,
        #[prost(int32, optional, tag = "2")]
        pub rssi: ::core::option::Option<i32>,
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ZigbeeSubscription {
        #[prost(int32, optional, tag = "1")]
        pub endpoint_id: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub cluster_id: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub attribute_id: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub attribute_type: ::core::option::Option<i32>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BleConnectionStatus {
        #[prost(string, tag = "1")]
        pub mac_address: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub connected: bool,
        #[prost(int32, optional, tag = "3")]
        pub reason: ::core::option::Option<i32>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RawPayload {
        #[prost(string, optional, tag = "1")]
        pub context_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subscription {
        #[prost(message, tag = "11")]
        BleSubscription(BleSubscription),
        #[prost(message, tag = "12")]
        BleAdvertisement(BleAdvertisement),
        #[prost(message, tag = "13")]
        ZigbeeSubscription(ZigbeeSubscription),
        #[prost(message, tag = "14")]
        RawPayload(RawPayload),
        #[prost(message, tag = "15")]
        BleConnectionStatus(BleConnectionStatus),
    }
}

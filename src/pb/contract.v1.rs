// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub fees: ::prost::alloc::vec::Vec<Fee>,
    #[prost(message, repeated, tag="2")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub fee_zt_o: u32,
    #[prost(uint32, tag="7")]
    pub fee_ot_z: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(int64, tag="12")]
    pub tick: i64,
}
// @@protoc_insertion_point(module)

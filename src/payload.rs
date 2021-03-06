use crate::payload_wrapper::*;
use crate::types::*;
use std::collections::HashMap;

// ====================== Request Header Processing API ===========================
pub fn get_request_header_pairs() -> Result<HashMap<String, String>, String> {
  get_header_map_pairs(HeaderMapType::RequestHeaders)
}

pub fn set_request_header_pairs(_pairs: &HashMap<String, String>) -> WasmResult {
  set_header_map_pairs(HeaderMapType::RequestHeaders, _pairs)
}

pub fn get_request_header(key: String) -> Result<Box<WasmData>, String> {
  get_header_map_value(HeaderMapType::RequestHeaders, key)
}

pub fn add_request_header(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::RequestHeaders, key, value)
}

pub fn replace_request_header(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::RequestHeaders, key, value)
}

pub fn remove_request_header(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::RequestHeaders, key)
}

pub fn get_request_header_size() -> u32 {
  get_header_map_value_size(HeaderMapType::RequestHeaders)
}
// ====================== Request Header Processing API ===========================

// ====================== Response Header Processing API ===========================
pub fn set_response_header_pairs(_pairs: &HashMap<String, String>) -> WasmResult {
  set_header_map_pairs(HeaderMapType::ResponseHeaders, _pairs)
}

pub fn get_response_header_pairs() -> Result<HashMap<String, String>, String> {
  get_header_map_pairs(HeaderMapType::ResponseHeaders)
}

pub fn get_response_header(key: String) -> Result<Box<WasmData>, String> {
  get_header_map_value(HeaderMapType::ResponseHeaders, key)
}

pub fn add_response_header(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::ResponseHeaders, key, value)
}

pub fn replace_response_header(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::ResponseHeaders, key, value)
}

pub fn remove_response_header(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::ResponseHeaders, key)
}

pub fn get_response_header_size() -> u32 {
  get_header_map_value_size(HeaderMapType::ResponseHeaders)
}
// ====================== Response Header Processing API ===========================

// ====================== Request Trailer Processing API ===========================
pub fn set_request_trailer_pairs(_pairs: &HashMap<String, String>) -> WasmResult {
  set_header_map_pairs(HeaderMapType::RequestTrailers, _pairs)
}

pub fn get_request_trailer_pairs() -> Result<HashMap<String, String>, String> {
  get_header_map_pairs(HeaderMapType::RequestTrailers)
}

pub fn get_request_trailer(key: String) -> Result<Box<WasmData>, String> {
  get_header_map_value(HeaderMapType::RequestTrailers, key)
}

pub fn add_request_trailer(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::RequestTrailers, key, value)
}

pub fn replace_request_trailer(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::RequestTrailers, key, value)
}

pub fn remove_request_trailer(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::RequestTrailers, key)
}

pub fn get_request_trailer_size() -> u32 {
  get_header_map_value_size(HeaderMapType::RequestTrailers)
}
// ====================== Request Trailer Processing API ===========================

// ====================== Response Trailer Processing API ===========================
pub fn set_response_trailer_pairs(_pairs: &HashMap<String, String>) -> WasmResult {
  set_header_map_pairs(HeaderMapType::ResponseTrailers, _pairs)
}

pub fn get_response_trailer_pairs() -> Result<HashMap<String, String>, String> {
  get_header_map_pairs(HeaderMapType::ResponseTrailers)
}

pub fn get_response_trailer(key: String) -> Result<Box<WasmData>, String> {
  get_header_map_value(HeaderMapType::ResponseTrailers, key)
}

pub fn add_response_trailer(key: String, value: String) -> WasmResult {
  add_header_map_value(HeaderMapType::ResponseTrailers, key, value)
}

pub fn replace_response_trailer(key: String, value: String) -> WasmResult {
  replace_header_map_value(HeaderMapType::ResponseTrailers, key, value)
}

pub fn remove_response_trailer(key: String) -> WasmResult {
  remove_header_map_value(HeaderMapType::ResponseTrailers, key)
}

pub fn get_response_trailer_size() -> u32 {
  get_header_map_value_size(HeaderMapType::ResponseTrailers)
}
// ====================== Response Trailer Processing API ===========================

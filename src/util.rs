// Copyright 2022 Sebastien Soudan.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Utilities for the Vizier API.

use prost::DecodeError;

use crate::google::rpc::Status;
use crate::operation;

/// Error from decoding operation results.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error while decoding
    #[error("{0}")]
    DecodeError(#[from] DecodeError),
    /// RPC error
    #[error("Status: {}", .0.message)]
    RPCStatus(Status),
    /// Invalid type
    #[error("Invalid type {0}")]
    InvalidType(String),
}

/// Decodes the result of an operation as with the specified [`type_url`](Any.type_url) as
/// the provided (by the generic type parameter `X`) message.
pub fn decode_operation_result_as<X>(
    result: operation::Result,
    type_url: impl AsRef<str>,
) -> Result<X, Error>
where
    X: prost::Message + Default,
{
    match result {
        operation::Result::Error(s) => Err(Error::RPCStatus(s)),
        operation::Result::Response(resp) => {
            let t = resp.type_url.as_str();
            if t == type_url.as_ref() {
                let resp: X = X::decode(&resp.value[..])?;
                Ok(resp)
            } else {
                Err(Error::InvalidType(t.to_string()))
            }
        }
    }
}

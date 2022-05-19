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

use gouth::Token;
use tonic::metadata::AsciiMetadataValue;
use tonic::service::Interceptor;
use tonic::{Code, Status};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error while getting the token {0}")]
    Gouth(#[from] gouth::Error),
}

pub struct AuthInterceptor {
    token: Token,
}

impl AuthInterceptor {
    pub(crate) fn new() -> Result<Self, Error> {
        let token = Token::new()?;
        Ok(Self { token })
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let bearer = self.token.header_value();

        match bearer {
            Err(e) => Err(Status::new(Code::Unauthenticated, e.to_string())),
            Ok(bearer) => {
                let token = &*bearer;
                let meta = AsciiMetadataValue::try_from(token);
                match meta {
                    Err(e) => Err(Status::new(Code::InvalidArgument, e.to_string())),
                    Ok(meta) => {
                        req.metadata_mut().insert("authorization", meta);
                        Ok(req)
                    }
                }
            }
        }
    }
}

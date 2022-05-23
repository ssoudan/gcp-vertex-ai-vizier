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

//! Trial list request builder.

use crate::google::cloud::aiplatform::v1::ListTrialsRequest;
use crate::StudyName;

/// [ListTrialsRequest] builder.
pub struct RequestBuilder {
    study_name: StudyName,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl RequestBuilder {
    /// Creates a new instance of [ListTrialsRequest] builder.
    pub fn new(study_name: StudyName) -> Self {
        RequestBuilder {
            study_name,
            page_size: None,
            page_token: None,
        }
    }

    /// Sets the page size.
    pub fn with_page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets the page token to get a following page - See
    /// [`next_page_token`](ListTrialsResponse.next_page_token).
    pub fn with_page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// Builds the [ListTrialsRequest].
    pub fn build(self) -> ListTrialsRequest {
        ListTrialsRequest {
            parent: self.study_name.into(),
            page_size: self.page_size.unwrap_or(0),
            page_token: self.page_token.unwrap_or_default(),
        }
    }
}

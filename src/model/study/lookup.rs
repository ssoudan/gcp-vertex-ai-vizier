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

//! Study lookup request builder.

use crate::google::cloud::aiplatform::v1::LookupStudyRequest;

/// [LookupStudyRequest] builder.
pub struct RequestBuilder {
    project: String,
    location: String,
    display_name: String,
}

impl RequestBuilder {
    /// Creates a new instance of [LookupStudyRequest] builder.
    pub fn new(project: String, location: String, display_name: String) -> Self {
        RequestBuilder {
            project,
            location,
            display_name,
        }
    }

    /// Builds the [LookupStudyRequest].
    pub fn build(self) -> LookupStudyRequest {
        LookupStudyRequest {
            parent: format!(
                "projects/{project}/locations/{location}",
                project = self.project,
                location = self.location,
            ),
            display_name: self.display_name,
        }
    }
}

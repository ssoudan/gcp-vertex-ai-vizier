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

//! Trial list optimal request builder.

use crate::google::cloud::aiplatform::v1::ListOptimalTrialsRequest;
use crate::StudyName;

/// [ListOptimalTrialsRequest] builder.
pub struct RequestBuilder {
    study_name: StudyName,
}

impl RequestBuilder {
    /// Creates a new instance of [ListOptimalTrialsRequest] builder.
    pub fn new(study_name: StudyName) -> Self {
        RequestBuilder { study_name }
    }

    /// Builds the [ListOptimalTrialsRequest].
    pub fn build(self) -> ListOptimalTrialsRequest {
        ListOptimalTrialsRequest {
            parent: self.study_name.into(),
        }
    }
}

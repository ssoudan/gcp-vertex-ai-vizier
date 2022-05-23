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

//! Trial create request builder.

use crate::google::cloud::aiplatform::v1::CreateTrialRequest;
use crate::StudyName;

/// [CreateTrialRequest] builder.
pub struct RequestBuilder {
    study_name: StudyName,
    trial: crate::google::cloud::aiplatform::v1::Trial,
}

impl RequestBuilder {
    /// Creates a new instance of [CreateTrialRequest] builder.
    pub fn new(study_name: StudyName, trial: crate::google::cloud::aiplatform::v1::Trial) -> Self {
        RequestBuilder { study_name, trial }
    }

    /// Builds the [CreateTrialRequest].
    pub fn build(self) -> CreateTrialRequest {
        CreateTrialRequest {
            parent: self.study_name.into(),
            trial: Some(self.trial),
        }
    }
}

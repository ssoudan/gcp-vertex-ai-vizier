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

//! Trial add_measurement request builder.

use crate::google::cloud::aiplatform::v1::AddTrialMeasurementRequest;
use crate::{Measurement, TrialName};

/// [AddTrialMeasurementRequest] builder.
pub struct RequestBuilder {
    trial_name: TrialName,
    measurement: Measurement,
}

impl RequestBuilder {
    /// Creates a new instance of [AddTrialMeasurementRequest] builder.
    pub fn new(trial_name: TrialName, measurement: Measurement) -> Self {
        RequestBuilder {
            trial_name,
            measurement,
        }
    }

    /// Builds the [AddTrialMeasurementRequest].
    pub fn build(self) -> AddTrialMeasurementRequest {
        AddTrialMeasurementRequest {
            trial_name: self.trial_name.into(),
            measurement: Some(self.measurement),
        }
    }
}

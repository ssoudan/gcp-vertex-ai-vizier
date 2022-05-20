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

use crate::google::cloud::aiplatform::v1::CompleteTrialRequest;
use crate::Measurement;

pub enum FinalMeasurementOrReason {
    FinalMeasurement(Measurement),
    Reason(String),
}

pub struct RequestBuilder {
    project: String,
    location: String,
    study: String,
    trial: String,
    final_measurement: FinalMeasurementOrReason,
}

impl RequestBuilder {
    pub fn new(
        project: String,
        location: String,
        study: String,
        trial: String,
        final_measurement: FinalMeasurementOrReason,
    ) -> Self {
        RequestBuilder {
            project,
            location,
            study,
            trial,
            final_measurement,
        }
    }

    pub fn build(self) -> CompleteTrialRequest {
        match self.final_measurement {
            FinalMeasurementOrReason::FinalMeasurement(m) => CompleteTrialRequest {
                name: format!(
                    "projects/{project}/locations/{location}/studies/{study}/trials/{trial}",
                    project = self.project,
                    location = self.location,
                    study = self.study,
                    trial = self.trial,
                ),
                final_measurement: Some(m),
                ..Default::default()
            },
            FinalMeasurementOrReason::Reason(infeasible_reason) => CompleteTrialRequest {
                name: format!(
                    "projects/{project}/locations/{location}/studies/{study}/trials/{trial}",
                    project = self.project,
                    location = self.location,
                    study = self.study,
                    trial = self.trial,
                ),
                final_measurement: None,
                trial_infeasible: true,
                infeasible_reason,
            },
        }
    }
}

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

//! StudySpec builder.

use crate::google::cloud::aiplatform::v1::study_spec::{
    Algorithm, AutomatedStoppingSpec, MeasurementSelectionType, MetricSpec, ObservationNoise,
    ParameterSpec,
};
use crate::google::cloud::aiplatform::v1::StudySpec;

/// [StudySpec] builder.
pub struct StudySpecBuilder {
    metrics: Vec<MetricSpec>,
    parameters: Vec<ParameterSpec>,
    algorithm: Algorithm,
    observation_noise: ObservationNoise,
    measurement_selection_type: MeasurementSelectionType,
    automated_stopping_spec: Option<AutomatedStoppingSpec>,
}

impl StudySpecBuilder {
    /// Creates a new instance of [StudySpec] builder.
    pub fn new(
        algorithm: Algorithm,
        observation_noise: ObservationNoise,
        measurement_selection_type: MeasurementSelectionType,
    ) -> Self {
        StudySpecBuilder {
            algorithm,
            observation_noise,
            metrics: vec![],
            parameters: vec![],
            measurement_selection_type,
            automated_stopping_spec: None,
        }
    }

    /// Sets the [MetricSpec]s to the [StudySpec].
    pub fn with_metric_specs(mut self, metrics: Vec<MetricSpec>) -> Self {
        self.metrics = metrics;
        self
    }

    /// Sets the [ParameterSpec]s to the [StudySpec].
    pub fn with_parameters(mut self, parameters: Vec<ParameterSpec>) -> Self {
        self.parameters = parameters;
        self
    }

    /// Sets the [AutomatedStoppingSpec] to the [StudySpec].
    pub fn with_automated_stopping_spec(
        mut self,
        automated_stopping_spec: AutomatedStoppingSpec,
    ) -> Self {
        self.automated_stopping_spec = Some(automated_stopping_spec);
        self
    }

    /// Builds the [StudySpec].
    pub fn build(self) -> StudySpec {
        StudySpec {
            metrics: self.metrics,
            parameters: self.parameters,
            algorithm: self.algorithm as i32,
            observation_noise: self.observation_noise as i32,
            measurement_selection_type: self.measurement_selection_type as i32,
            automated_stopping_spec: self.automated_stopping_spec,
        }
    }
}

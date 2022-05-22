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

use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::study_spec::metric_spec::GoalType;
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::study_spec::parameter_spec::{
    DoubleValueSpec, ParameterValueSpec, ScaleType,
};
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::study_spec::{
    Algorithm, MeasurementSelectionType, MetricSpec, ObservationNoise, ParameterSpec,
};
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::StudySpec;
use gcp_vertex_ai_vizier::model::study::ToStudyName;
use gcp_vertex_ai_vizier::VizierClient;
use std::env;
use std::time::Duration;

/// Hammelblau's function
fn f(x: f64, y: f64) -> f64 {
    (x.powi(2) + y - 11.).powi(2) + (x + y.powi(2) - 7.).powi(2)
}

#[tokio::main]
async fn main() {
    let project = env::var("GOOGLE_CLOUD_PROJECT").unwrap();

    let location = "us-central1".to_string();

    let mut client = VizierClient::new(project.clone(), location.clone())
        .await
        .unwrap();

    let study_spec = StudySpec {
        metrics: vec![MetricSpec {
            metric_id: "m".to_string(), // TODO(ssoudan) unique and w/o whitespaces
            goal: GoalType::Minimize as i32,
        }],
        parameters: vec![
            ParameterSpec {
                parameter_id: "x".to_string(),
                scale_type: ScaleType::Unspecified as i32,
                conditional_parameter_specs: vec![],
                parameter_value_spec: Some(ParameterValueSpec::DoubleValueSpec(DoubleValueSpec {
                    min_value: -5.0,
                    max_value: 5.0,
                    default_value: Some(0.0),
                })),
            },
            ParameterSpec {
                parameter_id: "y".to_string(),
                scale_type: ScaleType::Unspecified as i32,
                conditional_parameter_specs: vec![],
                parameter_value_spec: Some(ParameterValueSpec::DoubleValueSpec(DoubleValueSpec {
                    min_value: -5.0,
                    max_value: 5.0,
                    default_value: Some(0.0),
                })),
            },
        ],
        algorithm: Algorithm::Unspecified as i32,
        observation_noise: ObservationNoise::Low as i32,
        measurement_selection_type: MeasurementSelectionType::LastMeasurement as i32,
        automated_stopping_spec: None,
    };

    let request = client
        .mk_study_request_builder()
        .with_display_name("test_hammelblau".to_string())
        .with_study_spec(study_spec)
        .build()
        .unwrap();

    match client.service.create_study(request).await {
        Ok(study_response) => {
            let study = study_response.get_ref();
            dbg!(&study);

            let study_name = study.to_study_name();

            let req =
                client.mk_suggest_trials_request(study_name, 5, "test_hammelblau".to_string());

            let trials = client.service.suggest_trials(req).await.unwrap();
            let operation = trials.into_inner();

            let result = client
                .wait_for_operation(operation, Some(Duration::from_secs(4)))
                .await;

            dbg!(result);
            // TODO(ssoudan) parse result into trials

            // TODO(ssoudan) get suggested trials
            // TODO(ssoudan) run the trials
            // TODO(ssoudan) get the best trials
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

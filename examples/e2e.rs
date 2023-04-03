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

//! End to end example of how to use VizierClient to run a study.
use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::study_spec::metric_spec::GoalType;
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::study_spec::parameter_spec::{
    DoubleValueSpec, ParameterValueSpec, ScaleType,
};
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::study_spec::{
    Algorithm, MeasurementSelectionType, MetricSpec, ObservationNoise, ParameterSpec,
};
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::trial::State;
use gcp_vertex_ai_vizier::google::cloud::aiplatform::v1::{measurement, Measurement, Trial};
use gcp_vertex_ai_vizier::model::study::spec::StudySpecBuilder;
use gcp_vertex_ai_vizier::model::study::ToStudyName;
use gcp_vertex_ai_vizier::model::trial::complete::FinalMeasurementOrReason;
use gcp_vertex_ai_vizier::model::trial::ToTrialName;
use gcp_vertex_ai_vizier::prost_types::value::Kind;
use gcp_vertex_ai_vizier::VizierClient;

/// Hammelblau's function
fn f(x: f64, y: f64) -> f64 {
    (x.powi(2) + y - 11.).powi(2) + (x + y.powi(2) - 7.).powi(2)
}

#[tokio::main]
async fn main() {
    let project = env::var("GOOGLE_CLOUD_PROJECT").expect("GOOGLE_CLOUD_PROJECT must be set");

    let location = "us-central1".to_string();

    let mut client = VizierClient::new(project, location).await.unwrap();

    let study_spec = StudySpecBuilder::new(
        Algorithm::Unspecified,
        ObservationNoise::Low,
        MeasurementSelectionType::LastMeasurement,
    )
    .with_metric_specs(vec![MetricSpec {
        metric_id: "m".to_string(),
        goal: GoalType::Minimize as i32,
    }])
    .with_parameters(vec![
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
    ])
    .build();

    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let display_name = format!("test_hammelblau_{}", epoch);
    let request = client
        .mk_study_request_builder()
        .with_display_name(display_name)
        .with_study_spec(study_spec)
        .build()
        .unwrap();

    match client.service.create_study(request).await {
        Ok(study_response) => {
            let study = study_response.get_ref();
            dbg!(&study);

            let study_name = study.to_study_name();

            for _ in 0..4 {
                // get some suggested trials
                let req = client.mk_suggest_trials_request(
                    study_name.clone(),
                    5,
                    "test_hammelblau_1".to_string(),
                );

                let resp = client.suggest_trials(req).await.unwrap();

                // run the trials
                for trial in resp.trials.iter() {
                    dbg!(&trial);

                    let parameters = extract_parameters(trial);
                    dbg!(&parameters);

                    let start = SystemTime::now();

                    let x = parameters["x"];
                    let y = parameters["y"];

                    let value = f(x, y);

                    let elapsed_duration = start.elapsed().unwrap();
                    dbg!(&value);

                    let final_measurement_or_reason =
                        FinalMeasurementOrReason::FinalMeasurement(Measurement {
                            elapsed_duration: Some(elapsed_duration.try_into().unwrap()),
                            step_count: 14,
                            metrics: vec![measurement::Metric {
                                metric_id: "m".to_string(),
                                value,
                            }],
                        });

                    let request = client.mk_complete_trial_request(
                        trial.to_trial_name(),
                        final_measurement_or_reason,
                    );

                    let trial = client.service.complete_trial(request).await.unwrap();
                    let trial = trial.get_ref();
                    dbg!(State::from_i32(trial.state).unwrap());
                }
            }

            // get the best trials
            let request = client.mk_list_optimal_trials_request(study_name.clone());

            let resp = client.service.list_optimal_trials(request).await.unwrap();
            let optimal_trials = &resp.get_ref().optimal_trials;
            for t in optimal_trials {
                dbg!(&t.name);
                dbg!(&t.final_measurement.as_ref().map(|x| x.metrics.clone()));
                let parameters = extract_parameters(t);
                dbg!(&parameters);
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn extract_parameters(trial: &Trial) -> HashMap<String, f64> {
    let mut parameters = HashMap::new();
    for p in trial.parameters.iter() {
        let p_id = p.parameter_id.clone();
        if let Some(p) = &p.value {
            if let Some(Kind::NumberValue(v)) = p.kind {
                parameters.insert(p_id, v);
            }
        }
    }
    parameters
}

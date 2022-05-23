// Copyright 2022 Sebastien Soudan.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate core;

use google_authz::GoogleAuthz;
use std::time::Duration;
use tokio::time::sleep;
use tonic::codegen::http::uri::InvalidUri;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use crate::google::cloud::aiplatform::v1::{
    AddTrialMeasurementRequest, CheckTrialEarlyStoppingStateRequest, CompleteTrialRequest,
    CreateTrialRequest, DeleteStudyRequest, DeleteTrialRequest, GetStudyRequest, GetTrialRequest,
    ListOptimalTrialsRequest, LookupStudyRequest, Measurement, StopTrialRequest,
    SuggestTrialsRequest, SuggestTrialsResponse, Trial,
};
use crate::google::longrunning::operations_client::OperationsClient;
use crate::google::longrunning::{operation, GetOperationRequest, Operation, WaitOperationRequest};
use google::cloud::aiplatform::v1::vizier_service_client::VizierServiceClient;

use crate::model::study;
use crate::model::trial;
use crate::study::StudyName;
use crate::trial::complete::FinalMeasurementOrReason;
use crate::trial::{early_stopping, optimal, stop, TrialName};

pub mod model;
pub mod util;

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }

    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }

    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }

    pub mod cloud {
        pub mod aiplatform {
            pub mod v1 {
                tonic::include_proto!("google.cloud.aiplatform.v1");
            }
        }
    }
}

#[derive(Clone)]
pub struct VizierClient {
    location: String,
    project: String,
    pub service: VizierServiceClient<GoogleAuthz<Channel>>,
    pub operation_service: OperationsClient<GoogleAuthz<Channel>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("tonic transport error - {0}")]
    Tonic(#[from] tonic::transport::Error),
    #[error("{0}")]
    InvalidUri(#[from] InvalidUri),
    #[error("{0}")]
    DecodingError(#[from] util::Error),
    #[error("Status: {}", .0.message())]
    Status(#[from] tonic::Status),
}

const CERTIFICATES: &str = include_str!("../certs/roots.pem");

impl VizierClient {
    pub async fn new(project: String, location: String) -> Result<Self, Error> {
        let domain_name = format!("{location}-aiplatform.googleapis.com", location = location);

        let service = {
            let tls_config = ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(CERTIFICATES))
                .domain_name(&domain_name);

            let endpoint = format!("https://{endpoint}", endpoint = domain_name);

            let channel = Channel::from_shared(endpoint)?
                .tls_config(tls_config)?
                .connect()
                .await?;
            let channel = GoogleAuthz::new(channel).await;

            VizierServiceClient::new(channel).send_gzip().accept_gzip()
        };

        let operation_service = {
            let tls_config = ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(CERTIFICATES))
                .domain_name(&domain_name);

            let endpoint = format!("https://{endpoint}", endpoint = domain_name);

            let channel = Channel::from_shared(endpoint)?
                .tls_config(tls_config)?
                .connect()
                .await?;
            let channel = GoogleAuthz::new(channel).await;

            OperationsClient::new(channel).send_gzip().accept_gzip()
        };

        Ok(Self {
            project,
            location,
            service,
            operation_service,
        })
    }

    pub fn mk_study_request_builder(&self) -> study::create::RequestBuilder {
        study::create::RequestBuilder::new(self.project.clone(), self.location.clone())
    }

    pub fn mk_get_study_request(&self, study_name: StudyName) -> GetStudyRequest {
        study::get::RequestBuilder::new(study_name).build()
    }

    pub fn mk_delete_study_request(&self, study_name: StudyName) -> DeleteStudyRequest {
        study::delete::RequestBuilder::new(study_name).build()
    }

    pub fn mk_lookup_study_request(&self, display_name: String) -> LookupStudyRequest {
        study::lookup::RequestBuilder::new(
            self.project.clone(),
            self.location.clone(),
            display_name,
        )
        .build()
    }

    pub fn mk_list_studies_request_builder(&self) -> study::list::RequestBuilder {
        study::list::RequestBuilder::new(self.project.clone(), self.location.clone())
    }

    pub fn mk_get_trial_request(&self, trial_name: TrialName) -> GetTrialRequest {
        trial::get::RequestBuilder::new(trial_name).build()
    }

    pub fn mk_suggest_trials_request(
        &self,
        study_name: StudyName,
        suggestion_count: i32,
        client_id: String,
    ) -> SuggestTrialsRequest {
        trial::suggest::RequestBuilder::new(study_name, suggestion_count, client_id).build()
    }

    pub fn mk_create_trial_request(
        &self,
        study_name: StudyName,
        trial: Trial,
    ) -> CreateTrialRequest {
        trial::create::RequestBuilder::new(study_name, trial).build()
    }

    pub fn mk_delete_trial_request(&self, trial_name: TrialName) -> DeleteTrialRequest {
        trial::delete::RequestBuilder::new(trial_name).build()
    }

    pub fn mk_list_trials_request_builder(
        &self,
        study_name: StudyName,
    ) -> trial::list::RequestBuilder {
        trial::list::RequestBuilder::new(study_name)
    }

    pub fn mk_add_trial_measurement_request(
        &self,
        trial_name: TrialName,
        measurement: Measurement,
    ) -> AddTrialMeasurementRequest {
        trial::add_measurement::RequestBuilder::new(trial_name, measurement).build()
    }

    pub fn mk_complete_trial_request(
        &self,
        trial_name: TrialName,
        final_measurement: FinalMeasurementOrReason,
    ) -> CompleteTrialRequest {
        trial::complete::RequestBuilder::new(trial_name, final_measurement).build()
    }

    pub fn mk_check_trial_early_stopping_state_request(
        &self,
        trial_name: TrialName,
    ) -> CheckTrialEarlyStoppingStateRequest {
        early_stopping::RequestBuilder::new(trial_name).build()
    }

    pub fn mk_stop_trial_request(&self, trial_name: TrialName) -> StopTrialRequest {
        stop::RequestBuilder::new(trial_name).build()
    }

    pub fn mk_list_optimal_trials_request(
        &self,
        study_name: StudyName,
    ) -> ListOptimalTrialsRequest {
        optimal::RequestBuilder::new(study_name).build()
    }

    pub fn trial_name(&self, study: String, trial: String) -> TrialName {
        TrialName::new(self.project.clone(), self.location.clone(), study, trial)
    }

    pub fn trial_name_from_study(
        &self,
        study_name: &StudyName,
        trial: impl Into<String>,
    ) -> TrialName {
        TrialName::from_study(study_name, trial.into())
    }

    pub fn study_name(&self, study: impl Into<String>) -> StudyName {
        StudyName::new(self.project.clone(), self.location.clone(), study.into())
    }

    pub async fn wait_for_operation(
        &mut self,
        mut operation: Operation,
        timeout: Option<Duration>,
    ) -> Option<operation::Result> {
        while !operation.done {
            sleep(Duration::from_millis(500)).await;
            let resp = self
                .operation_service
                .wait_operation(WaitOperationRequest {
                    name: operation.name.clone(),
                    timeout: timeout.map(|d| d.into()),
                })
                .await
                .unwrap(); // TODO(ssoudan) handle errors

            operation = resp.into_inner();
            dbg!(&operation);
        }

        operation.result
    }

    pub async fn get_operation(&mut self, operation_name: String) -> Option<operation::Result> {
        let resp = self
            .operation_service
            .get_operation(GetOperationRequest {
                name: operation_name,
            })
            .await
            .unwrap(); // TODO(ssoudan) handle errors

        let operation = resp.into_inner();
        dbg!(&operation);

        if operation.done {
            operation.result
        } else {
            None
        }
    }

    pub async fn suggest_trials(
        &mut self,
        request: SuggestTrialsRequest,
    ) -> Result<SuggestTrialsResponse, Error> {
        let trials = self.service.suggest_trials(request).await?;
        let operation = trials.into_inner();

        dbg!(&operation);

        let result = loop {
            if let Some(result) = self.get_operation(operation.name.clone()).await {
                break result;
            }
            sleep(Duration::from_millis(500)).await; // TODO(ssoudan) need that?
        };

        // parse the result into trials
        let resp: SuggestTrialsResponse = util::decode_operation_result_as(
            result,
            "type.googleapis.com/google.cloud.aiplatform.v1.SuggestTrialsResponse",
        )?;

        Ok(resp)
    }
}

#[cfg(test)]
mod trials {
    use super::common::test_client;
    use crate::google::cloud::aiplatform::v1::{
        measurement, CheckTrialEarlyStoppingStateResponse, Measurement,
    };
    use crate::trial::complete::FinalMeasurementOrReason;
    use crate::util::decode_operation_result_as;
    use crate::{util, SuggestTrialsResponse};
    use std::time::Duration;
    use tonic::Code;

    #[tokio::test]
    async fn it_can_get_a_trial() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let trial = "1".to_string();

        let study_name = client.study_name(study);
        dbg!(&study_name);
        let trial_name = client.trial_name_from_study(&study_name, trial);
        dbg!(&trial_name);
        let request = client.mk_get_trial_request(trial_name);

        let trial = client.service.get_trial(request).await.unwrap();
        let trial = trial.get_ref();
        dbg!(trial);
    }

    #[tokio::test]
    async fn it_deletes_a_trial() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let trial = "2".to_string();

        let study_name = client.study_name(study);
        let trial_name = client.trial_name_from_study(&study_name, trial);

        let request = client.mk_delete_trial_request(trial_name);

        match client.service.delete_trial(request).await {
            Ok(study) => {
                let study = study.get_ref();
                dbg!(study);
            }
            Err(err) => {
                // dbg!(&err);
                assert_eq!(err.code(), Code::InvalidArgument);
            }
        }
    }

    #[tokio::test]
    async fn it_suggests_trials_raw() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();

        let study_name = client.study_name(study);

        let client_id = "it_can_suggest_trials".to_string();

        let request = client.mk_suggest_trials_request(study_name, 2, client_id);

        let resp = client.service.suggest_trials(request).await.unwrap();
        let operation = resp.into_inner();

        let result = client
            .wait_for_operation(operation, Some(Duration::from_secs(4)))
            .await
            .unwrap();

        // parse the result into trials
        let resp: SuggestTrialsResponse = util::decode_operation_result_as(
            result,
            "type.googleapis.com/google.cloud.aiplatform.v1.SuggestTrialsResponse",
        )
        .unwrap();

        dbg!(&resp);

        assert_eq!(resp.trials.len(), 2);
    }

    #[tokio::test]
    async fn it_suggests_trials() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();

        let study_name = client.study_name(study);

        let client_id = "it_can_suggest_trials".to_string();

        let request = client.mk_suggest_trials_request(study_name, 2, client_id);

        let resp = client.suggest_trials(request).await.unwrap();

        dbg!(resp);
    }

    // FUTURE(ssoudan) add a test for create_trial
    // #[tokio::test]
    // async fn it_can_create_a_trial() {
    //     let mut client = test_client().await;
    //
    //     let study = "53316451264".to_string();
    //
    //     let client_id = "it_can_create_a_trial".to_string();
    //     let parameters = vec![
    //         trial::Parameter {
    //             parameter_id: "a".to_string(),
    //             value: Some(Value {
    //                 kind: Some(Kind::NumberValue(2.0)),
    //             }),
    //         },
    //         trial::Parameter {
    //             parameter_id: "b".to_string(),
    //             value: Some(Value {
    //                 kind: Some(Kind::NumberValue(9.0)),
    //             }),
    //         },
    //     ];
    //     let trial = Trial {
    //         parameters,
    //         client_id,
    //         state: trial::State::Active as i32,
    //         ..Default::default()
    //     };
    //
    //     let request = client.mk_create_trials_request(study, trial);
    //
    //     let trial = client.service.create_trial(request).await.unwrap();
    //     let trial = trial.get_ref();
    //     dbg!(trial);
    // }

    #[tokio::test]
    async fn it_lists_trials() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let study_name = client.study_name(study);

        let request = client
            .mk_list_trials_request_builder(study_name.clone())
            .with_page_size(2)
            .build();

        let trials = client.service.list_trials(request).await.unwrap();
        let trial_list = &trials.get_ref().trials;
        for t in trial_list {
            dbg!(&t);
        }

        // TODO(ssoudan) look at generators and iterators

        if !trials.get_ref().next_page_token.is_empty() {
            let mut page_token = trials.get_ref().next_page_token.clone();

            while !page_token.is_empty() {
                println!("There is more! - {:?}", &page_token);

                let request = client
                    .mk_list_trials_request_builder(study_name.clone())
                    .with_page_token(page_token)
                    .with_page_size(2)
                    .build();

                let trials = client.service.list_trials(request).await.unwrap();
                let trial_list = &trials.get_ref().trials;
                for t in trial_list {
                    dbg!(&t);
                }

                page_token = trials.get_ref().next_page_token.clone();
            }
        }
    }

    #[tokio::test]
    async fn it_can_add_trial_measurement() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let trial = "1".to_string();

        let study_name = client.study_name(study);
        let trial_name = client.trial_name_from_study(&study_name, trial);

        let measurement = Measurement {
            elapsed_duration: Some(std::time::Duration::from_secs(10).into()),
            step_count: 13,
            metrics: vec![measurement::Metric {
                metric_id: "m1".to_string(),
                value: 2.1,
            }],
        };

        let request = client.mk_add_trial_measurement_request(trial_name, measurement);

        let trial = client.service.add_trial_measurement(request).await.unwrap();
        let trial = trial.get_ref();
        dbg!(trial);
    }

    #[tokio::test]
    async fn it_can_complete_a_trial() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let trial = "3".to_string();

        let study_name = client.study_name(study);
        let trial_name = client.trial_name_from_study(&study_name, trial);

        let final_measurement_or_reason = FinalMeasurementOrReason::FinalMeasurement(Measurement {
            elapsed_duration: Some(std::time::Duration::from_secs(100).into()),
            step_count: 14,
            metrics: vec![measurement::Metric {
                metric_id: "m1".to_string(),
                value: 3.1,
            }],
        });

        let request = client.mk_complete_trial_request(trial_name, final_measurement_or_reason);

        match client.service.complete_trial(request).await {
            Ok(trial) => {
                let trial = trial.get_ref();
                dbg!(trial);
            }
            Err(e) => {
                dbg!(e);
            }
        };
    }

    #[tokio::test]
    async fn it_can_check_trial_early_stopping_state() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let trial = "3".to_string();

        let study_name = client.study_name(study);
        let trial_name = client.trial_name_from_study(&study_name, trial);

        let request = client.mk_check_trial_early_stopping_state_request(trial_name);

        let resp = client
            .service
            .check_trial_early_stopping_state(request)
            .await
            .unwrap();

        let operation = resp.into_inner();

        let result = client
            .wait_for_operation(operation, Some(Duration::from_secs(4)))
            .await;

        let resp: CheckTrialEarlyStoppingStateResponse = decode_operation_result_as(
            result.unwrap(),
            "type.googleapis.com/google.cloud.aiplatform.v1.CheckTrialEarlyStoppingStateResponse",
        )
        .unwrap();

        dbg!(resp);
    }

    #[tokio::test]
    async fn it_can_stop_a_trial() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();
        let trial = "3".to_string();

        let study_name = client.study_name(study);
        let trial_name = client.trial_name_from_study(&study_name, trial);

        let request = client.mk_stop_trial_request(trial_name);

        match client.service.stop_trial(request).await {
            Ok(trial) => {
                let trial = trial.get_ref();
                dbg!(trial);
            }
            Err(err) => {
                dbg!(err);
            }
        };
    }

    #[tokio::test]
    async fn it_lists_optimal_trials() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();

        let study_name = client.study_name(study);

        let request = client.mk_list_optimal_trials_request(study_name);

        let trials = client.service.list_optimal_trials(request).await.unwrap();
        let trial_list = &trials.get_ref().optimal_trials;
        for t in trial_list {
            dbg!(&t.name);
        }
    }
}

#[cfg(test)]
mod studies {
    use super::common::test_client;
    use tonic::Code;

    use crate::google::cloud::aiplatform::v1::study_spec::metric_spec::GoalType;
    use crate::google::cloud::aiplatform::v1::study_spec::parameter_spec::{
        DoubleValueSpec, IntegerValueSpec, ParameterValueSpec, ScaleType,
    };
    use crate::google::cloud::aiplatform::v1::study_spec::{
        Algorithm, MeasurementSelectionType, MetricSpec, ObservationNoise, ParameterSpec,
    };
    use crate::google::cloud::aiplatform::v1::StudySpec;

    #[tokio::test]
    async fn it_list_studies() {
        let mut client = test_client().await;

        let request = client
            .mk_list_studies_request_builder()
            .with_page_size(2)
            .build();

        let studies = client.service.list_studies(request).await.unwrap();
        let study_list_resp = studies.get_ref();
        let study_list = &study_list_resp.studies;
        for t in study_list {
            dbg!(&t.name);
            dbg!(&t.display_name);
        }

        // TODO(ssoudan) look at generators and iterators

        if !studies.get_ref().next_page_token.is_empty() {
            let mut page_token = studies.get_ref().next_page_token.clone();

            while !page_token.is_empty() {
                println!("There is more! - {:?}", &page_token);

                let request = client
                    .mk_list_studies_request_builder()
                    .with_page_token(page_token)
                    .with_page_size(2)
                    .build();

                let studies = client.service.list_studies(request).await.unwrap();
                let study_list = &studies.get_ref().studies;
                for t in study_list {
                    dbg!(&t.display_name);
                }

                page_token = studies.get_ref().next_page_token.clone();
            }
        }
    }

    #[tokio::test]
    async fn it_creates_studies() {
        let mut client = test_client().await;

        // TODO(ssoudan) StudySpec builder
        let study_spec = StudySpec {
            metrics: vec![MetricSpec {
                metric_id: "m1".to_string(), // TODO(ssoudan) unique and w/o whitespaces
                goal: GoalType::Maximize as i32,
            }],
            parameters: vec![
                ParameterSpec {
                    parameter_id: "a".to_string(),
                    scale_type: ScaleType::Unspecified as i32,
                    conditional_parameter_specs: vec![],
                    parameter_value_spec: Some(ParameterValueSpec::DoubleValueSpec(
                        DoubleValueSpec {
                            min_value: 0.0,
                            max_value: 12.0,
                            default_value: Some(4.0),
                        },
                    )),
                },
                ParameterSpec {
                    parameter_id: "b".to_string(),
                    scale_type: ScaleType::Unspecified as i32,
                    conditional_parameter_specs: vec![],
                    parameter_value_spec: Some(ParameterValueSpec::IntegerValueSpec(
                        IntegerValueSpec {
                            min_value: 4,
                            max_value: 10,
                            default_value: Some(7),
                        },
                    )),
                },
            ],
            algorithm: Algorithm::Unspecified as i32,
            observation_noise: ObservationNoise::Unspecified as i32,
            measurement_selection_type: MeasurementSelectionType::LastMeasurement as i32,
            automated_stopping_spec: None,
        };

        let request = client
            .mk_study_request_builder()
            .with_display_name("blah".to_string())
            .with_study_spec(study_spec)
            .build()
            .unwrap();

        match client.service.create_study(request).await {
            Ok(study_response) => {
                let study = study_response.get_ref();
                dbg!(&study);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    #[tokio::test]
    async fn it_can_get_a_study() {
        let mut client = test_client().await;

        let study = "53316451264".to_string();

        let study_name = client.study_name(study);

        let request = client.mk_get_study_request(study_name);

        let study = client.service.get_study(request).await.unwrap();
        let study = study.get_ref();
        dbg!(study);
    }

    #[tokio::test]
    async fn it_finds_a_study_by_name() {
        let mut client = test_client().await;

        let display_name = "blah".to_string();

        let request = client.mk_lookup_study_request(display_name);

        let study = client.service.lookup_study(request).await.unwrap();
        let study = study.get_ref();
        dbg!(study);
    }

    #[tokio::test]
    async fn it_deletes_a_study() {
        let mut client = test_client().await;

        let study = "53316451265".to_string();
        let study_name = client.study_name(study);

        let request = client.mk_delete_study_request(study_name);

        match client.service.delete_study(request).await {
            Ok(study) => {
                let study = study.get_ref();
                dbg!(study);
            }
            Err(err) => {
                assert_eq!(err.code(), Code::NotFound);
            }
        }
    }
}

#[cfg(test)]
mod common {
    use crate::VizierClient;
    use std::env;

    pub(crate) async fn test_client() -> VizierClient {
        let project = env::var("GOOGLE_CLOUD_PROJECT").unwrap();

        let location = "us-central1".to_string();

        VizierClient::new(project.clone(), location.clone())
            .await
            .unwrap()
    }
}

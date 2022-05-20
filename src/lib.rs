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

use google_authz::GoogleAuthz;
use tonic::codegen::http::uri::InvalidUri;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use google::cloud::aiplatform::v1::vizier_service_client::VizierServiceClient;
use model::study::CreateStudyRequestBuilder;

use crate::model::study::ListStudiesRequestBuilder;

mod model;

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

pub struct VizierClient {
    location: String,
    project: String,
    pub service: VizierServiceClient<GoogleAuthz<Channel>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("tonic transport error - {0}")]
    Tonic(#[from] tonic::transport::Error),
    #[error("{0}")]
    InvalidUri(#[from] InvalidUri),
}

const CERTIFICATES: &str = include_str!("../certs/roots.pem");

impl VizierClient {
    pub async fn new(project: String, location: String) -> Result<Self, Error> {
        let domain_name = format!("{location}-aiplatform.googleapis.com", location = location);

        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name(&domain_name);

        let endpoint = format!("https://{endpoint}", endpoint = domain_name);

        let channel = Channel::from_shared(endpoint)?
            .tls_config(tls_config)?
            .connect()
            .await?;
        let channel = GoogleAuthz::new(channel).await;

        let service = VizierServiceClient::new(channel); // .send_gzip().accept_gzip();

        Ok(Self {
            project,
            location,
            service,
        })
    }

    pub fn create_study_request_builder(&self) -> CreateStudyRequestBuilder {
        CreateStudyRequestBuilder::new(self.project.clone(), self.location.clone())
    }

    pub fn create_list_studies_request_builder(&self) -> ListStudiesRequestBuilder {
        ListStudiesRequestBuilder::new(self.project.clone(), self.location.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

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
        let project = env::var("GOOGLE_CLOUD_PROJECT").unwrap();

        let location = "us-central1".to_string();

        let mut client = super::VizierClient::new(project.clone(), location.clone())
            .await
            .unwrap();

        let request = client
            .create_list_studies_request_builder()
            .with_page_size(2)
            .build();

        let studies = client.service.list_studies(request).await.unwrap();
        let study_list = &studies.get_ref().studies;
        for t in study_list {
            dbg!(&t.display_name);
        }

        // TODO(ssoudan) look at generators and iterators

        if !studies.get_ref().next_page_token.is_empty() {
            let mut token = studies.get_ref().next_page_token.clone();

            while !token.is_empty() {
                println!("There is more! - {:?}", &token);

                let request = client
                    .create_list_studies_request_builder()
                    .with_page_token(studies.get_ref().next_page_token.clone())
                    .with_page_size(2)
                    .build();

                let studies = client.service.list_studies(request).await.unwrap();
                let study_list = &studies.get_ref().studies;
                for t in study_list {
                    dbg!(&t.display_name);
                }

                token = studies.get_ref().next_page_token.clone();
            }
        }
    }

    #[tokio::test]
    async fn it_creates_studies() {
        let project = env::var("GOOGLE_CLOUD_PROJECT").unwrap();

        let location = "us-central1".to_string();

        let mut client = super::VizierClient::new(project.clone(), location.clone())
            .await
            .unwrap();

        // let req = ListStudiesRequest {
        //     parent: format!("projects/{project}/locations/{location}").to_string(),
        //     page_token: "".to_string(),
        //     page_size: 0,
        // };
        //
        // let trials = client.service.list_studies(req).await.unwrap();
        // let trial_list = &trials.get_ref().studies;
        // for t in trial_list {
        //     println!("{:?}", *t);
        // }
        //
        // // TODO(ssoudan) look at generators and iterators
        //
        // if !trials.get_ref().next_page_token.is_empty() {
        //     println!("{:?}", trials.get_ref().next_page_token);
        // }

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
            .create_study_request_builder()
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
}

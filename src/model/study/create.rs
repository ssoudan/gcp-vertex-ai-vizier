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

//! Study create request builder.

use regex::Regex;

use crate::google::cloud::aiplatform::v1::{CreateStudyRequest, Study, StudySpec};

/// Error returned by [RequestBuilder].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The display_name is invalid.
    #[error("display_name must match [a-z][a-z0-9_]*")]
    InvalidDisplayName,
    /// Display_name is missing
    #[error("display_name is required")]
    DisplayNameRequired,
    /// Study_spec is missing
    #[error("study_spec is required")]
    StudySpecRequired,
    /// Study_spec is missing or Display_name is missing
    #[error("study_spec and display_name is required")]
    StudySpecAndDisplayNameRequired,
}

/// [CreateStudyRequest] builder.
pub struct RequestBuilder {
    project: String,
    location: String,
    display_name: Option<String>,
    study_spec: Option<StudySpec>,
}

impl RequestBuilder {
    /// Create a new [RequestBuilder] for the given project and location.
    pub fn new(project: String, location: String) -> Self {
        Self {
            project,
            location,
            display_name: None,
            study_spec: None,
        }
    }

    /// Set the display name - required.
    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    /// Set the study spec - required.
    pub fn with_study_spec(mut self, study_spec: StudySpec) -> Self {
        self.study_spec = Some(study_spec);
        self
    }

    /// Builds the [CreateStudyRequest].
    pub fn build(self) -> Result<CreateStudyRequest, Error> {
        match (self.display_name, self.study_spec) {
            (Some(display_name), Some(study_spec)) => {
                let re = Regex::new(r"^[a-z][a-z\d_]*$").unwrap();
                if !re.is_match(display_name.as_str()) {
                    return Err(Error::InvalidDisplayName);
                }

                Ok(CreateStudyRequest {
                    parent: format!(
                        "projects/{project}/locations/{location}",
                        project = &self.project,
                        location = &self.location
                    ),
                    study: Some(Study {
                        display_name,
                        study_spec: Some(study_spec),
                        ..Default::default()
                    }),
                })
            }

            (None, Some(_)) => Err(Error::DisplayNameRequired),
            (Some(_), None) => Err(Error::StudySpecRequired),
            (None, None) => Err(Error::StudySpecAndDisplayNameRequired),
        }
    }
}

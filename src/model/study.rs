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

//! Study model.

use crate::google::cloud::aiplatform::v1::Study;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod lookup;
pub mod spec;

/// The name of a study.
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct StudyName(String);

impl StudyName {
    /// Creates a new StudyName from its parts.
    pub fn new(project: String, location: String, study: String) -> Self {
        StudyName(format!(
            "projects/{}/locations/{}/studies/{}",
            project, location, study
        ))
    }
}

/// Can be converted to a [StudyName].
pub trait ToStudyName {
    /// Converts this object to a [StudyName].
    fn to_study_name(&self) -> StudyName;
}

impl ToStudyName for Study {
    fn to_study_name(&self) -> StudyName {
        StudyName(self.name.clone())
    }
}

impl From<StudyName> for String {
    fn from(study_name: StudyName) -> String {
        study_name.0
    }
}

impl From<&StudyName> for String {
    fn from(study_name: &StudyName) -> String {
        study_name.0.clone()
    }
}

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

use crate::{StudyName, Trial};

pub mod add_measurement;
pub mod complete;
pub mod create;
pub mod delete;
pub mod early_stopping;
pub mod get;
pub mod list;
pub mod optimal;
pub mod stop;
pub mod suggest;

#[derive(Clone, PartialEq, Debug)]
pub struct TrialName(String);

impl TrialName {
    pub fn new(project: String, location: String, study: String, trial: String) -> Self {
        TrialName(format!(
            "projects/{}/locations/{}/studies/{}/trials/{}",
            project, location, study, trial
        ))
    }

    pub fn from_study(study_name: &StudyName, trial: String) -> Self {
        let study: String = study_name.into();
        TrialName(format!("{}/trials/{}", study, trial))
    }
}

pub trait ToTrialName {
    fn to_trial_name(&self) -> TrialName;
}

impl ToTrialName for Trial {
    fn to_trial_name(&self) -> TrialName {
        TrialName(self.name.clone())
    }
}

impl From<TrialName> for String {
    fn from(trial_name: TrialName) -> Self {
        trial_name.0
    }
}

impl From<&TrialName> for String {
    fn from(trial_name: &TrialName) -> Self {
        trial_name.0.clone()
    }
}

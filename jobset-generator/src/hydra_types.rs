use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct HydraJobsetInput {
    pub r#type: String,
    pub value: String,
    pub emailresponsible: bool,
}

pub type JobInputCollection = BTreeMap<String, HydraJobsetInput>;

pub struct HydraJobsetLegacy {
    pub nixexprinput: String,
    pub nixexprpath: String,
    pub inputs: JobInputCollection,
}

pub struct HydraJobsetFlake {
    pub flake_uri: String,
}

pub enum HydraInputDefinition {
    Legacy(HydraJobsetLegacy),
    Flake(HydraJobsetFlake),
}

pub struct HydraJobset {
    pub enabled: bool,
    pub hidden: bool,
    pub description: String,
    pub checkinterval: u64,
    pub schedulingshares: u64,
    pub enableemail: bool,
    pub emailoverride: String,
    pub keepnr: u64,
    pub definition: HydraInputDefinition,
}

#[derive(Debug, Serialize)]
pub struct FlattenedHydraJobset {
    pub enabled: bool,
    pub hidden: bool,
    pub description: String,
    pub checkinterval: u64,
    pub schedulingshares: u64,
    pub enableemail: bool,
    pub emailoverride: String,
    pub keepnr: u64,
    pub flake: Option<String>,
    pub nixexprinput: Option<String>,
    pub nixexprpath: Option<String>,
    pub inputs: Option<JobInputCollection>,
}

impl HydraJobset {
    pub fn flatten(self) -> FlattenedHydraJobset {
        let mut job = FlattenedHydraJobset {
            enabled: self.enabled,
            hidden: self.hidden,
            description: self.description,
            checkinterval: self.checkinterval,
            schedulingshares: self.schedulingshares,
            enableemail: self.enableemail,
            emailoverride: self.emailoverride,
            keepnr: self.keepnr,
            flake: None,
            inputs: None,
            nixexprinput: None,
            nixexprpath: None,
        };

        match self.definition {
            HydraInputDefinition::Flake(flake) => {
                job.flake = Some(flake.flake_uri);
            }
            HydraInputDefinition::Legacy(legacy) => {
                job.inputs = Some(legacy.inputs);
                job.nixexprinput = Some(legacy.nixexprinput);
                job.nixexprpath = Some(legacy.nixexprpath);
            }
        }

        job
    }
}

pub type HydraJobsets = BTreeMap<String, FlattenedHydraJobset>;

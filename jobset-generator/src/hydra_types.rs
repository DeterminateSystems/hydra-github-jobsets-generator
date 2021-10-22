use std::collections::BTreeMap;

pub struct HydraJobInput {
    pub r#type: String,
    pub value: String,
    pub emailresponsible: bool,
}

pub type JobInputCollection = BTreeMap<String, HydraJobInput>;

pub struct HydraJobLegacy {
    pub nixexprinput: String,
    pub nixexprpath: String,
    pub inputs: JobInputCollection,
}

pub struct HydraJobFlake {
    pub flake_uri: String,
}

pub enum HydraInputDefinition {
    Legacy(HydraJobLegacy),
    Flake(HydraJobFlake),
}

pub struct HydraJob {
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

pub struct FlattenedHydraJob {
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

impl HydraJob {
    pub fn flatten(self) -> FlattenedHydraJob {
        let mut job = FlattenedHydraJob {
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

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct HydraJobsetInput {
    pub r#type: String,
    pub value: String,
    pub emailresponsible: bool,
}

pub type JobInputCollection = BTreeMap<String, HydraJobsetInput>;

#[derive(Clone)]
pub struct HydraJobsetLegacy {
    pub nixexprinput: String,
    pub nixexprpath: String,
    pub inputs: JobInputCollection,
}

#[derive(Clone)]
pub struct HydraJobsetFlake {
    pub flake_uri: String,
}

#[derive(Clone)]
pub enum HydraInputDefinition {
    Legacy(HydraJobsetLegacy),
    Flake(HydraJobsetFlake),
}

#[derive(Clone)]
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
    pub flake: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nixexprinput: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nixexprpath: Option<String>,
    pub inputs: JobInputCollection,
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
            flake: String::from(""),
            inputs: BTreeMap::new(),
            nixexprinput: None,
            nixexprpath: None,
        };

        match self.definition {
            HydraInputDefinition::Flake(flake) => {
                job.flake = flake.flake_uri;
            }
            HydraInputDefinition::Legacy(legacy) => {
                job.inputs = legacy.inputs;
                job.nixexprinput = Some(legacy.nixexprinput);
                job.nixexprpath = Some(legacy.nixexprpath);
            }
        }

        job
    }
}

pub type HydraJobsets = BTreeMap<String, FlattenedHydraJobset>;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::hydra_types::HydraJobsetFlake;

    use super::{HydraInputDefinition, HydraJobset, HydraJobsetLegacy};

    #[test]
    fn hydrajobset_flatten_legacy() {
        let defn = HydraJobsetLegacy {
            nixexprinput: String::from("asdf"),
            nixexprpath: String::from("fdsa"),
            inputs: BTreeMap::new(),
        };
        let jobset = HydraJobset {
            enabled: true,
            hidden: false,
            description: String::from("Some description"),
            checkinterval: 299,
            schedulingshares: 2,
            enableemail: false,
            emailoverride: String::from(""),
            keepnr: 5,
            definition: HydraInputDefinition::Legacy(defn.clone()),
        };

        let flat = jobset.clone().flatten();
        let json = serde_json::to_string_pretty(&flat).unwrap();

        assert_eq!(
            (
                jobset.enabled,
                jobset.hidden,
                jobset.description,
                jobset.checkinterval,
                jobset.schedulingshares,
                jobset.enableemail,
                jobset.emailoverride,
                jobset.keepnr,
            ),
            (
                flat.enabled,
                flat.hidden,
                flat.description,
                flat.checkinterval,
                flat.schedulingshares,
                flat.enableemail,
                flat.emailoverride,
                flat.keepnr,
            )
        );

        assert_eq!(flat.nixexprinput, Some(defn.nixexprinput));
        assert_eq!(flat.nixexprpath, Some(defn.nixexprpath));
        assert_eq!(flat.flake, "");
        assert!(flat.inputs.is_empty());

        assert_eq!(
            json,
            r#"{
  "enabled": true,
  "hidden": false,
  "description": "Some description",
  "checkinterval": 299,
  "schedulingshares": 2,
  "enableemail": false,
  "emailoverride": "",
  "keepnr": 5,
  "flake": "",
  "nixexprinput": "asdf",
  "nixexprpath": "fdsa",
  "inputs": {}
}"#
        );
    }

    #[test]
    fn hydrajobset_flatten_flake() {
        let defn = HydraJobsetFlake {
            flake_uri: String::from("fake/uri"),
        };
        let jobset = HydraJobset {
            enabled: false,
            hidden: true,
            description: String::from("Another description"),
            checkinterval: 298,
            schedulingshares: 5,
            enableemail: true,
            emailoverride: String::from("asdf@asdf.asdf"),
            keepnr: 9,
            definition: HydraInputDefinition::Flake(defn.clone()),
        };

        let flat = jobset.clone().flatten();
        let json = serde_json::to_string_pretty(&flat).unwrap();

        assert_eq!(
            (
                jobset.enabled,
                jobset.hidden,
                jobset.description,
                jobset.checkinterval,
                jobset.schedulingshares,
                jobset.enableemail,
                jobset.emailoverride,
                jobset.keepnr,
            ),
            (
                flat.enabled,
                flat.hidden,
                flat.description,
                flat.checkinterval,
                flat.schedulingshares,
                flat.enableemail,
                flat.emailoverride,
                flat.keepnr,
            )
        );

        assert_eq!(flat.nixexprinput, None);
        assert_eq!(flat.nixexprpath, None);
        assert_eq!(flat.flake, defn.flake_uri);
        assert!(flat.inputs.is_empty());

        assert_eq!(
            json,
            r#"{
  "enabled": false,
  "hidden": true,
  "description": "Another description",
  "checkinterval": 298,
  "schedulingshares": 5,
  "enableemail": true,
  "emailoverride": "asdf@asdf.asdf",
  "keepnr": 9,
  "flake": "fake/uri",
  "inputs": {}
}"#
        );
    }
}

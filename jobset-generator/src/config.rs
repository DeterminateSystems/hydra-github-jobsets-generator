use crate::hydra_types::JobInputCollection;

pub struct JobConfig {
    pub checkinterval: u64,
    pub emailoverride: String,
    pub enableemail: bool,
    pub keepnr: u64,
    pub schedulingshares: u64,
    pub input_template: JobInputCollection,
    pub email_responsible: bool,
    pub inputname: String,
    pub inputpath: String,
}

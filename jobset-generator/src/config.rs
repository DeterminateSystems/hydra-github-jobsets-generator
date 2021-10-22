use crate::hydra_types;

struct JobConfig {
    checkinterval: u64,
    emailoverride: String,
    enableemail: bool,
    keepnr: u64,
    schedulingshares: u64,
    input_template: hydra_types::JobInputCollection,
    email_responsible: bool,
    inputname: String,
    inputpath: String,
}

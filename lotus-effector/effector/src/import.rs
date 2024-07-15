use marine_rs_sdk::{marine, MountedBinaryResult};

// Here we need to import the mounted binary in the module using `host_import`
#[marine]
#[host_import]
extern "C" {
    pub fn lotus(cmd: Vec<String>) -> MountedBinaryResult;
}

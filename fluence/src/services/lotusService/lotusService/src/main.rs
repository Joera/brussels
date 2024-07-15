#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

// use lotus_effector_imports as lotus; //lotus_cmd;

#[marine]
#[derive(Clone, Debug)]
pub struct LotusResult {
    /// True when the binary executed successfully.
    pub success: bool,
    /// Error message if the binary execution failed.
    pub error: String,
    /// List of files in the provided directory.
    pub result: Vec<String>,
}

impl<E: ToString> From<Result<Vec<String>, E>> for LotusResult {
    fn from(res: Result<Vec<String>, E>) -> Self {
        match res {
            Ok(result) => LotusResult {
                success: true,
                error: "".to_string(),
                result,
            },
            Err(e) => LotusResult {
                success: false,
                error: e.to_string(),
                result: vec![],
            },
        }
    }
}

module_manifest!();

pub fn main() {}

#[marine]
pub fn test() -> bool {

    true
}

#[marine]
#[module_import("lotus_effector")]
extern "C" {
    pub fn cmd(cmd: Vec<String>) -> LotusResult;
}
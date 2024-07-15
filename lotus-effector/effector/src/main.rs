#![feature(try_blocks)]
#![feature(assert_matches)]
#![allow(improper_ctypes)]
#![allow(non_snake_case)]

mod import;

use eyre::{eyre, Result};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use crate::import::lotus;

module_manifest!();

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

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Debug)
        .build()
        .unwrap();
}

fn run_lotus(cmd: Vec<String>) -> Result<String> {
    log::debug!("Running lotus with arguments: {:?}", cmd);
    let result = lotus(cmd.clone());
    log::debug!("Got result: {:?}", result.stringify());

    result
        .into_std()
        .ok_or(eyre!("stdout or stderr contains non valid UTF8 string"))?
        .map_err(|e| eyre!("lotus call failed \n{:?}: {}", cmd.join(" "), e))
}


#[marine]
pub fn cmd(cmd: Vec<String>) -> LotusResult {
    lotus_impl(cmd).into()
}

fn lotus_impl(cmd: Vec<String>) -> Result<Vec<String>> {
    let result = run_lotus(cmd)?;
    Ok(result.lines().map(|s| s.to_string()).collect())
}


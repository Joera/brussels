pub use lotus_effector_types::*;
use marine_rs_sdk::marine;

#[marine]
#[module_import("lotus_effector")]
extern "C" {
    pub fn cmd(cmd: Vec<String>) -> LotusResult;
}

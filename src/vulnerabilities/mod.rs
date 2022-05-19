pub mod processes;
pub mod structs;

use crate::footprint::structs::FootPrint;
use processes::{merge_footprint_with_vulneralbilities, read_vulnerabilities};
use structs::VulnerabilityFootPrint;

pub fn merge_vulnerablities_wite_footprints(
    footprints: Vec<FootPrint>,
    base_path: String,
) -> Vec<VulnerabilityFootPrint> {
    let vulnerabilities = read_vulnerabilities(base_path).unwrap();

    merge_footprint_with_vulneralbilities(vulnerabilities, footprints)
}

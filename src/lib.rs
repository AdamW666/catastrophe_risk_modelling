use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

mod footprint;
mod vulnerabilities;

use footprint::merge_event_ids_with_footprint;
use vulnerabilities::merge_vulnerablities_wite_footprints;

#[pyfunction]
fn get_model<'a>(event_ids: Vec<i32>, base_path: String, py: Python) -> Vec<&PyDict> {
    let footprints = merge_event_ids_with_footprint(event_ids, base_path.clone());

    let model = merge_vulnerablities_wite_footprints(footprints, base_path);

    let mut buffer = Vec::new();

    for i in model {
        let placeholder = PyDict::new(py);
        placeholder
            .set_item("vulnerability_id", i.vulnerability_id)
            .unwrap();
        placeholder
            .set_item("intensity_bin_id", i.intensity_bin_id)
            .unwrap();
        placeholder
            .set_item("damage_bin_id", i.damage_bin_id)
            .unwrap();
        placeholder
            .set_item("damage_probability", i.damage_probability)
            .unwrap();
        placeholder.set_item("event_id", i.event_id).unwrap();
        placeholder
            .set_item("areaperil_id", i.areaperil_id)
            .unwrap();
        placeholder
            .set_item("footprint_probability", i.footprint_probability)
            .unwrap();
        placeholder
            .set_item("total_probability", i.total_probability)
            .unwrap();
        buffer.push(placeholder);
    }

    buffer
}

#[pymodule]
fn catastrophe_risk_modelling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_model))?;

    Ok(())
}

pub mod processes;
pub mod structs;

use processes::{merge_footprint_with_events, read_footprint};
use structs::FootPrint;

pub fn merge_event_ids_with_footprint(event_ids: Vec<i32>, base_path: String) -> Vec<FootPrint> {
    let fp = read_footprint(base_path).unwrap();

    return merge_footprint_with_events(event_ids, fp);
}

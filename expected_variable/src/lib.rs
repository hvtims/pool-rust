use edit_distance::edit_distance;
use convert_case::{Case, Casing};

pub fn expected_variable(origin: &str, expected: &str) -> Option<String> {
    let origin_lower = origin.to_lowercase();
    let expected_lower = expected.to_lowercase();
    if origin_lower == origin_lower.to_case(Case::Camel) || origin_lower == origin_lower.to_case(Case::Snake) {
        let distance = edit_distance(&origin_lower, &expected_lower);
        let percentage = ((expected.len() as f64 - distance as f64) * 100.0) / expected.len() as f64;

        return (percentage > 50.0).then(|| format!("{}%", percentage.round()));
    }

    None
}

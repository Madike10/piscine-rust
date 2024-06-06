extern crate case;
use edit_distance::edit_distance;

pub fn expected_variable(original: &str, expected: &str) -> Option<String>{
    if original.contains(" ") {
        return None;
    }
    // println!("{}",!original.chars().any(|c| c.is_ascii_uppercase()));
    if !original.contains('_') && !original.chars().any(|c| c.is_ascii_uppercase()) {
        None
    } else {
        
        let diff = edit_distance(&original.to_lowercase(),&expected.to_lowercase());
        if diff > original.len() {
            return None;
        }
        let bigger = std::cmp::max(original.len(), expected.len());
        let res = ((bigger-diff)*100) as f64/bigger as f64;
        let resu = res.ceil();
        if resu < 50.0 {
            return None;
        }
        return Some(resu.to_string()+&"%".to_string());
    }
}

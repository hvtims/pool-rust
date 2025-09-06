use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max = -999999;
    for (_ , v) in h{
        if v > max{
            max = v ;
        }
    }
    return max
}
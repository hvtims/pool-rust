use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    let sum : i32 = list.iter().sum();
    let finish = sum as f64 / list.len() as f64;
    return finish
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    println!("{:?}" , sorted);
    if sorted.len() % 2 != 0 {
        let median = sorted[list.len()/2];
        return median;
    }
        let wstani = sorted[list.len()/2];
        let sahbwstani = sorted[list.len()/2 - 1];    
    return (wstani + sahbwstani) / 2
}

pub fn mode(list: &[i32]) -> i32 {
    let mut hashihasi = HashMap::new();
    for i in list {
        if !hashihasi.contains_key(i){
            hashihasi.insert(i , 1);
        }else{
            let count = hashihasi.get(i).unwrap();
            hashihasi.insert(i ,count +1 );
        }
    }
    let mut max = -999999;
    let mut key = 0;
    for (i , value) in hashihasi{
        if value > max{
            max = value ;
            key = *i ;
        }
    }
    return key
}


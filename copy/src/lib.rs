pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return (c , (c as f64).exp() , (c.abs() as f64).ln()  )
}

pub fn str_function(a: String) -> (String, String) {
    let splited : Vec<_>= a.split(" ").collect();
    let mut ok = String::new();
    for i in splited{
        let mut opp : f64;
        opp = i.parse().unwrap();
        opp = opp.exp();
        ok += &opp.to_string();
        ok+= " ";
    }
    
    return (a , ok);

}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut newvec : Vec<f64>  = vec![];
    for i in &b {
        let mut opp: f64  = 0.0;
        opp = (i.abs() as f64).ln();
        newvec.push(opp);
    }
    println!("{:?}" , newvec);
    return (b , newvec )
}
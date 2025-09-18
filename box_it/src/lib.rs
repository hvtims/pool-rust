pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut res : Vec<Box<u32>> = Vec::new();
        let stringg = s.split(" ").collect::<Vec<_>>();
        for i in stringg{
            if i.ends_with("k"){
               let mut ok = i.to_string();
               ok.pop();
               let pl = ok.parse::<f32>().unwrap(); 
                res.push(Box::new((pl * 1000.0) as u32));
            }else{
                 let pl = i.parse::<f32>().unwrap(); 
                res.push(Box::new((pl * 1000.0) as u32));
            }
        }
        return res

}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut veco = vec![];
    for i in a {
        veco.push(*i);
    }
    return veco 
}
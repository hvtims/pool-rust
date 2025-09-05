pub fn to_url(s: &str) -> String {
    let mut ghadir = String::new();
    for i in s.chars(){
        if i == ' '{
            ghadir+= "%20";
            continue;
        }
        ghadir+= &i.to_string();
    }
    return ghadir
}
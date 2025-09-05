pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut ghadir = vec![];
    for i in names {
        let mut temp = String::new();
        for k in i.split(" ").collect::<Vec<_>>() {
            temp.push(k.chars().next().unwrap());
            temp.push('.');

            temp.push(' ');
        }
        temp.pop();
        ghadir.push(temp);
    }
    return ghadir
}

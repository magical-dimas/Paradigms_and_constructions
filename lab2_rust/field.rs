use std::collections::HashMap;

struct Field{
    index: usize,
    maps: Box<Vec<HashMap<String, String>>>,
    keys: Vec<String>,
}

impl Iterator for Field{
    type Item = HashMap<String, String>;
    fn next(&mut self)->Option<Self::Item>{
        while self.index<(*self.maps).len(){
            let unit = &self.maps[self.index];
            let mut res = HashMap::new();
            for k in &self.keys{
                if unit.contains_key(k){
                    res.insert(k.to_string(), unit.get(k).unwrap().clone());
                }
            }
            self.index+=1;
            if !res.is_empty() {return Some(res);}
        }
        None
    }
}

trait Creatable{
    fn new(maps: Box<Vec<HashMap<String, String>>>, keys: Vec<String>) -> Self;
}

impl Creatable for Field{
    fn new(maps: Box<Vec<HashMap<String, String>>>, keys: Vec<String>) -> Self{
        Field{index: 0, maps, keys}
    }
}

fn field(maps: Box<Vec<HashMap<String, String>>>, keys: Vec<String>) -> Field{
    Field::new(maps, keys)
}

fn main() {
    let goods = vec![
        HashMap::from([
            ("title".to_string(), "Ковер".to_string()),
            ("price".to_string(), "2000".to_string()),
            ("color".to_string(), "green".to_string()),
        ]),
        HashMap::from([
            ("title".to_string(), "Диван для отдыха".to_string()),
            ("color".to_string(), "black".to_string()),
        ]),
    ];
    let boxed = Box::new(goods);
    let titles: Vec<_> = field(boxed.clone(), vec!["title".to_string()]).collect();
    for unit in titles {
        println!("{}", unit.get("title").unwrap());
    }
    println!("");
    let items: Vec<_> = field(boxed.clone(), vec!["title".to_string(), "price".to_string()]).collect();
    for item in items {
        println!("{:?}", item);
    }
}
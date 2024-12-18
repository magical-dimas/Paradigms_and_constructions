use serde_json::Value;
use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use logger_macros::log_duration;

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}


fn f1(profs: &Vec<Value>) -> Vec<String>{
    let mut professions: Vec<String> = profs.iter().map(|v| v["job-name"].to_string().to_lowercase().replace("\"", "")).collect::<Vec<_>>();
    professions.sort();
    professions.dedup();
    professions
}

#[log_duration]
fn f2(profs: Vec<String>) -> Vec<String>{
    profs.into_iter().filter(|v| v.starts_with("программист")).collect::<Vec<_>>()
}

#[log_duration]
fn f3(profs: Vec<String>) -> Vec<String>{
    profs.into_iter().map(|v| v+" с опытом Python").collect::<Vec<_>>()
}

#[log_duration]
fn f4(profs: Vec<String>) -> Vec<String>{
    let mut rng = rand::thread_rng();
    let salaries = (0..profs.len()).map(|_| rng.gen_range(100000..200000)).collect::<Vec<_>>();
    profs.into_iter().zip(salaries).map(|(v, u)| format!("{}, зарплата {} руб.", v, u)).collect()
}

fn main() {
    let data = read_file("/Users/hunter/sem3/lab2_rust/data_light.json").expect("Invalid file");
    let values: Vec<Value> = serde_json::from_str(&data).expect("Invalid JSON");
    let u: Vec<String> = f4(f3(f2(f1(&values))));
    for i in 0..u.len(){
        println!("Job name: {}", u[i]);
    }
}

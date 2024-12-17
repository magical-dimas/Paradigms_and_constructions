struct Unique{
    
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return;
    }

    let ignore_case = true;
    let unique = Unique::new(args.into_iter(), ignore_case);

    for item in unique {
        println!("{}", item);
    }
}

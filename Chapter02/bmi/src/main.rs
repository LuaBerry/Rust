fn main() {
    let mut height:f32;
    loop {

    }

}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Input err");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,

        
        Err(_) => def,
    }
}

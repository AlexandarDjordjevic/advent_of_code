use std::path::Path;
mod module;

#[path = "../../parser"]
mod parser {
    pub mod parser;
}

fn main() {
    #[allow(unused)]
    let data = Path::new("src/y2016/day2/data.txt");
    let test_string = parser::parser::parse_input_file(data);
    let result = module::part_1(&test_string);
    println!("Result part 1: {}", result);
    let result2 = module::part_2(&test_string);
    println!("Result part 2: {}", result2);
}

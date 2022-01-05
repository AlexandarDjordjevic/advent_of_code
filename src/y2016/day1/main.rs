use std::path::Path;
mod module;

#[path = "../../parser"]
mod parser {
    pub mod parser;
}

fn main() {
    let data = Path::new("src/y2016/day1/data.txt");
    #[allow(unused)]
    let commands = parser::parser::parse_input_file(&data);
    //     let mut position = module::Position::new(module::Direction::North, 0, 0);
    //     for command in commands {
    //         position.advance(&command);
    //     }
    //     println!("Position {}", position);
    //     println!(
    //         "Distance: {}",
    //         position.get_distance_from(module::Position::new(module::Direction::North, 0, 0))
    //     );
}

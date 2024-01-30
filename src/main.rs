use wasm_game_of_life::Universe;

struct MyStruct {
    s: String,
}

impl MyStruct {
    fn retrun_s(&self) -> &String {
        &self.s
    }
}

fn main() {
    let universe = Universe::new();
    println!("{}", universe.to_string())
}

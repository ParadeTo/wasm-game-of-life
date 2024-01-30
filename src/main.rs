use wasm_game_of_life::Universe;

fn main() {
    let universe = Universe::new();
    println!("{}", universe.to_string())
}

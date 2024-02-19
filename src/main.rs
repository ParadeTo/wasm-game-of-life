use wasm_game_of_life::Universe;

struct MyStruct {
    s: String,
}

struct A {
    foo: String,
}

impl A {
    fn play<'a>(&self, a: &'a str, b: &str) -> &str {
        &self.foo
    }
}

fn main() {
    // let universe = Universe::new();
    // println!("{}", universe.to_string())
    // let s = A {
    //     foo: "ddd".to_string(),
    // };
    // s.play();
    // println!("{}", s.s)

    let a = A {
        foo: "dd".to_string(),
    };
    println!("{}", a.foo);
    let b = a;
    println!("{}", b.foo)
}

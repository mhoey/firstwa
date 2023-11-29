use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    return format!("Hello, {}!", name).to_string();
}

#[wasm_bindgen]
pub fn todos(number: u16) -> Vec<String> {
    let mut todos: Vec<String> = Vec::new();

    for n in 1..number+1 {
        let plural = match n > 1 { true => "s", false => "" };
        todos.push(format!("{}. Thing{} to do", n, plural))
    }

    return todos;
}
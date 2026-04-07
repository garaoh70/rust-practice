#[allow(warnings)]
mod bindings;

use bindings::exports::garaoh70::greet::sayable::Guest;
use bindings::garaoh70::greet::greetable::{name, greet};

struct Component;

impl Guest for Component {
    fn say() -> String {
        let name = name();
        let greeting = greet(&name);
        let mut buffer = Vec::new();

        ferris_says::say(&greeting, 80, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

bindings::export!(Component with_types_in bindings);

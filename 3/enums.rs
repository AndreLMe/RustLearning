#![allow(dead_code)]
enum WebEvent {
    // An `enum` may either be 'unit-like',
    PageLoad,
    PageUnload,
    //tuple structs,
    KeyPress(char),
    Paste(String),
    //C-like structures.
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure C from inside the 'enum'.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure Click into 'x' and 'y'.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias, very similar to typedef 
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// we can also use 'Self', on 'impl' blocks
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // 'to_owned()' creates an owned 'String' from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    //we can also 'use' foreach name so they are available
    use crate::VeryVerboseEnumOfThingsToDoWithNumbers::{Add, Subtract};
    let sum = Add;
    let sub = Subtract;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
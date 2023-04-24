use std::{env, process::exit};

#[derive(Debug)]
enum Node {
    Character(char)
}

struct DFA {
    // 1. Store whole DFA as a single object
    // Assume start is at index 0
    items: Vec<Node>
}

impl DFA {
    fn new(regex: &str) -> Self {
        let items = regex.chars().map(|c| {
            match c {
                token if token.is_alphabetic() => {
                    Node::Character(c)
                },
                _ => todo!()
            }
        }).collect();

        println!("{:#?}", items);

        DFA { items }
    }

    fn run(self, needle: &str) -> bool {
        self.items.iter().zip(needle.chars()).all(|(edge, character)| {
            match edge {
                &Node::Character(c) => c == character
            }
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: reginald <needle> <haystack>");
        exit(1);
    }

    let needle = &args[1];
    let haystack = &args[2];

    let dfa = DFA::new(haystack);

    if dfa.run(needle) {
        println!("Success");
    }
    else {
        println!("Failure");
    }
}

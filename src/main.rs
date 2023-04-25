use std::{env, process::exit};

#[derive(Debug, Clone)]
enum Node {
    Character(char),
    Group(Vec<Node>),
    Dot
}

struct DFA {
    // 1. Store whole DFA as a single object
    // Assume start is at index 0
    items: Vec<Node>
}

impl DFA {
    fn new(regex: &str) -> Self {
        let mut items = Vec::new();
        let mut groups = Vec::new();
        for c in regex.chars() {
            match c {
                '(' => groups.push(items.len()),
                ')' => {
                    let start = groups.pop().unwrap();
                    let end = items.len();
                    let group = items[start..end].to_vec();
                    items.push(Node::Group(group))
                },
                '.' => items.push(Node::Dot),
                _ => items.push(Node::Character(c))
            };
        }

        println!("{:#?}", items);

        DFA { items }
    }

    fn run(self, needle: &str) -> bool {
        self.items.iter().zip(needle.chars()).all(|(edge, character)| {
            match edge {
                Node::Character(c) => *c == character,
                Node::Dot => true,
                _ => todo!()
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

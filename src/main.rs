use std::{env, process::exit};

type Matcher<'a> = dyn Fn(&str) -> bool + 'a;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: reginald <needle> <haystack>");
        exit(1);
    }

    let needle = &args[1];
    let haystack = &args[2];

    let matcher = compile(haystack);

    if matcher(needle) {
        println!("Success");
    }
    else {
        println!("Failure");
    }
}

fn compile(haystack: &str) -> Box<Matcher> {
    Box::new(
        move |needle: &str| -> bool { needle == haystack }
    )
}

use ego_tree::iter::Edge::*;
use scraper::{Html, Node, Selector};
use std::fs::File;
use std::io::{self, prelude::*};

struct State {
    code: bool,
    small_code: bool,
}

fn main() {
    let filename = std::env::args().nth(1);

    let mut html = String::new();
    match filename {
        Some(filename) => {
            let mut file = File::open(filename).expect("Can’t open file");
            file.read_to_string(&mut html)
                .expect("Can’t read in the file");
        }
        None => {
            io::stdin()
                .read_to_string(&mut html)
                .expect("Can’t read in stdin");
        }
    }

    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("article").unwrap();

    for article in fragment.select(&selector) {
        let mut state = State {
            code: false,
            small_code: false,
        };

        for element in article.traverse() {
            match element {
                Open(el) => handle_open(&mut state, el.value()),
                Close(el) => handle_close(&mut state, el.value()),
            }
        }
    }
}

fn handle_open(state: &mut State, node: &Node) {
    let el = match node {
        Node::Element(el) => el,
        Node::Text(t) => {
            if state.code {
                print!("{}", t.text.trim());
            } else if state.small_code {
                print!("{}", t.text.trim_end());
            } else if t.text.trim() != "" {
                print!("{}", t.text);
            }
            return;
        }
        n => panic!("Unknown open node: {:?}", n),
    };
    if state.code || state.small_code {
        return;
    }
    match el.name() {
        "article" => (),
        "h2" => print!("## "),
        "p" => (),
        "a" => print!(" ["),
        "span" => print!(" *"),
        "em" => print!(" **"),
        "code" => {
            print!("`");
            state.small_code = true;
        }
        "pre" => {
            println!("```");
            state.code = true;
        }
        "ul" => println!(),
        "li" => print!("- "),
        el => panic!("Unknown open element: {:?}", el),
    }
}

fn handle_close(state: &mut State, node: &Node) {
    let el = match node {
        Node::Element(el) => el,
        Node::Text(_) => return,
        n => panic!("Unknown close node: {:?}", n),
    };
    // this will work only for well formed documents
    if state.code | state.small_code {
        match el.name() {
            "pre" => {
                print!("\n```\n");
                state.code = false;
            }
            "code" if !state.code => {
                print!("`");
                state.small_code = false;
            }
            _ => (),
        }
    } else {
        match el.name() {
            "article" => (),
            "h2" => println!(),
            "p" => print!("\n\n"),
            "a" => print!("]({}) ", el.attr("href").unwrap_or("invalid link")),
            "span" => print!("* "),
            "em" => print!("** "),
            "code" => print!("`"),
            "ul" => println!(),
            "li" => println!(),
            el => panic!("Unknown close element: {:?}", el),
        }
    }
}

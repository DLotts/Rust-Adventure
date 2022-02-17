#[macro_use]
extern crate text_io;
use std::{
    io::{self, Write},
    process::exit,
    sync::Mutex,
};

use lazy_static::lazy_static; // 1.4.0

#[derive(PartialEq, Eq)]
enum Item {
    Light(),
    Axe(),
}

struct Inventory {
    bag: Mutex<Vec<Item>>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            bag: Mutex::new(Vec::<Item>::new()),
        }
    }
    pub fn add(&self, item: Item) {
        self.bag.lock().unwrap().push(item)
    }
    pub fn contains(&self, item: Item) -> bool {
        self.bag.lock().unwrap().contains(&item)
    }
}
lazy_static! {
    static ref INV: Inventory = Inventory::new();
}

fn main() {
    println!("Welcome to Rust Adventure!");

    outside_cave();
}

fn outside_cave() {
    loop {
        println!(
            "To the east and west are a shady path through the woods.
            To the north, a dark gap in the rocks.
            To the south, dense forest."
        );

        let words = get_words();

        match words.iter().next().unwrap_or(&String::from("")).as_str() {
            "north" => cave_allow(),
            "east" => east_path(),
            "south" => dense_forrest(),
            "west" => west_path(),
            _ => default(words),
        }
    }
}

fn default(words: Vec<String>) {
    match &words[..] {
        [w, ..] if w == "north" || w == "east" || w == "south" || w == "west" => {
            println!("Sorry, you cannot go that way.")
        }
        [w, ..] if w == "exit" || w == "quit" => exit(0),
        [verb, obj, ..] if verb == "eat" => println!("Yum yum!  This {} is de-lish.", obj),
        [verb, obj, ..] if verb == "smell" => {
            println!("Strange aroma.  Do you like to smell {}s?", obj)
        }
        _ => println!("I don't know those words.  Say something else.  Try 'help'"),
    }
}
fn cave_allow() {
    println!("You scrape through a narrow gap between boulders.");
    if INV.contains(Item::Light()) {
        cave_entry();
    } else {
        println!(
            "Can't see anything!  Need some kind of light!
    Exiting to the south."
        )
    }
}
fn cave_entry() {
    loop {
        println!(
            "Good thing you brought a flash light AKA torch. 
    Kind of drippy and scarey.  
    I think I see a bear! Run!
    You can run to the exit to the south,
    or a dark passage to the east or west."
        );
        let words = get_words();
        match words.iter().next().unwrap_or(&String::from("")).as_str() {
            "east" => east_cave_path(),
            "south" => outside_cave(),
            "west" => west_cave_path(),
            _ => default(words),
        }
    }
}
fn east_cave_path() {
    println!("Dead end.  Going back.")
}

fn west_cave_path() {
    println!("Dead end.  Going back.")
}

fn east_path() {
    println!(
        "On a shady path.
    Rocks up ahead to the west.
    Is that a house to  the east?"
    );
    //if !INV.lock().unwrap().contains(&Item::Light()) {
    if !INV.contains(Item::Light()) {
        println!("Before you get very far, You find a nice little LED flashlight!");
        //    INV.lock().unwrap().push(Item::Light());
        INV.add(Item::Light());
    } else {
        println!("Too boring, heading back!");
    }
}
fn dense_forrest() {
    println!("Terribly dense forest -- can't get anywhere.")
}
fn west_path() {
    if !INV.contains(Item::Axe()) {
        println!("Oh look, an Axe!");
        INV.add(Item::Axe());
    }
    println!("You get bored and head back toward the cave.");
}

fn get_words() -> Vec<String> {
    loop {
        print!("What do you do? > ");
        io::stdout().flush().unwrap();
        let the_line: String = read!("{}\n");
        let words: Vec<String> = the_line.split_whitespace().map(|s| s.to_string()).collect();
        match &words[..] {
            [verb,..] if verb=="help" => println!("Try east, west, north, or south.  Or maybe a two word sentence: verb noun, like 'eat bread'."),
            _ => {println!("============"); return words}
        }
    }
}

#[macro_use]
extern crate text_io;
use ansi_term::Color;
use ansi_term::Color::Blue;
use ansi_term::Color::Green;
use ansi_term::Color::Purple;
use ansi_term::Color::Yellow;
use ansi_term::Style;
use std::thread::Thread;
use std::{
    io::{self, Write},
    process::exit,
    sync::Mutex,
};

use lazy_static::lazy_static; // 1.4.0
lazy_static! {
    static ref INV: Inventory = Inventory::new();
}

fn main() {
    let enabled = ansi_term::enable_ansi_support();
    if enabled.is_err() {
        println!("Error initializeing colors: {}", enabled.err().unwrap());
        exit(1)
    }
    println!("{}", Color::Red.paint("Welcome to Rust Adventure!"));
    println!(
        "Type {} for {}.  ",
        Style::new().underline().paint("help"),
        Style::new().bold().paint("tips"),
    );

    outside_cave();
}

fn outside_cave() {
    loop {
        println!(
            "To the {} and {} are a shady path through the woods.
            To the {}, a dark gap in the rocks.
            To the {}, dense forest.",
            Yellow.paint("east"),
            Yellow.paint("west"),
            Yellow.paint("north"),
            Yellow.paint("south")
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
    loop {
        println!(
            "What is that?  Hold your light up to the ceiling.
        oh my, bats!  Shhhhh, don't wake them up!
        I hear growling to the west.
        dark passage to the north."
        );
        let words = get_words();
        match words[0].as_str() {
            "north" => home_cave(),
            "west" => cave_entry(),
            _ => default(words),
        }
    }
}
fn home_cave() {
    let example_style = Yellow.underline();
    loop {
        println!(r##"Looks like someone was living here for a while. A dusty bed, a table."##);
        if !INV.contains(Item::HeavyDutyAxe()) {
            println!(
                "Is that something on the table?  Try {}.",
                example_style.paint("look table")
            );
        } else {
            println!(r###"Oh, that's where I found my axe handle.("##); wierd huh?"###);
        }
        println!(
            r#"There is a stinky passage to the east.
            I see black figures hanging from the ceiling(!) to the west."#
        );
        let words = get_words();
        match &words[..] {
            [verb, ..] if verb == "west" => east_cave_path(),
            [verb, ..] if verb == "east" => west_cave_path(),
            [verb, obj, ..] if verb == "look" && obj == "table" => {
                if INV.contains(Item::BrokenAxe()) {
                    println!("You find among the dust and some wood chips a heavy duty east carved axe handle!  
                        Taken.  Looks like it fits your axe nicely.");
                    INV.remove(Item::BrokenAxe());
                    INV.add(Item::HeavyDutyAxe());
                } else {
                    println!("Lot's of wood chips and an carved axe handle.  You don't need those, right?");
                }
            }
            _ => default(words),
        }
    }
}

fn west_cave_path() {
    loop {
        println!(
            "Hmmm, the floor is muddy, and a little sticky, and stinky.
            Stinky? Wait, that's not mud, that's bear poo!
            I hear growling to the east.
            dark passage to the north."
        );
        let words = get_words();
        match words[0].as_str() {
            "north" => home_cave(),
            "east" => cave_entry(),
            _ => default(words),
        }
    }
}

fn east_path() {
    loop {
        println!(
            "On a shady path.
    Rocks up ahead to the west.
    Is that a house to  the east?"
        );
        if !INV.contains(Item::Light()) {
            println!("You found a powerful LED flashlight! Might need this!");
            INV.add(Item::Light());
        } else {
            println!("This is where I found my flashlight.");
        }
        let words = get_words();
        match words[0].as_str() {
            "east" => println!("Aaaa!, there is a cliff from erosion!  Can't go this way."),
            "west" => outside_cave(),
            _ => default(words),
        }
    }
}
fn dense_forrest() {
    loop {
        if !has_axe() {
            println!("Terribly dense forest -- can't get anywhere.")
        } else {
            println!("Thanks to your axe, there is a clearing here.");
            if INV.contains(Item::Axe()) {
                println!("*Crack*  Your rusty axe handle just broke!  Need a new handle to continue in the forest.");
                INV.add(Item::BrokenAxe());
                INV.remove(Item::Axe());
            }
        }
        println!(
            "More forest to the south.
            Rocks and a shady path to the north."
        );
        let words = get_words();
        match words[0].as_str() {
            "south" => {
                if INV.contains(Item::HeavyDutyAxe()) {
                    more_forrest()
                } else {
                    println!("Sorry, you're going to need something to get through these trees.")
                }
            }
            "north" => outside_cave(),
            _ => default(words),
        }
    }
}

fn has_axe() -> bool {
    INV.contains(Item::Axe()) || INV.contains(Item::HeavyDutyAxe())
}
fn more_forrest() {
    loop {
        if has_axe() {
            println!("You use your heavy duty axe and clear a path.  Is that your house up ahead?  It is!  Congratulations, you are home!
            The end!  Credits roll: Rust Adventure by David Lotts.")
        } else {
            println!("You need a new axe or maybe just an axe handle to fix yours.  Then you can continue this way.
            South goes toward the rocks.");
        }
        let words = get_words();
        match words[0].as_str() {
            //"south" => home_is_a_halucination(), //TODO
            "north" => dense_forrest(),
            _ => default(words),
        }
    }
}
fn west_path() {
    loop {
        if !INV.contains(Item::Axe()) && !INV.contains(Item::BrokenAxe()) {
            println!("Oh look, an old rusty axe!  Got it.  Looks like it is about to fall apart.");
            INV.add(Item::Axe());
        }
        println!(
            "Beautiful flowers grow all around a rusty spot where an axe was.
            Dense forrest overtakes the path to the west.
            Path continues to the east."
        );
        let words = get_words();
        match words[0].as_str() {
            "west" => println!("Huge deep puddle is in the way!  You don't want to get your boots wet, do you?  Can't go this way."),
            "east" => outside_cave(),
            _ => default(words),
        }
    }
}

#[derive(PartialEq, Eq, strum_macros::Display)]
enum Item {
    Light(),
    Axe(),
    BrokenAxe(),
    HeavyDutyAxe(),
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
    pub fn remove(&self, rm_item: Item) {
        let mut my_bag = self.bag.lock().unwrap();
        if let Some(index) = my_bag.iter().position(|an_item| *an_item == rm_item) {
            my_bag.remove(index);
        }
    }
    pub fn contains(&self, item: Item) -> bool {
        self.bag.lock().unwrap().contains(&item)
    }
    fn list(&self) -> String {
        self.bag
            .lock()
            .unwrap()
            .iter()
            .map(Item::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn get_words() -> Vec<String> {
    loop {
        print!("{}", Blue.paint("What do you do? > "));
        io::stdout().flush().unwrap();
        let the_line: String = read!("{}\n");
        let words: Vec<String> = the_line.split_whitespace().map(|s| s.to_string()).collect();
        match &words[..] {
            [verb,..] if verb=="help" => println!("Try east, west, north, south, or inventory.  Or maybe a two word sentence: verb noun, like 'eat bread'."),
            [verb,..] if verb=="inventory" => println!("inventory:\n{}",INV.list()),
            _ => {println!("============"); return words}
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

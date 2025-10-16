use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use viuer::{Config, print_from_file};


const TAROT_CARDS: [&str; 78] = [
    "The Fool", 
    "The Magician", 
    "The High Priestess", 
    "The Empress",
    "The Emperor", 
    "The Hierophant", 
    "The Lovers", 
    "The Chariot",
    "Strength", 
    "The Hermit", 
    "Wheel of Fortune", 
    "Justice",
    "The Hanged Man", 
    "Death", 
    "Temperance", 
    "The Devil",
    "The Tower", 
    "The Star", 
    "The Moon", 
    "The Sun",
    "Judgement", 
    "The World",
    "Ace of Wands", 
    "Two of Wands", 
    "Three of Wands", 
    "Four of Wands",
    "Five of Wands", 
    "Six of Wands", 
    "Seven of Wands",
    "Eight of Wands",
    "Nine of Wands", 
    "Ten of Wands", 
    "Page of Wands", 
    "Knight of Wands",
    "Queen of Wands", 
    "King of Wands",
    "Ace of Cups", 
    "Two of Cups", 
    "Three of Cups",
    "Four of Cups",
    "Five of Cups", 
    "Six of Cups", 
    "Seven of Cups", 
    "Eight of Cups",
    "Nine of Cups", 
    "Ten of Cups", 
    "Page of Cups", 
    "Knight of Cups",
    "Queen of Cups", 
    "King of Cups",
    "Ace of Swords", 
    "Two of Swords", 
    "Three of Swords", 
    "Four of Swords",
    "Five of Swords", 
    "Six of Swords", 
    "Seven of Swords", 
    "Eight of Swords",
    "Nine of Swords", 
    "Ten of Swords", 
    "Page of Swords", 
    "Knight of Swords",
    "Queen of Swords", 
    "King of Swords",
    "Ace of Pentacles", 
    "Two of Pentacles", 
    "Three of Pentacles", 
    "Four of Pentacles",
    "Five of Pentacles", 
    "Six of Pentacles", 
    "Seven of Pentacles", 
    "Eight of Pentacles",
    "Nine of Pentacles",
    "Ten of Pentacles", 
    "Page of Pentacles", 
    "Knight of Pentacles",
    "Queen of Pentacles", 
    "King of Pentacles",
]; 

const AGE: [&str; 3] = [
    "Past",
    "Present",
    "Future",
];

struct TarotCard {
    name: String,
    reversed: bool,
    age: String,
}

fn display_tarot(card: &TarotCard) {
    let conf = Config {
        width: Some(25),
        height: Some(20),
        x: 10,
        y: 4,
        ..Default::default()
    };
    let img_name = card.name.to_lowercase().replace(" ", "_");
    let img_path = format!("images/{}.jpg", img_name);
    print!("\x1B[2J\x1B[1;1H");
    print_from_file(&img_path, &conf).expect("Image printing failed.");
}

// fn interpret_patterns(cards){

// }

// right now printing out result for both upright and reversed
fn gather_meaning<P: AsRef<Path>>(tarot_file: P, card: &TarotCard) -> Result<(), Box<dyn Error>> {
    let tarot_file = File::open(tarot_file)?;
    let mut rdr = csv::Reader::from_reader(tarot_file);
    for result in rdr.records() {
        let record = result?;   
        if record.get(0) == Some(&card.name) {
            if let Some(value) = record.get(1) {
                println!("Tarot Meaning: {}", value);
            }
        }  
    }
    Ok(())
}

fn build_card(name: String, reversed: bool, age: String) -> TarotCard {
    TarotCard { name, reversed, age }
}

fn draw_card(drawn_cards: &[usize], age: &str) -> (TarotCard, usize) {
    let mut card_num = rand::rng().random_range(0..78);
    while drawn_cards.contains(&card_num) {
        card_num = rand::rng().random_range(0..78);
    }
    let is_reverse = rand::rng().random_range(0..2);
    let reversed = is_reverse == 1;
    (build_card(TAROT_CARDS[card_num].to_string(), reversed, age.to_string()), card_num)
}

fn main(){
    let tarot_file = "TarotCardsUpright.csv";
    let mut cards: Vec<TarotCard> = Vec::new();
    let mut drawn_cards = [0; 3];
    for i in 0..3 {
        let (card, tarot_index) = draw_card(&drawn_cards, AGE[i]);
        drawn_cards[i] = tarot_index;
        cards.push(card);
    }

    for card in &cards {
        println!("{}: {}{}", card.age, card.name, if card.reversed {" Reversed"} else {""});
        let _ = gather_meaning(tarot_file, card);
        display_tarot(card);
    }
}

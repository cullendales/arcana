use rand::Rng;

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

fn main(){
    println!("The first tarot card is: {}", TAROT_CARDS[0]);
    println!("The last tarot card is: {}", TAROT_CARDS[77]);
    println!("The length of the deck is: {}", TAROT_CARDS.len());

    let mut cards = [0; 3];
    for i in 0..3 {
        let mut card = rand::thread_rng().gen_range(0..78);
        while cards.contains(&card) {
            card = rand::thread_rng().gen_range(0..78);
        }
        cards[i] = card;
    }
    for i in 0..3 {
        println!("Here is the {} card: {}", i+1, TAROT_CARDS[cards[i]]);
    }
}
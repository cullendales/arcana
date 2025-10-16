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

struct TarotCard {
    name: String,
    reversed: bool,
}

// fn interpret_patterns(cards){

// }

// fn gather_meaning(card: TarotCard) -> String {

// } 

// fn display_tarot(card: TarotCard) {

// }

fn build_card(name: String, reversed: bool) -> TarotCard {
    TarotCard { name, reversed }
}

fn draw_card(drawn_cards: &[usize]) -> (TarotCard, usize) {
    let mut card_num = rand::rng().random_range(0..78);
    while drawn_cards.contains(&card_num) {
        card_num = rand::rng().random_range(0..78);
    }
    let is_reverse = rand::rng().random_range(0..2);
    let reversed = is_reverse == 1;
    (build_card(TAROT_CARDS[card_num].to_string(), reversed), card_num)
}

fn main(){
    let mut cards: Vec<TarotCard> = Vec::new();
    let mut drawn_cards = [0; 3];
    for i in 0..3 {
        let (card, tarot_index) = draw_card(&drawn_cards);
        drawn_cards[i] = tarot_index;
        cards.push(card);
    }
    for card in &cards {
        println!("{}{}", card.name, if card.reversed {" Reversed"} else {""});
    }
}

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum CardNumber {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Card(Suit, CardNumber);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}_{}",
            card_shape_to_char(&self.0),
            card_number_to_char(&self.1)
        ))
    }
}
struct CardList(Vec<Card>);

impl Display for CardList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = &self.0;
        let mut output_str = String::from("[");
        for (i, card) in list.iter().enumerate() {
            output_str.push_str(format!("{}", card).as_str());
            if i < list.len() - 1 {
                output_str.push_str(", ");
            }
        }
        output_str.push(']');

        f.write_str(&output_str)
    }
}

fn card_shape_to_char(shape: &Suit) -> char {
    match shape {
        Suit::Clubs => 'C',
        Suit::Diamonds => 'D',
        Suit::Hearts => 'H',
        Suit::Spades => 'S',
    }
}

fn card_number_to_char(number: &CardNumber) -> String {
    let num = card_number_to_number(number);
    let result: String;
    if num == 1 || num > 10 {
        result = match num {
            1 => "A",
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => panic!("Unknow number {}", num),
        }
        .to_string();
    } else {
        result = num.to_string();
    }

    result
}

fn encode_dist(card_list: &mut Vec<Card>, dist: u8) {
    // S: Smallest, M: Middle, L: Largest Card
    // cards are always passed in ascending order SML

    match dist {
        1 => {
            return; // SML
        }
        2 => {
            card_list.swap(1, 2); // SLM
        }
        3 => {
            card_list.swap(0, 1); // MSL
        }
        4 => {
            card_list.swap(0, 1); // MLS
            card_list.swap(1, 2);
        }
        5 => {
            card_list.swap(0, 2); // LMS
        }
        6 => {
            card_list.swap(0, 2); // LSM
            card_list.swap(1, 2);
        }
        _ => {
            panic!("Unknown distance of {}", dist) // Unknown
        }
    }
}

fn move_clockwise(start: u8, step: u8) -> u8 {
    start + step
}

fn card_number_to_number(card_number: &CardNumber) -> u8 {
    match card_number {
        CardNumber::King => 13,
        CardNumber::Queen => 12,
        CardNumber::Jack => 11,
        CardNumber::Ten => 10,
        CardNumber::Nine => 9,
        CardNumber::Eight => 8,
        CardNumber::Seven => 7,
        CardNumber::Six => 6,
        CardNumber::Five => 5,
        CardNumber::Four => 4,
        CardNumber::Three => 3,
        CardNumber::Two => 2,
        CardNumber::Ace => 1,
    }
}

fn you_can_read_minds(cards: &[Card; 5]) {
    // get duplicate suits
    let mut duplicate_suits: Vec<(Card, Card)> = Vec::new();

    for i in 0..cards.len() {
        for j in (i + 1)..cards.len() {
            if cards[i].0 == cards[j].0 {
                duplicate_suits.push((cards[i], cards[j]));
            }
        }
    }

    let mut possible_solutions: Vec<(CardList, Card)> = Vec::new();

    for (mut card1, mut card2) in duplicate_suits.iter_mut() {
        let mut arrangement = Vec::<Card>::with_capacity(4);

        // rearrange to have smaller first and larger last (makes calculation easy)
        if card1.1 > card2.1 {
            (card1, card2) = (card2, card1);
        }

        let (pos1, pos2) = (
            card_number_to_number(&card1.1),
            card_number_to_number(&card2.1),
        );
        let expose: Card;
        let hide: Card;
        let dist: u8;

        // pick card among duplicate pair with clockwise circular distance on number cycle <= 6 to the other member of the pair
        if move_clockwise(pos1, 6) >= pos2 {
            expose = card1;
            hide = card2;
            dist = pos2 - pos1;
        } else {
            expose = card2;
            hide = card1;
            dist = pos1 + 13 - pos2;
        }

        for card in cards {
            if *card != expose && *card != hide {
                arrangement.push(*card);
            }
        }
        arrangement.sort();
        encode_dist(&mut arrangement, dist);
        arrangement.insert(0, expose);

        // println!("{:?} exposing {:?} with distance {}", (card1, card2), expose, dist);
        possible_solutions.push((CardList(arrangement), hide));
    }

    // println!("duplicates {:?}", duplicate_suits);

    // println!("{:?}", cards);

    println!("Show the cards in any one of the following orders:");
    possible_solutions
        .iter()
        .for_each(|x| println!("{}, {}", x.0, x.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_you_can_read_minds() {
        you_can_read_minds(&[
            Card(Suit::Spades, CardNumber::Jack),
            Card(Suit::Clubs, CardNumber::Ace),
            Card(Suit::Clubs, CardNumber::Eight),
            Card(Suit::Clubs, CardNumber::King),
            Card(Suit::Hearts, CardNumber::Five),
        ]);
    }
}

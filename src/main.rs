use rand::seq::SliceRandom;
use rand::RngExt;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Two = 2,
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
    Ace,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", symbol)
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    fn value(&self) -> i32 {
        match self.rank {
            Rank::Ace => 10,
            Rank::Queen => 10,
            Rank::Ten => 5,
            _ => 0,
        }
    }
}

struct Player {
    id: usize,
    hand: Vec<Card>,
    team: usize,
}

impl Player {
    fn show_hand(&self) {
        println!("Player {} (Team {}):", self.id, self.team);
        let mut sorted_hand = self.hand.clone();
        sorted_hand.sort_by(|a, b| {
            if a.suit == b.suit {
                a.rank.cmp(&b.rank)
            } else {
                (a.suit as u8).cmp(&(b.suit as u8))
            }
        });
        
        for card in &sorted_hand {
            print!("{}  ", card);
        }
        println!("\n");
    }
}

struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
    trump: Option<Suit>,
}

impl Game {
    fn new() -> Self {
        let mut deck = Vec::new();

        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in 2..=14 {
                let rank = match rank {
                    2 => Rank::Two,
                    3 => Rank::Three,
                    4 => Rank::Four,
                    5 => Rank::Five,
                    6 => Rank::Six,
                    7 => Rank::Seven,
                    8 => Rank::Eight,
                    9 => Rank::Nine,
                    10 => Rank::Ten,
                    11 => Rank::Jack,
                    12 => Rank::Queen,
                    13 => Rank::King,
                    14 => Rank::Ace,
                    _ => unreachable!(),
                };
                deck.push(Card { suit, rank });
            }
        }

        let players = (0..4)
            .map(|i| Player {
                id: i,
                hand: Vec::new(),
                team: i % 2,
            })
            .collect();

        Game {
            players,
            deck,
            trump: None,
        }
    }

    fn shuffle_and_deal(&mut self) -> Vec<Card> {
        println!("{}", "=".repeat(60));
        println!("SHUFFLING AND DEALING");
        println!("{}", "=".repeat(60));
        
        let mut rng = rand::rng();
        self.deck.shuffle(&mut rng);

        // Deal 12 cards to each player
        for i in 0..4 {
            self.players[i].hand = self.deck[i * 12..(i + 1) * 12].to_vec();
        }

        // Show initial hands
        for i in 0..4 {
            self.players[i].show_hand();
        }

        // Widow (last 4 cards)
        let widow = self.deck[48..52].to_vec();
        println!("Widow (4 cards face down):");
        for _ in 0..4 {
            print!("??  ");
        }
        println!("\n");
        
        widow
    }

    fn bidding(&self) -> usize {
        println!("{}", "=".repeat(60));
        println!("BIDDING PHASE");
        println!("{}", "=".repeat(60));
        
        let mut rng = rand::rng();
        let declarer = rng.random_range(0..4);
        println!("Player {} (Team {}) wins the bid!\n", declarer, self.players[declarer].team);
        declarer
    }

    fn choose_trump(&mut self, player_id: usize) {
        let mut rng = rand::rng();
        let suit = match rng.random_range(0..4) {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            _ => Suit::Spades,
        };
        self.trump = Some(suit);
        println!("Player {} (Team {}) chooses trump: {}\n", 
                 player_id, 
                 self.players[player_id].team, 
                 suit);
    }

    fn play_trick(&mut self, trick_num: usize, leader: usize) -> (usize, i32) {
        println!("{}", "-".repeat(40));
        println!("Trick #{}", trick_num);
        println!("{}", "-".repeat(40));
        println!("Leader: Player {} (Team {})", leader, self.players[leader].team);
        
        let mut played: Vec<(usize, Card)> = Vec::new();
        let mut _lead_suit = None;

        for i in 0..4 {
            let player_index = (leader + i) % 4;
            let card = self.players[player_index].hand.pop().unwrap();
            
            if i == 0 {
                _lead_suit = Some(card.suit);
                println!("\nPlayer {} leads with {}", player_index, card);
            } else {
                println!("Player {} plays {}", player_index, card);
            }
            
            played.push((player_index, card));
        }

        let trump = self.trump.unwrap();

        let winner = played
            .iter()
            .max_by(|(_, a), (_, b)| {
                if a.suit == b.suit {
                    a.rank.cmp(&b.rank)
                } else if a.suit == trump {
                    Ordering::Greater
                } else if b.suit == trump {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap();

        // Calculate points in this trick
        let mut trick_points = 0;
        println!("\nPoints in this trick:");
        for (player_idx, card) in &played {
            let points = card.value();
            if points > 0 {
                println!("  {} from Player {}'s {}", points, player_idx, card);
            }
            trick_points += points;
        }
        
        let total_points = trick_points + 5; // +5 for winning the trick
        
        println!("\nTrick points: {}", trick_points);
        println!("Bonus for winning trick: +5");
        println!("Total trick value: {}", total_points);
        println!("Winner: Player {} (Team {})", winner.0, self.players[winner.0].team);
        
        // Show remaining cards
        println!("\nRemaining cards:");
        for i in 0..4 {
            println!("Player {}: {} cards left", i, self.players[i].hand.len());
        }
        println!();

        (winner.0, total_points)
    }

    fn play_round(&mut self) {
        println!("\n");
        println!("{}", "*".repeat(60));
        println!("NEW GAME STARTING");
        println!("{}", "*".repeat(60));
        println!();
        
        let widow = self.shuffle_and_deal();
        let declarer = self.bidding();

        // Show widow
        println!("Declarer picks up the widow:");
        for card in &widow {
            print!("{}  ", card);
        }
        println!("\n");
        
        // Add widow to declarer
        self.players[declarer].hand.extend(widow);
        
        println!("Declarer's hand after picking up widow:");
        self.players[declarer].show_hand();

        // Remove 4 cards (simplified)
        println!("Declarer discards 4 cards...");
        self.players[declarer].hand.truncate(12);
        println!("After discarding:");
        self.players[declarer].show_hand();

        self.choose_trump(declarer);

        println!("{}", "=".repeat(60));
        println!("PLAYING TRICKS");
        println!("{}", "=".repeat(60));
        println!();

        let mut scores = [0, 0];
        let mut leader = declarer;

        for trick_num in 1..=12 {
            let (winner, pts) = self.play_trick(trick_num, leader);
            scores[self.players[winner].team] += pts;
            leader = winner;
            
            println!("Current scores: Team 0: {}, Team 1: {}\n", scores[0], scores[1]);
        }

        println!("{}", "=".repeat(60));
        println!("GAME OVER");
        println!("{}", "=".repeat(60));
        println!("Final scores:");
        println!("Team 0: {}", scores[0]);
        println!("Team 1: {}", scores[1]);
        
        if scores[0] > scores[1] {
            println!("Team 0 wins by {} points!", scores[0] - scores[1]);
        } else if scores[1] > scores[0] {
            println!("Team 1 wins by {} points!", scores[1] - scores[0]);
        } else {
            println!("It's a tie!");
        }
        println!();
    }
}

fn main() {
    let mut game = Game::new();
    game.play_round();
}
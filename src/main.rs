use rand::{rngs::ThreadRng, seq::IndexedRandom, Rng};

/// All possible player choices in a rock paper scissors game
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RpsChoice {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

/// All possible players in this scenario
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RpsPlayer {
    Player1 = 0,
    Player2 = 1,
    Player3 = 2,
    Player4 = 3,
    Player5 = 4
}

/// Winner of a rock paper scissors game
#[derive(Debug)]
enum RpsWinner {
    Player(RpsPlayer),
    Draw
}

/// Record of all matches for a player
#[derive(Debug)]
struct RpsPlayerRecord {
    wins: i32,
    losses: i32,
    draws: i32,
    winning_choices: Vec<RpsChoice>,
    losing_choices: Vec<RpsChoice>,
    choices: Vec<RpsChoice>
}

impl RpsPlayerRecord {
    /// Creates a blank record
    pub fn new() -> Self {
        RpsPlayerRecord { 
            wins: 0, 
            losses: 0, 
            draws: 0,
            winning_choices: Vec::new(), 
            losing_choices: Vec::new(),
            choices: Vec::new()
        }
    }

    /// Records a win
    pub fn win(&mut self, choice: RpsChoice) {
        self.wins += 1;
        self.winning_choices.push(choice);
        self.choices.push(choice);
    }

    /// Records a loss
    pub fn lose(&mut self, choice: RpsChoice) {
        self.losses += 1;
        self.losing_choices.push(choice);
        self.choices.push(choice);
    }

    /// Record a draw
    pub fn draw(&mut self) {
        self.draws += 1;
    }

    /// Returns the most recent choice or None if no choices
    pub fn most_recent_choice(&self) -> Option<RpsChoice> {
        self.choices.last().cloned()
    }

    /// Returns the most recent successful move or None if there are no moves
    pub fn most_recent_win(&self) -> Option<RpsChoice> {
        return self.winning_choices.last().cloned()
    }

    /// Returns the most common choice that this player lost to or None if no losses
    pub fn most_common_loss(&self) -> Option<RpsChoice> {
        if self.losing_choices.len() == 0 {
            return None
        }

        let most_recent_x = 100;

        let last_100;
        if self.losing_choices.len() < most_recent_x {
            last_100 = self.losing_choices.as_slice();
        } else {
            last_100 = &self.losing_choices[&self.losing_choices.len() - most_recent_x..];
        }

        let mut counts = vec!(0, 0, 0);
        for choice in last_100 {
            counts[choice.clone() as usize] += 1;
        }

        let max = counts.iter().max().unwrap().clone();

        if max == counts[0] {
            return Some(RpsChoice::Rock);
        }
        if max == counts[1] {
            return Some(RpsChoice::Paper);
        }
        Some(RpsChoice::Scissors)
    }

    fn print_list_stats(l: &Vec<RpsChoice>, preamble: String) {
        println!("{preamble} [Rock {}x, Paper {}x, Scissors {}x]", 
            l.iter().filter(|&&x| x == RpsChoice::Rock).count(),
            l.iter().filter(|&&x| x == RpsChoice::Paper).count(),
            l.iter().filter(|&&x| x == RpsChoice::Scissors).count());
    }

    /// Returns the win loss ratio for this record
    pub fn ratio(&self) -> f32 {
        self.wins as f32 / self.losses as f32
    }

    /// Prints record to stdout in a decent way
    pub fn print(&self) {
        println!("Wins/Losses/Draws: {}/{}/{}, W/L = {:.2}", self.wins, self.losses, self.draws, self.ratio());
        RpsPlayerRecord::print_list_stats(&self.winning_choices, "Wins".to_string());
        RpsPlayerRecord::print_list_stats(&self.losing_choices, "Losses".to_string());
        RpsPlayerRecord::print_list_stats(&self.choices, "Total Plays".to_string());
    }
}

/// A single rock paper scissors match
#[derive(Debug)]
struct RpsMatch {
    player_1: RpsPlayer,
    player_2: RpsPlayer,
    player_1_choice: RpsChoice,
    player_2_choice: RpsChoice
}

impl RpsMatch {
    /// Creates a new RpsMatch
    pub fn new(player_1: RpsPlayer, player_2: RpsPlayer, player_1_choice: RpsChoice, player_2_choice: RpsChoice) -> Self {
        RpsMatch {
            player_1,
            player_2,
            player_1_choice,
            player_2_choice
        }
    }

    /// Computes the winner of a rock paper scissors match
    pub fn winner(&self) -> RpsWinner {
        match self.player_1_choice {
            RpsChoice::Rock => {
                match self.player_2_choice {
                    RpsChoice::Rock => RpsWinner::Draw,
                    RpsChoice::Paper => RpsWinner::Player(self.player_2),
                    RpsChoice::Scissors => RpsWinner::Player(self.player_1)
                }
            },
            RpsChoice::Paper => {
                match self.player_2_choice {
                    RpsChoice::Rock => RpsWinner::Player(self.player_1),
                    RpsChoice::Paper => RpsWinner::Draw,
                    RpsChoice::Scissors => RpsWinner::Player(self.player_2)
                }
            },
            RpsChoice::Scissors => {
                match self.player_2_choice {
                    RpsChoice::Rock => RpsWinner::Player(self.player_2),
                    RpsChoice::Paper => RpsWinner::Player(self.player_1),
                    RpsChoice::Scissors => RpsWinner::Draw
                }
            }
        }
    }

    /// Prints the match and the results to stdout
    #[allow(dead_code)]
    pub fn result(&self) {
        println!("Match: {:?} vs {:?} => {:?}", self.player_1, self.player_2, self.winner());
    }
}

/// Returns a choice depending on a lot of things
fn strategy(rng: &mut ThreadRng, player: RpsPlayer, player_record: &RpsPlayerRecord, _opponent: RpsPlayer, opponent_record: &RpsPlayerRecord) -> RpsChoice {
    let choices = vec!(
        RpsChoice::Rock,
        RpsChoice::Paper,
        RpsChoice::Scissors
    );
    let random_choice = choices.choose(rng).unwrap().clone();

    match player {
        // player 1 is completely random
        RpsPlayer::Player1 => {
            random_choice
        },
        // player 2 is heavily weighted to scissors but still "random"
        RpsPlayer::Player2 => {
            let weight = rng.random_range(0..10);
            let choice;
            if weight < 6 {
                choice = RpsChoice::Scissors;
            } else if weight < 8 {
                choice = RpsChoice::Rock;
            } else {
                choice = RpsChoice::Paper;
            }
            choice
        },
        // player 3 will choose the most common move in their opponents losing record
        RpsPlayer::Player3 => {
            match opponent_record.most_common_loss() {
                Some(choice) => choice,
                None => random_choice
            }
        },
        // player 4 will cycle going RPS ad nauseum
        RpsPlayer::Player4 => {
            match player_record.most_recent_choice().unwrap_or(random_choice) {
                RpsChoice::Paper => RpsChoice::Scissors,
                RpsChoice::Rock => RpsChoice::Paper,
                RpsChoice::Scissors => RpsChoice::Rock,
            }
        },
        // player 5 will copy their opponents most recent successful move, or random if there are no recorded moves yet
        RpsPlayer::Player5 => {
            match opponent_record.most_recent_win() {
                Some(choice) => choice,
                None => random_choice
            }
        }
    }
}

fn pop_random_element(rng: &mut ThreadRng, list: &mut Vec<RpsPlayer>) -> RpsPlayer {
    let idx = rng.random_range(0..list.len());
    list.remove(idx)
}

fn main() {
    let mut matches = Vec::new();
    let mut rng = rand::rng();
    let mut player_records = vec!(
        RpsPlayerRecord::new(),
        RpsPlayerRecord::new(),
        RpsPlayerRecord::new(),
        RpsPlayerRecord::new(),
        RpsPlayerRecord::new()
    );

    let match_count = 1000;

    // Compile random matches
    let start_time = std::time::Instant::now();
    for _ in 0..match_count {
        // Pick two random players
        let mut players = vec!(
            RpsPlayer::Player1,
            RpsPlayer::Player2,
            RpsPlayer::Player3,
            RpsPlayer::Player4,
            RpsPlayer::Player5
        );
        let player_1 = pop_random_element(&mut rng, &mut players);
        let player_2 = pop_random_element(&mut rng, &mut players);

        // Pick each player choice
        let choice_1 = strategy(&mut rng, player_1, &player_records[player_1 as usize], player_2, &player_records[player_2 as usize]);
        let choice_2 = strategy(&mut rng, player_2, &player_records[player_2 as usize], player_1, &player_records[player_1 as usize]);

        // Create a match where the 2 random players picked two random moves
        let rps_match = RpsMatch::new(
            player_1, 
            player_2, 
            choice_1,
            choice_2,
        );

        // Record wins and losses
        match rps_match.winner() {
            RpsWinner::Player(p) if p == player_1 => {
                player_records[player_1 as usize].win(choice_1);
                player_records[player_2 as usize].lose(choice_2);
            },
            RpsWinner::Player(p) if p == player_2 => {
                player_records[player_2 as usize].win(choice_2);
                player_records[player_1 as usize].lose(choice_1);
            },
            _ => {
                player_records[player_1 as usize].draw();
                player_records[player_2 as usize].draw();
            },
        }

        matches.push(rps_match);
    }
    let end_time = std::time::Instant::now();
    let between_time = end_time.duration_since(start_time).as_micros();

    println!("Time to create {} records: {:.3}ms", match_count, between_time as f64 / 1000.0);

    let player_strats = vec!(
        "Random Ramsy is completely random",
        "Scissor Sally is heavily weighted to scissors and otherwise random",
        "Loser Larry will choose the most common move in their opponents losing record",
        "Groovy Garth will cycle going rock-paper-scissors ad nauseum",
        "Copycat Candice will copy their opponents most recent successful move"
    );

    println!("===========================================================================");
    for i in 0..5 {
        println!("{}:", player_strats[i]);
        player_records[i].print();
        println!("===========================================================================");
    }

    // Print just the W/L to make it easier to find
    println!("Random Ramsy W/L: {:.2}\n\
            Scissor Sally W/L: {:.2}\n\
            Loser Larry W/L: {:.2}\n\
            Groovy Garth W/L: {:.2}\n\
            Copycat Candice W/L: {:.2}",
            player_records[0].ratio(), 
            player_records[1].ratio(), 
            player_records[2].ratio(), 
            player_records[3].ratio(), 
            player_records[4].ratio());
}

/*

Write a  program that simulates the "Rock, Paper, Scissors" game.

The game should ask the user to enter an option (either "Rock", "Paper", or "Scissors").

The player should play against the computer, which will select a random option.

The computer's selection will be compared against the player's selection to determine who wins.

A descriptive message should be displayed indicating if the player won, lost, or if the game ended in a tie.

Basic Game Rules:

    Paper beats Rock

    Rock beats Scissors

    Scissors beat Paper.

 Expected Output:
Sample Game 1:

====== Welcome to the game ======
Please enter Rock, Paper, or Scissors below:
Rock
It's a tie! Try again.
Sample Game 2:

====== Welcome to the game ======
Please enter Rock, Paper, or Scissors below:
Paper
You lose! Your opponent chose 'Scissors'
Sample Game 3:

====== Welcome to the game ======
Please enter Rock, Paper, or Scissors below:
Rock
You win! Your opponent chose 'Scissors'
*/

use std::io;

use rand::{random, Rng};

#[derive(Clone,Debug)]
pub enum PlayAct {
    Rock,
    Paper,
    Scissors,
    Default
}




pub fn play_rps() {
    let mut rng = rand::thread_rng();
    let random_number: i8 = rng.gen_range(1..3);
    let computer_act: PlayAct = get_act(random_number);

    println!("====== Welcome to the game ======");
    println!("Please enter Rock, Paper, or Scissors below:");
    
    let user_guess = take_input();
    if is_player_win(computer_act.clone(), user_guess) {
        println!("You win! You opponent chose '{:?}'",computer_act);
    } else {
        println!("You lose~ You opponent chose '{:?}'",computer_act);
    }
    

}

pub fn is_player_win(computer_act: PlayAct, user_act: PlayAct) -> bool {
    let mut result = false;
    match (computer_act, user_act) {
        (PlayAct::Rock, PlayAct::Scissors) => result = false,
        (PlayAct::Rock, PlayAct::Paper) => result = true,
        (PlayAct::Paper, PlayAct::Rock) => result = false,
        (PlayAct::Paper, PlayAct::Scissors) => result = true,
        (PlayAct::Scissors, PlayAct::Paper) => result = false,
        (PlayAct::Scissors, PlayAct::Rock) => result = true,
        (_,_) => result = false,
    }
    return result;
}

pub fn take_input() -> PlayAct {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Invalid data pleass try again");
    }
    let trim_data = buffer.trim();
    if trim_data.is_empty() {
        println!("invalid data to play");
    }

    if trim_data == "Rock" {
        return PlayAct::Rock;
    } else if trim_data == "Paper" {
        return PlayAct::Paper;
    } else {
        return  PlayAct::Scissors;
    }
    
    
}

fn get_act(number: i8) -> PlayAct {
    match number {
        1 => return PlayAct::Rock,
        2 => return PlayAct::Paper,
        3 => return PlayAct::Scissors,
        _ => return PlayAct::Default,
    }
}
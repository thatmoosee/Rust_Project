mod player;
mod file_reader;
use crate::file_reader::read_file;
use crate::player::search_player;
use crate::player::sort_by_rbis;
use crate::player::sort_by_homeruns;
use crate::player::sort_by_batavg;
use std::io::{self, Write};

fn main() {
    let mut players = read_file();
    loop {
        //menu
        println!("\n1. Search for a player");
        println!("2. Sort by RBIs");
        println!("3. Sort by Home Runs");
        println!("4. Sort by Batting Average");
        println!("5. Exit");
        println!("Enter input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: u8 = input.trim().parse().unwrap();
        match input {
            //switch statement for the menu
            1 => {
                //getting the player's name you want to retrieve
                print!("Enter name of the player: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                //A match statement to make finding the player a lot easier
                match search_player(&players, name){
                    Some(player) => player.display_info(),
                    _ => println!("There is no player with that name."),
                }
            }
            2 => {
                //returning all the players in descending order of rbis
                sort_by_rbis(&mut players);
                for player in &players{
                    println!("{} has {} RBIs", player.get_name(), player.get_rbis());
                }
            }
            3 => {
                //returning all the players in descending order of homeruns
                sort_by_homeruns(&mut players);
                for player in &players {
                    println!("{} has {} Homeruns", player.get_name(), player.get_rbis());
                }
            }
            4 => {
                //returning all the players in descending order of batting average
                sort_by_batavg(&mut players);
                for player in &players{
                    println!("{} has {} Batting Average", player.get_name(), player.get_batavg());
                }
            }
            //to quit
            5 => break,
            //any invalid u8
            _ => continue,
        }
    }
}
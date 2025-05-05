use std::fs::File;
use std::io::Read;
use crate::player::Player;

pub fn read_file() -> Vec<Player> {
    //to read the file and to turn into a string
    let mut file = File::open("players.csv").expect("Can't open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file!");
    //to return all the players at the end
    let mut players = Vec::new();
    //each line has name, team, position, rbis, homeruns, and batting average
    //so we split it by commas because it is a .csv
    for line in contents.lines(){
        let categories: Vec<&str> = line.split(',').collect();
        if categories.len() == 6 {
            let name = categories[0].to_string();
            let team = categories[1].to_string();
            let position = categories[2].to_string();
            let rbis = categories[3].parse::<u16>().unwrap();
            let homeruns = categories[4].parse::<u16>().unwrap();
            let batavg = categories[5].parse::<f64>().unwrap();
            //creating an object to insert all of the information
            let player = Player::new(name, team, position, batavg, rbis, homeruns);
            //adding it to the vector players
            players.push(player);
        }
    }
    players
}
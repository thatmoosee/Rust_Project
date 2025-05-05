pub struct Player {
    name: String,
    team: String,
    position: String,
    batavg: f64,
    rbis: u16,
    homeruns: u16,
}

impl Player {
    
    pub fn new(name: String, team: String, position: String, batavg: f64, rbis: u16, homeruns: u16) -> Player{
        Player {name, team, position, batavg, rbis, homeruns}
    }
    //for search player to display all the players stats
    pub fn display_info(&self) {
        println!("\nName: {}", self.get_name());
        println!("Team: {}", self.get_team());
        println!("Position: {}", self.get_position());
        println!("Batting Average: {}", self.get_batavg());
        println!("RBIs: {}", self.get_rbis());
        println!("Home Runs: {}", self.get_homeruns());
    }
    //getter for name
    pub fn get_name(&self) -> &str {
        &self.name
    }
    //getter for team
    pub fn get_team(&self) -> &str {
        &self.team
    }
    //getter for position
    pub fn get_position(&self) -> &str {
        &self.position
    }
    //getter for number of rbis
    pub fn get_rbis(&self) -> u16 {
        self.rbis
    }
    //getter for number of homeruns
    pub fn get_homeruns(&self) -> u16 {
        self.homeruns
    }
    //getter for batting average
    pub fn get_batavg(&self) -> f64 {
        self.batavg
    }
}
//a way to find the certain player in the csv file and returns it
pub fn search_player<'a>(players: &'a [Player], name: &'a str) -> Option<&'a Player> {
    players.iter().find(|p| p.name.eq_ignore_ascii_case(name))
}
//sorting all players by rbis by descending order
pub fn sort_by_rbis(players: &mut Vec<Player>){
    players.sort_by(|player1,player2| player2.get_rbis().cmp(&player1.get_rbis()));
}
//sorting all players by homeruns by descending order
pub fn sort_by_homeruns(players: &mut Vec<Player>){
    players.sort_by(|player1,player2| player2.get_homeruns().cmp(&player1.get_homeruns()));
}
//sorting all players by batavg by descending order
pub fn sort_by_batavg(players: &mut Vec<Player>){
    players.sort_by_key(|p| std::cmp::Reverse(p.get_batavg().to_bits()));
}
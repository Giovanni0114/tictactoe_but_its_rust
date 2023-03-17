use std::i8;

enum Sign {
    X, 
    O,
}
impl Sign {
    fn as_str(&self) -> String {
        match self {
            Sign::X => "X".to_string(),
            Sign::O => "O".to_string()
        }
    }
}

struct Player{
    id: String,
    sign: Sign,
    is_human: bool,
}


impl Player {
    fn get_name(&self) -> String{
        self.id.to_string()
    }

    fn get_sign(&self) -> String{
        self.sign.as_str()
    }
}

fn player_swap(cur: &mut Player, p1: &Player, p2: &Player){
    if (*cur).get_name() == (*p1).get_name(){
        cur = p2; 
    }
    else {
        cur = p2; 
    }
}


fn main() {
    println!("Hello! Wanna play a game?");
    println!("--------------------------");

    let p1 = Player {
        id: "human1".to_string(), 
        sign: Sign::X, 
        is_human: true 
    };
    let p2 = Player {
        id: "human2".to_string(), 
        sign: Sign::O, 
        is_human: true 
    };


    let mut current_player: &mut Player;
    let mut moves_left: u8 = 9;


    if rand::random(){
        current_player = p1;
    }
    else {
        current_player = &p2;
    }


    while moves_left > 0 {
        println!("Round: {}", 10-moves_left);
        println!("Current player: {}", (*current_player).get_name());
        
        moves_left -= 1;
        player_swap(&mut current_player, &p1, &p2)  ;
    }
    
}

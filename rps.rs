This a rock paper scissors code 
//my_code
fn rps(p1: &str, p2: &str) -> &'static str  {
    let choice = String::from("rock");
    let choice_2 =String::from("paper");
    let choice_3 = String::from("scissors");
    let result_1:&str = "Player 1 won!";
    let result_2 :&str ="Player 2 won!";
    
    if p1 == p2{
        return "Draw!";
    }
    if p1 == choice && p2 == choice_3{
        return result_1;
    }
    else if p1 == choice_2 && p2 == choice{
        return result_1;
    }
    else if p1 == choice_3 && p2 == choice_2{
        return result_1;
    }
    else{ 
        return result_2; 
    }
}
vs 
//best_practise_1
fn rps(p1: &str, p2: &str) -> &'static str {
    if (p1 == p2) {
        return "Draw!";
    }
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        _ => "Player 2 won!",
    }
}
vs 
//best_practise_2
fn rps(p1: &str, p2: &str) -> &'static str  {
    match (p1, p2) {
        ("scissors", "paper")|("paper", "rock")|("rock", "scissors") => "Player 1 won!",
        ("paper", "scissors")|("scissors", "rock")|("rock", "paper") => "Player 2 won!",
        _ => "Draw!"
    }
}


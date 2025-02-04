use std::collections::HashMap;

fn main (){
    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Red"),30);

    let team_name=String::from("Blue");
    match scores.get(&team_name){
        Some(score)=>println!("Score :{}",score),
        None =>println!("No score found"),
    };
}
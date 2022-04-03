mod pokemon;
use crate::pokemon::Pokemon;

fn main() {
    let mut charizard = Pokemon::new(5, 100);
    let mut pikachu = Pokemon::new(2,100);
    println!("life: {}", charizard.get_life());
    println!("life: {}", pikachu.get_life());
    charizard.attack(&mut pikachu);
    charizard.attack(&mut pikachu);
    charizard.attack(&mut pikachu);
    pikachu.attack(&mut charizard);
    pikachu.attack(&mut charizard);
    pikachu.attack(&mut charizard);
    println!("life: {}", charizard.get_life());
    println!("life: {}", pikachu.get_life());
}

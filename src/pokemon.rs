#[derive(Copy, Clone)]
pub(crate) struct Pokemon {
    attack:i32,
    life:i32
}

impl Pokemon {
    pub fn new (attack:i32, life:i32) -> Pokemon {
        Pokemon{attack,life}
    }
    pub fn attack (&self, pokemon:&mut Pokemon) {
        pokemon.receive_attack(self.attack)
    }

    pub fn receive_attack(&mut self, attack:i32){
        self.life = self.life - attack;
    }

    pub fn get_life(&self) -> i32 {
        self.life
    }
}
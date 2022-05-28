use std::sync::RwLockWriteGuard;
use rand::Rng;
use crate::{Resource, Wood};

pub struct Gold {
    amount: f64
}
impl Gold{

    pub fn new(amount: f64) -> Self
    {
        Self {amount}
    }

    pub fn getAmount(&self) -> f64
    {
        return self.amount;
    }

    pub fn generate(&mut self)
    {
        let percentage :f64 = rand::thread_rng().gen();
        self.amount = self.amount + (percentage * self.amount);
    }

    pub fn exchangeWood(&mut self, mut wood: RwLockWriteGuard<Wood>)
    {
        let woodPrice = wood.getPrice();
        if self.amount > woodPrice {
            self.amount = self.amount - woodPrice;
            wood.add(1);
        }
    }
}
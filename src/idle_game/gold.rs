use std::sync::RwLockWriteGuard;
use rand::Rng;
use crate::{Resource, Wood};

pub struct Gold {
    amount: f64
}
impl Gold
{
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
        let amount_generate:f64 = rand::thread_rng().gen();
        self.amount = self.amount + (amount_generate * 100.0).round();
    }

    pub fn exchange(&mut self, mut resource: RwLockWriteGuard<Resource>)
    {
        let resource_price = resource.getPrice();
        if self.amount > resource_price {
            self.amount = self.amount - resource_price;
            resource.add(1);
        }
    }
}
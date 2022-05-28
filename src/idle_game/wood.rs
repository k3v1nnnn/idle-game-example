use crate::Resource;

pub struct Wood {
    amount: u32,
    price: f64
}

impl Wood
{

    pub fn new(amount: u32, price: f64) -> Self
    {
        Self {amount, price }
    }
}

impl Resource for Wood
{

    fn add(&mut self, wood: u32)
    {
        self.amount = self.amount + wood;
    }

    fn getPrice(&self) -> f64
    {
        return self.price;
    }

    fn getAmount(&self) -> u32
    {
        return self.amount;
    }
}
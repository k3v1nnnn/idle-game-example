pub struct Water {
    amount: u32,
    price: f64
}

impl Water
{
    pub fn new(amount: u32, price: f64) -> Self
    {
        Self {amount, price }
    }

    pub fn add(&mut self, water: u32)
    {
        self.amount = self.amount + water;
    }

    pub fn getPrice(&self) -> f64
    {
        return self.price;
    }

    pub fn getAmount(&self) -> u32
    {
        return self.amount;
    }
}
pub struct Resource {
    amount: u32,
    price: f64
}

impl Resource
{
    pub fn new(amount: u32, price: f64) -> Self
    {
        Self {amount, price }
    }

    pub fn add(&mut self, resource: u32)
    {
        self.amount = self.amount + resource;
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
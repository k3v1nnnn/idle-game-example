pub struct Wood {
    amount: u32,
    price: f64
}
impl Wood{

    pub fn new(amount: u32, price: f64) -> Self
    {
        Self {amount, price }
    }

    pub fn addWood(&mut self, wood:u32)
    {
        self.amount = self.amount + wood;
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
pub trait Resource
{
    fn add(&mut self, wood:u32);
    fn getPrice(&self) -> f64;
    fn getAmount(&self) -> u32;
}
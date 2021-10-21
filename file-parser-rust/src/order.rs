use std::fmt;

#[derive(Clone, Debug)]
pub struct Order {
    pub id: String,
    pub bank_fund: f32,
    pub bank_refund: f32,
    pub sass_fund: f32,
}

impl Order {
    pub fn picked(&self) -> bool {
        self.sass_fund == self.bank_fund && self.bank_refund == 0.0
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Order({}, {}, {}, {})",
            self.id, self.bank_fund, self.bank_refund, self.sass_fund
        )
    }
}

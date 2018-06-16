pub trait Order {
    fn execute(&self);
}

pub struct Stock {
    name: String,
    quantity: isize,
}

impl Stock {
    pub fn new() -> Self {
        Stock { name: "ABC".to_string(), quantity: 10 }
    }
    fn buy(&self) {
        println!("Stock [ Name: {}, Quantity: {} ] bought", self.name, self.quantity);
    }
    fn sell(&self) {
        println!("Stock [ Name: {}, Quantity: {} ] sold", self.name, self.quantity);
    }
}

pub struct BuyStock {
    abc_stock: Stock,
}

impl BuyStock {
    pub fn new(abc_stock: Stock) -> Self {
        BuyStock { abc_stock }
    }
}

impl Order for BuyStock {
    fn execute(&self) {
        self.abc_stock.buy();
    }
}

pub struct SellStock {
    abc_stock: Stock,
}

impl SellStock {
    pub fn new(abc_stock: Stock) -> Self {
        SellStock { abc_stock }
    }
}

impl Order for SellStock {
    fn execute(&self) {
        self.abc_stock.sell();
    }
}

pub struct Broker {
    orders: Vec<Box<Order>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker { orders: vec![] }
    }
    pub fn take_order(&mut self, order: Box<Order>) {
        self.orders.push(order);
    }
    pub fn place_orders(&mut self) {
        for order in &self.orders {
            order.execute();
        }

        self.orders.clear();
    }
}

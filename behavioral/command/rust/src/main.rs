extern crate command;

use command::{Stock, BuyStock, SellStock, Order, Broker};

fn main() {
    let abc_stock1 = Stock::new();
    let abc_stock2 = Stock::new();

    let buy_stock_order = BuyStock::new(abc_stock1);
    let sell_stock_order = SellStock::new(abc_stock2);

    let mut broker = Broker::new();
    broker.take_order(Box::new(buy_stock_order));
    broker.take_order(Box::new(sell_stock_order));

    broker.place_orders();
}

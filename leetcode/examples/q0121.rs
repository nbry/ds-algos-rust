pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy_price = &prices[0];
    let mut max_profit = 0;

    for price in prices.iter() {
        if price < buy_price {
            buy_price = price
        } else {
            let profit = price - buy_price;

            if profit > max_profit {
                max_profit = profit
            }
        }
    }

    max_profit
}

fn main() {}

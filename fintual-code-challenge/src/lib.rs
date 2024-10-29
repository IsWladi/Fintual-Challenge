pub mod stocks_api;
use stocks_api::StockAPI;

#[allow(dead_code)]
struct Stock {
    company_name: String,
    date: String,
    amount: i64,
}

#[allow(dead_code)]
struct Portfolio {
    stocks: Box<[Stock]>,
}

#[allow(dead_code)]
impl Portfolio {
    pub fn profit(&self, initial_date: &str, final_date: &str) -> f64 {
        // Create the connection to the API
        let stock_api = StockAPI::new();

        let mut total_profit = 0.0;

        for stock in self.stocks.iter() {
            let initial_price = stock_api
                .get_stock_price(&stock.company_name, initial_date)
                .unwrap();
            let final_price = stock_api
                .get_stock_price(&stock.company_name, final_date)
                .unwrap();

            // Add the profit of each stock
            total_profit += (final_price - initial_price) * stock.amount as f64;
        }
        total_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_profit_of_portfolio_fintual() {
        let company_name_stocks = "fintual".to_string();
        let stock = Stock {
            company_name: company_name_stocks.clone(),
            date: "2024-01-01".to_string(),
            amount: 10,
        };

        let portfolio = Portfolio {
            stocks: Box::new([stock]),
        };

        let profit = portfolio.profit("2024-01-01", "2024-01-03");
        assert_eq!(profit, 20.0);
    }

    #[test]
    fn get_profit_of_portfolio_fintual_and_other() {
        let company_name_stocks = "fintual".to_string();
        let fintual_stock = Stock {
            company_name: company_name_stocks.clone(),
            date: "2024-01-02".to_string(),
            amount: 15,
        };

        // Create the "Other" stock
        let company_name_stocks = "other".to_string();
        let other_stock = Stock {
            company_name: company_name_stocks.clone(),
            date: "2024-01-02".to_string(),
            amount: 5,
        };

        let portfolio = Portfolio {
            stocks: Box::new([fintual_stock, other_stock]),
        };

        let profit = portfolio.profit("2024-01-02", "2024-01-03");
        assert_eq!(profit, 515.0);
    }
}

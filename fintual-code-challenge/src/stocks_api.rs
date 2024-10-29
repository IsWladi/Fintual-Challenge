use std::collections::HashMap;

struct Company {
    name: String,
    stocks: HashMap<String, f64>,
}

impl Company {
    pub fn price(&self, date: &str) -> Option<f64> {
        self.stocks.get(date).copied()
    }
}

pub struct StockAPI {
    companies: Box<[Company]>,
}

impl StockAPI {
    pub fn new() -> StockAPI {
        // Create the fintual company with its stocks
        let mut fintual_stocks = HashMap::new();

        fintual_stocks.insert("2024-01-01".to_string(), 100.0);
        fintual_stocks.insert("2024-01-02".to_string(), 101.0);
        fintual_stocks.insert("2024-01-03".to_string(), 102.0);

        let fintual = Company {
            name: "fintual".to_string(),
            stocks: fintual_stocks,
        };

        // Create the "Other" company with its stocks
        let mut other_stocks = HashMap::new();
        other_stocks.insert("2024-01-01".to_string(), 200.0);
        other_stocks.insert("2024-01-02".to_string(), 300.0);
        other_stocks.insert("2024-01-03".to_string(), 400.0);

        let other = Company {
            name: "other".to_string(),
            stocks: other_stocks,
        };

        StockAPI {
            companies: Box::new([fintual, other]),
        }
    }

    pub fn get_stock_price(&self, company_name: &str, date: &str) -> Option<f64> {
        self.companies
            .iter()
            .find(|company| company.name == company_name)
            .and_then(|company| company.price(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_price_of_company_stock() {
        let stock_api = StockAPI::new();

        let company_name = "fintual";

        assert_eq!(
            stock_api.get_stock_price(company_name, "2024-01-01"),
            Some(100.0)
        );

        assert_eq!(
            stock_api.get_stock_price(company_name, "2024-01-02"),
            Some(101.0)
        );

        assert_eq!(
            stock_api.get_stock_price(company_name, "2024-01-03"),
            Some(102.0)
        );
    }
}

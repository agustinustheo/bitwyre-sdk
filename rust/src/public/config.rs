pub fn timeout() -> u64 {
    return 5;
}
pub fn url_api_bitwyre<'a>() -> &'a str {
    return "https://api.bitwyre.com";
}
pub fn get_public_api_endpoint(key: &str) -> &str {
    match key {
        "SERVERTIME" => "/public/time",
        "TRADES" => "/public/trades",
        "TICKER" => "/public/ticker",
        "INSTRUMENT" => "/public/pairs",
        "DEPTH" => "/public/depth",
        "ANNOUNCEMENTS" => "/public/announcements",
        "PRODUCTS" => "/public/products",
        "MARKETS" => "/public/markets",
        "ASSETS" => "/public/assets",
        _ => ""
    }
}
pub fn get_private_api_endpoint(key: &str) -> &str {
    match key {
        "ACCOUNT_BALANCE" => "/private/account/spotbalance",
        "ACCOUNT_STATEMENT" => "/private/account/statement",
        "TRANSACTION_HISTORIES" => "/private/account/transactions",
        "ORDER" => "/private/orders",
        "CANCEL_ORDER" => "/private/orders/cancel",
        "OPEN_ORDERS" => "/private/orders/open",
        "CLOSED_ORDERS" => "/private/orders/closed",
        "ORDER_INFO" => "/private/orders/info",
        "TRADE_HISTORY" => "/private/trades",
        _ => ""
    }
}
pub fn bitwyre_instrument(key: &str) -> &str {
    match key {
        "btc_usdt_spot" => "btc_usdt_spot",
        "eth_usdt_spot" => "eth_usdt_spot",
        "usdt_idr_spot" => "usdt_idr_spot",
        "usdc_idr_spot" => "usdc_idr_spot",
        "usdc_usd_spot" => "usdc_usd_spot",
        "usdt_usd_spot" => "usdt_usd_spot",
        _ => ""
    }
}
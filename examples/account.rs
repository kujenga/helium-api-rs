use helium_api::Client;

fn main() {
    let client = Client::default();
    let account = client.get_account("13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH");
    println!("Account: {:?}", account);

    let oracle_price = client.get_current_oracle_price();
    println!("Oracle price: {:?}", oracle_price);
}

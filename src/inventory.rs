use crate::client;

fn test() {
    let config = client::Config::new("www.cv-staging.arista.io".to_string(), Some(443), "".to_string());
    let something = client::Client::new(config);
}

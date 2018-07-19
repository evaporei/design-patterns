extern crate intercepting_filter;

use intercepting_filter::{DebugFilter, AuthenticationFilter, FilterManager, Client};

fn main() {
    let mut filter_manager = FilterManager::new();
    filter_manager.set_filter(Box::new(AuthenticationFilter));
    filter_manager.set_filter(Box::new(DebugFilter));

    let client = Client::new(filter_manager);
    client.send_request("HOME");
}

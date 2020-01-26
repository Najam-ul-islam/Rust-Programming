// declaration of client module from another (client.rs)file
mod client;
mod network;

    // Inside of network Module
//    mod server{

//    }
// Access method of both modules
//network::connect
//network::server::connect











//================================================
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

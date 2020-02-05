pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        let _str = String::from("Test");
        client::connect();
    }
}

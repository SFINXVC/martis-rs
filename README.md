# The World's First *Unofficial* Martis SDK for Rust  
Introducing the first-ever unofficial Martis SDK for the Rust programming language. Yes, **before** Martis themselves even thought about releasing one, maybe?.

## Wait, what does that even mean?  
This project gives you an early look at how you'd integrate MartisPay as a payment gateway in Rust!, assuming, of course, Martis ever decides to release an official Rust SDK (spoiler: they might won't, since who actually use rust as their backend to integrate with a payment gateway?).

## Examples  
If you're wondering how to use it, check out the `examples/` folder, or just copy n paste the entire thing

### Basic usage:
```rust
// Create a new Martis client
let mut client = Client::new("your_api_key")?; // should be starting with sk_ or something, otherwise, it will returns an error
client.with_base_url("http://localhost:5279"); // why localhost? since it hasn't been released yet. (coming soon!)

// Create a new payout req (literally a disbursement)
let payout = CreatePayoutRequest {
    // fill in your payout data here...
};

// create the payout. (it's still premature for now, kinda lazy to add the error handling, so just go handle it on your own)
let response = client.create_payout(payout).await?;
```

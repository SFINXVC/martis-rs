use martis_rs::{Client, CreatePayoutRequest, PayoutDestination, BankAccountDestination, ClientError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("sk_martistest_r_2UbGb46fhucLjWyiXxEgXUnoo56Fn9T8B9wk_5C1DW4fyfsQwATwi2QsgYctxaFXHmyESV5FKDwYKgGwF")?;
    client.with_base_url("http://localhost:5279");

    let payout_request = CreatePayoutRequest {
        client_reference_id: "payout_123456".to_string(),
        amount: 100000,
        currency: "idr".to_string(),
        destination: PayoutDestination {
            destination_type: "bank_account".to_string(),
            bank_account: BankAccountDestination {
                bank_code: "bca".to_string(),
                account_number: "1234567890".to_string(),
                account_holder_name: "John Doe".to_string(),
            },
        },
        description: "Test payout from Rust SDK".to_string(),
    };

    match client.create_payout(payout_request.clone()).await {
        Ok(payout_response) => {
            println!("✅ Payout created successfully!");
            println!("Status: {}", payout_response.status);
            println!("Payout ID: {}", payout_response.data.id);
            println!("Amount: {}", payout_response.data.amount);
            println!("Currency: {}", payout_response.data.currency.name);
            println!("Status: {}", payout_response.data.status.name);
            println!("Account Holder: {}", payout_response.data.account_holder_name);
            println!("Bank: {}", payout_response.data.bank.name);
            println!("Created at: {}", payout_response.data.created_at);
        }
        Err(error) => {
            println!("❌ Payout creation failed!");
            match error {
                ClientError::ApiError { status, body } => {
                    println!("API Error - Status: {}", status);
                    println!("Response body: {}", body);
                }
                ClientError::RequestError(e) => {
                    println!("Request Error: {}", e);
                    println!("Details: This usually means network connectivity issues or the server is not running");
                    println!("Make sure the API server is running on the specified URL");
                }
                ClientError::InvalidCredentials => {
                    println!("Invalid API credentials");
                }
            }
        }
    }

    Ok(())
}
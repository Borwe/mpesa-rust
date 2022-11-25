use crate::get_mpesa_client;
use mpesa::TransactionType;
#[tokio::test]
async fn dynamic_qr_code_test() {
    let client = get_mpesa_client!();

    let response = client
        .dynamic_qrcode()
        .amount(2000)
        .credit_party_identifier("17408")
        .merchant_name("Safaricom LTD")
        .ref_no("rf38f04")
        .trx_code("BG".into())
        .send()
        .await;
    eprintln!("RES {response:?}");
    assert!(response.is_ok())
}
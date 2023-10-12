use colored::*;
use generic_search::search::SearchRequest;
use generic_search::search::search_service_client::SearchServiceClient;
use tonic::transport::{ Identity, ClientTlsConfig, Certificate, Channel };


pub async fn run(server_domain_name: &str, server_address: &str, server_port: u16, root_directory: String, query: String) -> Result<(), Box<dyn std::error::Error>> {
    let server_root_ca_cert = std::fs::read_to_string(r".\certs\chain.pem")?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    let client_cert = std::fs::read_to_string(r".\certs\client-leaf\client-leaf.pem")?;
    let client_key = std::fs::read_to_string(r".\certs\client-leaf\client-leaf.key")?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::new()
        .domain_name(server_domain_name)
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let server_address = format!("{}:{}", server_address, server_port);
    println!("Connecting to {}", server_address);

    let channel = Channel::from_shared(server_address)
        .unwrap()
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = SearchServiceClient::new(channel);

    let request = tonic::Request::new(SearchRequest {
        root_directory,
        query
    });

    let mut specs_response_stream = client.search(request).await.unwrap().into_inner();
    while let Some(specs_response) = specs_response_stream.message().await? {
        if specs_response.is_directory {
            // Print the file name in green.
            println!("{} {}", "[REMOTE][FIL] ".green(), specs_response.r#match.green());
        } else {
            // Print the directory name in green and blink.
            println!("{} {}", "[REMOTE][DIR] ".green().blink(), specs_response.r#match.green().green().blink());
        }
    }

    Ok(())

}
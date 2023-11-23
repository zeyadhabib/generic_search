use crate::helpers::ClientConfig;
use generic_search::search::SearchRequest;
use generic_search::search::search_service_client::SearchServiceClient;
use generic_search::common::{ print_remote_file_match, print_remote_directory_match };

use tonic::transport::{ ClientTlsConfig, Channel };


pub async fn run(client_config: ClientConfig, root_directory: String, query: String) -> Result<(), Box<dyn std::error::Error>> {
    let (client_identity, server_root_ca_cert) = client_config.certs_info.get();

    let tls = ClientTlsConfig::new()
        .domain_name(client_config.server_info.domain.as_str())
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity);

    let server_address = format!("{}:{}", client_config.server_info.address, client_config.server_info.port);
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
            print_remote_directory_match(specs_response.r#match.as_str());
        } else {
            // Print the directory name in green and blink.
            print_remote_file_match(specs_response.r#match.as_str());
        }
    }

    Ok(())

}
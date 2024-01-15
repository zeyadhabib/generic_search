mod helpers;

use std::pin::Pin;
use std::path::PathBuf;
use tokio_stream::{ Stream, wrappers::ReceiverStream };
use tonic::{ transport::{ Server, ServerTlsConfig }, Request, Response, Status };

use generic_search::remote_orchestrator::RemoteOrchestrator;
use generic_search::search::{ SearchRequest, SearchResponse };
use generic_search::search::search_service_server::{ SearchService, SearchServiceServer };

#[derive(Debug, Default)]
struct ZombieHuntServer {}

#[tonic::async_trait]
impl SearchService for ZombieHuntServer {
    
    type SearchStream = Pin<Box<dyn Stream<Item = Result<SearchResponse, Status>> + Send>>;

    async fn search(&self, _request: Request<SearchRequest>) -> Result<Response<Self::SearchStream>, Status> {
        let (orchestrator, stream_reciever) = RemoteOrchestrator::new(
            _request.get_ref().query.clone(),
            PathBuf::from(_request.get_ref().root_directory.clone())
        );
        tokio::spawn(async move {
            orchestrator.run().await;
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(stream_reciever))))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_config = helpers::ServerConfig::new(
        String::from(r".\zombie_hunt_server\config\server_config.yaml")
    );
    let (identity, ca_root) = server_config.certs_info.get();
    let search_service = ZombieHuntServer::default();

    let server_address = format!("{}:{}", server_config.server_info.address, server_config.server_info.port);

    let tls = ServerTlsConfig::new()
        .identity(identity)
        .client_ca_root(ca_root);

    Server::builder()
        .tls_config(tls)?
        .add_service(SearchServiceServer::new(search_service))
        .serve(server_address.parse()?)
        .await?;

    Ok(())
}
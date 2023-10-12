use std::path::PathBuf;
use std::pin::Pin;
use tokio_stream::{ Stream, wrappers::ReceiverStream };
use tonic::{ transport::{ Server, Identity, ServerTlsConfig, Certificate }, Request, Response, Status };

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
    let addr = "[::1]:50051".parse()?;
    let key = std::fs::read_to_string(r".\certs\server-leaf\server-leaf.key")?;
    let cert = std::fs::read_to_string(r".\certs\server-leaf\server-leaf.pem")?;
    let ca_root = Certificate::from_pem(std::fs::read_to_string(r".\certs\chain.pem")?);
    let identity = Identity::from_pem(cert, key);
    let search_service = ZombieHuntServer::default();

    let tls = ServerTlsConfig::new()
        .identity(identity)
        .client_ca_root(ca_root);

    Server::builder()
        .tls_config(tls)?
        .add_service(SearchServiceServer::new(search_service))
        .serve(addr)
        .await?;

    Ok(())
}
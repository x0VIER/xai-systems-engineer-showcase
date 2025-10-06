pub mod distributed_kv {
    tonic::include_proto!("distributed_kv");
}

use distributed_kv::kv_store_server::{KvStore, KvStoreServer};
use distributed_kv::{GetRequest, GetResponse, SetRequest, SetResponse, DeleteRequest, DeleteResponse};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyKvStore;

#[tonic::async_trait]
impl KvStore for MyKvStore {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetResponse {
            value: format!("Hello {}!", request.into_inner().key).into(),
        };

        Ok(Response::new(reply))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SetResponse {};

        Ok(Response::new(reply))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = DeleteResponse {};

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let kv_store = MyKvStore::default();

    Server::builder()
        .add_service(KvStoreServer::new(kv_store))
        .serve(addr)
        .await?;

    Ok(())
}


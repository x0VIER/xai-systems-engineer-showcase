use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use openraft::{
    BasicNode,
    storage::{Adaptor, RaftLogStorage, RaftStateMachine},
    testing::{MemStore, TypeConfig},
    Entry,
    LogId,
    Vote,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClientRequest {
    pub client: String,
    pub serial: u64,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientResponse {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct KvStore {
    pub data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl RaftStateMachine<TypeConfig> for KvStore {
    type Snapshot = ();

    async fn applied_state(&mut self) -> Result<(Option<LogId<u64>>, Self::Snapshot), std::io::Error> {
        Ok((None, ()))
    }

    async fn apply(&mut self, entry: &Entry<TypeConfig>) -> Result<ClientResponse, std::io::Error> {
        let req: ClientRequest = serde_json::from_slice(&entry.payload).unwrap();
        let value = req.value.clone();
        self.data.insert(req.key, value.unwrap_or_default());
        Ok(ClientResponse { value: None })
    }

    async fn begin_snapshot(&mut self) -> Result<Self::Snapshot, std::io::Error> {
        Ok(())
    }

    async fn install_snapshot(&mut self, _: &Option<LogId<u64>>, _: Self::Snapshot) -> Result<(), std::io::Error> {
        Ok(())
    }

    async fn get_snapshot(&mut self) -> Result<Option<(Self::Snapshot, Option<LogId<u64>>)>, std::io::Error> {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct StoredVote(pub Vote<u64>);

#[async_trait::async_trait]
impl RaftLogStorage<TypeConfig> for Adaptor<MemStore<TypeConfig>> {
    type LogReader = ();

    async fn get_log_reader(&mut self) -> Self::LogReader {
        ()
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<u64>>, std::io::Error> {
        Ok(None)
    }

    async fn save_vote(&mut self, vote: &Vote<u64>) -> Result<(), std::io::Error> {
        Ok(())
    }

    async fn append<I>(&mut self, entries: I, callback: Option<tokio::sync::oneshot::Sender<()>>) -> Result<(), std::io::Error>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
    {
        Ok(())
    }

    async fn truncate(&mut self, log_id: LogId<u64>) -> Result<(), std::io::Error> {
        Ok(())
    }

    async fn purge(&mut self, log_id: LogId<u64>) -> Result<(), std::io::Error> {
        Ok(())
    }
}


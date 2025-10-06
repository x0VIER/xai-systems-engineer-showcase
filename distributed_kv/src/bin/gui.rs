use eframe::{egui, App, Frame};
use tokio::runtime::Runtime;
use tonic::transport::Channel;
use distributed_kv::kv_store_client::KvStoreClient;
use distributed_kv::{GetRequest, SetRequest, DeleteRequest};

pub mod distributed_kv {
    tonic::include_proto!(\"distributed_kv\");
}

struct KvApp {
    rt: Runtime,
    client: KvStoreClient<Channel>,
    key: String,
    value: String,
    result: String,
}

impl KvApp {
    fn new(_cc: &eframe::CreationContext<'_'>) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let client = rt.block_on(async {
            KvStoreClient::connect(\"http://[::1]:50051\").await.unwrap()
        });

        Self {
            rt,
            client,
            key: \"\".to_owned(),
            value: \"\".to_owned(),
            result: \"\".to_owned(),
        }
    }
}

impl App for KvApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(\"Distributed KV Store\");

            ui.horizontal(|ui| {
                ui.label(\"Key: \");
                ui.text_edit_singleline(&mut self.key);
            });

            ui.horizontal(|ui| {
                ui.label(\"Value: \");
                ui.text_edit_singleline(&mut self.value);
            });

            ui.horizontal(|ui| {
                if ui.button(\"Get\").clicked() {
                    let mut client = self.client.clone();
                    let key = self.key.clone();
                    self.result = self.rt.block_on(async move {
                        let request = tonic::Request::new(GetRequest { key });
                        match client.get(request).await {
                            Ok(response) => response.into_inner().value,
                            Err(e) => format!(\"Error: {}\"),
                        }
                    });
                }

                if ui.button(\"Set\").clicked() {
                    let mut client = self.client.clone();
                    let key = self.key.clone();
                    let value = self.value.clone();
                    self.result = self.rt.block_on(async move {
                        let request = tonic::Request::new(SetRequest { key, value });
                        match client.set(request).await {
                            Ok(_) => \"Set successful\".to_string(),
                            Err(e) => format!(\"Error: {}\"),
                        }
                    });
                }

                if ui.button(\"Delete\").clicked() {
                    let mut client = self.client.clone();
                    let key = self.key.clone();
                    self.result = self.rt.block_on(async move {
                        let request = tonic::Request::new(DeleteRequest { key });
                        match client.delete(request).await {
                            Ok(_) => \"Delete successful\".to_string(),
                            Err(e) => format!(\"Error: {}\"),
                        }
                    });
                }
            });

            ui.separator();

            ui.label(&self.result);
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        \"Distributed KV Store\",
        native_options,
        Box::new(|cc| Box::new(KvApp::new(cc))),
    ).unwrap();
}


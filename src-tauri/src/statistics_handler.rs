use crate::util::*;
use chrono;
use statistics::stats_service_client::StatsServiceClient;
use statistics::{
    GetStatsRequest, GetStatsResponse, QueryStatsRequest, QueryStatsResponse, SysStatsRequest,
    SysStatsResponse,
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::window::Window;
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request, Response, Status};

pub mod statistics {
    tonic::include_proto!("xray.app.stats.command");
}

pub struct StatisticsHandler {
    client: Arc<Mutex<Option<StatsServiceClient<Channel>>>>,
}

#[allow(unused)]
impl StatisticsHandler {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self, window: &Window) -> Result<(), Status> {
        let mut lock = self.client.lock().await;
        if !lock.is_none() {
            return Ok(());
        }
        let mut zanki: u8 = 10;
        while lock.is_none() {
            if let Some(port) = get_available_port() {
                *lock = StatsServiceClient::connect(format!("http://127.0.0.1:{}", port))
                    .await
                    .ok();
                log!("Port {} is available", port);
            } else {
                log!("No port is available");
                return Err(Status::unavailable("No port is available"));
            }

            zanki -= 1;

            if zanki == 0 {
                log!("Failed to connect to grpc");
                return Err(Status::unavailable("Failed to connect to grpc"));
            }

            log!("I'm trying!");

            thread::sleep(Duration::from_millis(200));
        }

        log!("Grpc is connected!");

        if lock.is_none() {
            Err(Status::unavailable("Client is invalid"))
        } else {
            drop(lock);
            self.run(window, 1000).await?;
            Ok(())
        }
    }

    pub async fn stop(&self) -> Result<(), Status> {
        let mut lock = self.client.lock().await;
        if let Some(client) = &mut *lock {
            drop(client);
            *lock = None;
            log!("Grpc is disconnected!");
        }
        Ok(())
    }

    async fn run(&self, window: &Window, interval: u64) -> Result<(), Status> {
        loop {
            match self
                .query_stats(Request::new(QueryStatsRequest {
                    pattern: "".into(),
                    reset: true,
                }))
                .await
            {
                Ok(response) => {
                    let stat = &response.get_ref().stat;
                    let mut downlink = 0;
                    for item in stat {
                        if item.name == "outbound>>>proxy>>>traffic>>>downlink".to_string() {
                            downlink = item.value;
                        }
                    }
                    let bandwidth =
                        bandwitdh_display(downlink, interval).unwrap_or(BandWidth::KB(0.0));
                    let _ = emit_stats(
                        window,
                        StatsPayload {
                            outbound_proxy_traffic_downlink_speed: format!("{}", bandwidth),
                        },
                    );
                    thread::sleep(Duration::from_millis(interval));
                }
                Err(e) => {
                    log!("{:?}", e);
                    return Err(Status::unknown("Response is invalid"));
                }
            }
        }
    }

    async fn get_stats(
        &self,
        request: Request<GetStatsRequest>,
    ) -> Result<Response<GetStatsResponse>, Status> {
        let mut lock = self.client.lock().await;
        if let Some(client) = &mut *lock {
            let response = client.get_stats(request).await?;
            Ok(response)
        } else {
            Err(Status::failed_precondition(
                "Client has not been initalized",
            ))
        }
    }

    async fn query_stats(
        &self,
        request: Request<QueryStatsRequest>,
    ) -> Result<Response<QueryStatsResponse>, Status> {
        let mut lock = self.client.lock().await;
        if let Some(client) = &mut *lock {
            let response = client.query_stats(request).await?;
            Ok(response)
        } else {
            Err(Status::failed_precondition(
                "Client has not been initalized",
            ))
        }
    }

    async fn get_sys_stats(
        &self,
        request: Request<SysStatsRequest>,
    ) -> Result<Response<SysStatsResponse>, Status> {
        todo!()
    }
}

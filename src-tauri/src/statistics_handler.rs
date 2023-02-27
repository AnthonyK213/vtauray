use crate::util::*;
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
    tonic::include_proto!("statistics");
}

pub struct StatisticsHandler {
    client: Arc<Mutex<Option<StatsServiceClient<Channel>>>>,
}

impl StatisticsHandler {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self, window: &Window) -> Result<(), Status> {
        let mut lock = self.client.lock().await;
        if !lock.is_none() {
            return Ok(())
        }
        while lock.is_none() {
            *lock = StatsServiceClient::connect("http://127.0.0.1:10809").await.ok();

            #[cfg(debug_assertions)]
            let _ = emit_logging(window, 1, "I'm trying!!".into());

            thread::sleep(Duration::from_millis(200));
        }

        #[cfg(debug_assertions)]
        let _ = emit_logging(window, 1, "Grpc is connected!".into());

        if lock.is_none() {
            Err(Status::unavailable("Client is invalid"))
        } else {
            self.run(window).await?;
            Ok(())
        }
    }

    pub async fn stop(&self) -> Result<(), Status> {
        let mut lock = self.client.lock().await;
        if let Some(client) = &mut *lock {
            drop(client);
            *lock = None;
        }
        Ok(())
    }

    async fn run(&self, window: &Window) -> Result<(), Status> {
        loop {
            if let Ok(response) = self
                .get_stats(Request::new(GetStatsRequest {
                    name: "".into(),
                    reset: true,
                }))
                .await
            {
                let message = String::from(&response.get_ref().stat.as_ref().unwrap().value.to_string());
                thread::sleep(Duration::from_millis(1000));

                #[cfg(debug_assertions)]
                let _ = emit_logging(window, 1, message);
                #[cfg(debug_assertions)]
                let _ = emit_logging(window, 1, "Looping".into());
            } else {
                return Err(Status::unknown("Response is invalid"));
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

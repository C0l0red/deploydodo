use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    db,
    services::{
        JobService, ServerService, SessionService, SshService, UserService, VariablesService,
    },
};

#[derive(Clone)]
pub struct Dependencies {
    pub user_service: Arc<UserService>,
    pub session_service: Arc<SessionService>,
    pub variables_service: Arc<VariablesService>,
    pub server_service: Arc<ServerService>,
    pub ssh_service: Arc<SshService>,
    pub job_service: Arc<JobService>,
}

impl Dependencies {
    pub async fn init() -> Result<Self, sqlx::Error> {
        Self::init_with_db(Arc::new(db::create_pool().await?))
    }

    pub fn init_with_db(db: Arc<Pool<Postgres>>) -> Result<Self, sqlx::Error> {
        let user_service = Arc::new(UserService::new(db.clone()));
        let session_service = Arc::new(SessionService::new(db.clone()));
        let variables_service = Arc::new(VariablesService::new(db.clone()));
        let ssh_service = Arc::new(SshService::new(db.clone()));
        let server_service = Arc::new(ServerService::new(db.clone()));
        let job_service = Arc::new(JobService::new(db.clone()));

        Ok(Self {
            user_service,
            session_service,
            variables_service,
            server_service,
            ssh_service,
            job_service,
        })
    }
}

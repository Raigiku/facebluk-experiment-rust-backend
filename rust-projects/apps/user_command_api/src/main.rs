mod routes;
mod shared;

use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use domain::config::SharedConfig;
use integrator::{EventStore, FileStorage, MsgBroker, UserAuth};
use shared::config::Config;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

use crate::shared::error_handling::FaceblukHttpError;

#[get("/health-check")]
async fn health_check() -> actix_web::Result<impl Responder, FaceblukHttpError> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("LOGLEVEL"))
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .init();

    let api_config = Arc::new(Config::new());
    let api_port = api_config.port;

    let shared_config = Arc::new(SharedConfig::new());
    let supabase_config = Arc::new(infra_supabase::Config::new());
    let user_auth = setup_user_auth(supabase_config.clone());
    let file_storage = setup_file_storage(supabase_config.clone());
    let event_store = setup_event_store().await;
    let msg_broker = setup_msg_broker().await;

    HttpServer::new(move || {
        App::new().wrap(TracingLogger::default()).service(
            web::scope("/api")
                .app_data(web::Data::new(api_config.clone()))
                .app_data(web::Data::new(shared_config.clone()))
                .app_data(web::Data::new(event_store.clone()))
                .app_data(web::Data::new(file_storage.clone()))
                .app_data(web::Data::new(user_auth.clone()))
                .app_data(web::Data::new(msg_broker.clone()))
                .service(health_check)
                .service(routes::register_user_route::handle),
        )
    })
    .bind(("127.0.0.1", api_port))?
    .run()
    .await
}

fn setup_file_storage(config: Arc<infra_supabase::Config>) -> Arc<dyn FileStorage> {
    let user_accessor = Arc::new(infra_supabase::file_storage::UserAccessor::new(config));
    let file_storage = Arc::new(infra_supabase::file_storage::FileStorageImpl::new(
        user_accessor.clone(),
        user_accessor.clone(),
    ));
    file_storage
}

fn setup_user_auth(config: Arc<infra_supabase::Config>) -> Arc<dyn UserAuth> {
    let user_accessor = Arc::new(infra_supabase::user_auth::UserAccessor::new(config));
    let user_auth = Arc::new(infra_supabase::user_auth::UserAuthImpl::new(
        user_accessor.clone(),
        user_accessor.clone(),
    ));
    user_auth
}

async fn setup_event_store() -> Arc<dyn EventStore> {
    let pool = Arc::new(
        infra_event_store_pg::new_pool(infra_event_store_pg::Config::new())
            .await
            .unwrap(),
    );
    let user_accessor = Arc::new(infra_event_store_pg::UserAccessor::new(pool));
    let event_store = Arc::new(infra_event_store_pg::EventStoreImpl::new(
        user_accessor.clone(),
        user_accessor.clone(),
    ));
    event_store
}

async fn setup_msg_broker() -> Arc<dyn MsgBroker> {
    let rabbit_conn = Arc::new(
        infra_rabbitmq::new_connection(infra_rabbitmq::Config::new())
            .await
            .unwrap(),
    );
    let msg_broker = Arc::new(infra_rabbitmq::MsgBrokerImpl::new(rabbit_conn));
    msg_broker
}

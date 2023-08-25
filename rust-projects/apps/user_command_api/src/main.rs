mod routes;
mod shared;

use std::sync::Arc;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use integrator::{EventStore, FileStorage, MsgBroker, UserAuth};
use shared::{app_state::AppState, config::Config};

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").unwrap();
    env_logger::init_from_env(env_logger::Env::new().filter("LOGLEVEL"));

    let api_config = Config::new();
    let api_port = api_config.port;

    let supabase_config = Arc::new(infra_supabase::Config::new());
    let user_auth = setup_user_auth(supabase_config.clone());
    let file_storage = setup_file_storage(supabase_config.clone());
    let event_store = setup_event_store().await;
    let msg_broker = setup_msg_broker().await;

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new().wrap(logger).service(
            web::scope("/api")
                .app_data(web::Data::new(AppState::new(
                    api_config.clone(),
                    file_storage.clone(),
                    user_auth.clone(),
                    event_store.clone(),
                    msg_broker.clone(),
                )))
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

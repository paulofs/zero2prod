//! src/startup.rs
// see: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs

use std::net::TcpListener;

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{
        admin_dashboard, confirm, health_check, home, login, login_form, publish_newsletter,
        subscribe, change_password, change_password_form,
    },
};
use axum::{
    routing::{get, post, IntoMakeService},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
use hyper::server::conn::AddrIncoming;
use secrecy::Secret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;

pub struct Application {
    port: u16,
    server: hyper::Server<AddrIncoming, IntoMakeService<Router>>,
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
struct KeyHolder {
    key: Key,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = std::net::TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> anyhow::Result<(), hyper::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub type RunningServer = hyper::Server<AddrIncoming, IntoMakeService<Router>>;
pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>,
    _redis_uri: Secret<String>,
) -> anyhow::Result<RunningServer> {
    // let redis_store = RedisSessionStore::new(redis_uri.expose_secret().to_string())?;
    let store = axum_sessions::async_session::MemoryStore::new();
    let secret = b"123456789012345678901234567890ygufcvretrdf546sdcgtedecfvyuuit7xt";
    let session_layer = axum_sessions::SessionLayer::new(store, secret);

    let key_holder = KeyHolder {
        key: Key::generate(),
    };

    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(true)
                        .level(Level::ERROR),
                )
                .on_response(DefaultOnResponse::new().include_headers(true)),
        )
        .propagate_x_request_id();

    let app = Router::new()
        .route("/", get(home))
        .route("/login", get(login_form))
        .route("/login", post(login))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .route("/newsletters", post(publish_newsletter))
        .route("/admin/dashboard", get(admin_dashboard))
        .route("/admin/password", get(change_password_form))
        .route("/admin/password", post(change_password))
        .layer(Extension(db_pool))
        .layer(Extension(email_client))
        .layer(Extension(ApplicationBaseUrl(base_url)))
        .layer(Extension(HmacSecret(hmac_secret.clone())))
        .layer(session_layer)
        .layer(middleware)
        .with_state(key_holder);

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

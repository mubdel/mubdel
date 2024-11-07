use async_graphql::{ObjectType, Result, Schema, SubscriptionType};
use poem::{listener::TcpListener, post, EndpointExt, Route, Server};
use tracing::info;
use tracing_loki::url::Url;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use cache::conn::{connect as connect_cache, Cache};
use cfg::Config;
use database::conn::{connect, DB};
use errors::{fluent_messages, DiagCtx};
use middleware::auth::authenticator;
use payment_gate::stripe::Stripe;
use ty::service::Service;

#[derive(Clone)]
pub struct Startup<Q, M, S, H> {
    cfg: Config,
    service: Service,
    query: Q,
    mutation: M,
    subscription: S,
    handler: H,
}

impl<Q, M, S, H> Startup<Q, M, S, H>
where
    Q: Copy + ObjectType + 'static,
    M: Copy + ObjectType + 'static,
    S: Copy + SubscriptionType + 'static,
    H: poem::Endpoint + 'static,
{
    pub fn new(
        cfg_path: String,
        service: Service,
        query: Q,
        mutation: M,
        subscription: S,
        handler: H,
    ) -> Result<Self> {
        let cfg = Config::load_cfg(cfg_path)?;

        Ok(Self {
            cfg,
            service,
            query,
            mutation,
            subscription,
            handler,
        })
    }

    pub async fn startup(self) -> Result<()> {
        self.init_log()?;

        let db = self.connect_to_db().await?;
        info!("connected to database successfully");

        let cache = self.connect_to_cache().await?;
        info!("connected to cache successfully");

        let diag = self.build_diag(self.cfg.fluent().fluent_path.clone())?;
        info!("diagnostic built successfully");

        let schema = self.build_schema(db.clone(), cache.clone(), diag)?;
        info!("schema built successfully");

        let service_cfg = self.cfg.service();

        let app = Route::new().at(
            "/",
            post(self.handler)
                .around(authenticator)
                .data(schema)
                .data(db)
                .data(cache),
        );

        info!("server running on :{}", service_cfg.port);

        Server::new(TcpListener::bind(
            "0.0.0.0:".to_owned() + &service_cfg.port.to_string(),
        ))
        .run(app)
        .await?;

        Ok(())
    }

    fn init_log(&self) -> Result<()> {
        let (loki_layer, task) = tracing_loki::builder()
            .label("service", self.service.as_str())?
            .build_url(Url::parse(&self.cfg.loki_cfg().url)?)?;

        tracing_subscriber::registry().with(loki_layer).init();

        tokio::spawn(task);

        Ok(())
    }

    fn build_schema(&self, db: DB, cache: Cache, diag: DiagCtx) -> Result<Schema<Q, M, S>> {
        let mut schema = Schema::build(self.query, self.mutation, self.subscription)
            .data(db)
            .data(cache)
            .data(diag);

        if matches!(self.service, Service::Payment) {
            let stripe = self.build_payment()?;
            schema = schema.data(stripe);

            info!("setup payment gates successfully");
        }

        Ok(schema.finish())
    }

    async fn connect_to_db(&self) -> Result<DB> {
        Ok(connect(self.cfg.db()).await?)
    }

    async fn connect_to_cache(&self) -> Result<Cache> {
        Ok(connect_cache(self.cfg.cache_cfg()).await?)
    }

    fn build_diag(&self, fluent_path: String) -> Result<DiagCtx> {
        let ftl = fluent_messages(fluent_path)?;
        Ok(DiagCtx::new(ftl))
    }

    fn build_payment(&self) -> Result<Stripe> {
        let gate = self.cfg.payment_gate()?;
        Ok(Stripe::new(
            gate.stripe_secret.clone(),
            gate.stripe_wvt.clone(),
        ))
    }
}

use std::path::Path;

use async_trait::async_trait;
use axum::Router;
use loco_rs::{
    app::{AppContext, Hooks},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    task::Tasks,
    worker::{AppWorker, Processor},
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;
use tower_http::services::ServeDir;

use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};

pub struct App;

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &str) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    fn routes() -> AppRoutes {
        // 静的ファイル配信（最も優先度を低くする = 最後にマッチさせる）
        let static_router = Router::new()
            .nest_service("/static", ServeDir::new("static"))
            .nest_service("/assets", ServeDir::new("frontend/dist/assets"));

        // APIルート
        let api_router = Router::new()
            .merge(controllers::auth::routes())
            .merge(controllers::user::routes())
            .merge(controllers::post::routes());

        AppRoutes::with_default_routes()
            // APIは /api プレフィックスで
            .add_route(api_router.prefix("/api"))
            // 静的ファイルは最後に
            .add_route(static_router)
    }

    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext) {
        p.register(DownloadWorker::build(ctx));
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}

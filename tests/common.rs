use common::AppConfig;
use reqwest::Url;
use tokio::sync::OnceCell;

pub struct SpawnAppVariables {
    pub base_url: Url,
}

static CELL: OnceCell<SpawnAppVariables> = OnceCell::const_new();

pub async fn spawn_app() -> &'static SpawnAppVariables {
    CELL.get_or_init(|| async {
        let app_config = AppConfig::initialize()
            .await
            .expect("Failed to initialize app config");

        let server = app::run_app().await.expect("Failed to start application");
        let _handle = tokio::spawn(server);

        SpawnAppVariables {
            base_url: Url::parse(&format!(
                "http://{}:{}",
                app_config.app.host, app_config.app.port
            ))
            .expect("Failed to parse base URL"),
        }
    })
    .await
}

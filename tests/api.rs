use axum_postgres_polars::model::Boat;
use sqlx::Connection;
use testcontainers_modules::{
    postgres,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};
use tokio::{net::TcpListener, sync::OnceCell};

/// Use a shared DB across all tests so we don't have to spin up a new container for each test
static DB: OnceCell<ContainerAsync<postgres::Postgres>> = OnceCell::const_new();

async fn init_db() -> axum_postgres_polars::config::Db {
    let db = DB
        .get_or_init(|| async { postgres::Postgres::default().start().await.unwrap() })
        .await;
    let config = axum_postgres_polars::config::Db::new()
        .username("postgres")
        .password("postgres")
        .host(&db.get_host().await.unwrap().to_string())
        .port(db.get_host_port_ipv4(5432).await.unwrap());

    sqlx::migrate!("./migrations")
        .run(&mut sqlx::PgConnection::connect_with(&config).await.unwrap())
        .await
        .unwrap();
    config
}

async fn init_test() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());

    let config = axum_postgres_polars::config::Config {
        db: init_db().await,
    };
    let _app = tokio::task::spawn(async move {
        axum_postgres_polars::start(config, listener).await.unwrap();
    });
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            if (reqwest::get(format!("{base_url}/up")).await).is_ok() {
                break;
            }
        }
    })
    .await
    .unwrap();
    base_url
}

#[tokio::test]
pub async fn health_check() {
    let base_url = init_test().await;

    let response = reqwest::get(format!("{base_url}/up")).await.unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await.unwrap(), "I'm alive");
}

#[tokio::test]
pub async fn put_get() {
    let base_url = init_test().await;

    let client = reqwest::Client::new();
    client
        .put(format!("{base_url}/boats"))
        .json(&Boat {
            name: "Enterprise".to_owned(),
            length_ft: 36,
            rig: "Sloop".to_owned(),
        })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    client
        .put(format!("{base_url}/boats"))
        .json(&Boat {
            name: "Endeavour".to_owned(),
            length_ft: 42,
            rig: "Ketch".to_owned(),
        })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let res: Vec<Boat> = client
        .get(format!("{base_url}/boats"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(res.len(), 2);
}

#[cfg(feature = "ssr")]
use std::path::PathBuf;

#[cfg(feature = "ssr")]
use clap::Parser;

#[cfg(feature = "ssr")]
#[derive(Parser)]
struct Args {
    #[arg(short = 'd', long = "db", help = "The path to the sqlite db")]
    db_path: std::path::PathBuf,
    #[arg(short = 'q', long = "create", help = "Create the database and exit", action = clap::ArgAction::SetTrue)]
    create: bool,
    #[arg(short='v', long, action = clap::ArgAction::Count, help="Sets the verbose level. More v's more output")]
    verbose: u8,

    #[arg(short = 'c', long = "config", help = "The config file to use")]
    config: PathBuf,
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::process::exit;
    use std::sync::Arc;

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use strichliste_rs::app::*;

    use strichliste_rs::backend::database::DB;
    use strichliste_rs::backend::{core::ServerState, core::Settings, core::State};

    use tokio::sync::Mutex;
    use tracing::error;
    use tracing_subscriber::EnvFilter;

    let args = Args::parse();

    let level = match args.verbose {
        0 => "info,sqlx=warn",
        1 => "debug,sqlx=warn",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    let settings = match Settings::new(args.config) {
        Ok(mut settings) => {
            if settings.accounts.lower_limit > 0 {
                error!("Failed to parse config: accounts.lower_limit may not be positive!");
                exit(1);
            }

            if settings.accounts.upper_limit < 0 {
                error!("Failed to parse config: accounts.upper_limit may not be negative!");
                exit(1);
            }

            if settings.accounts.upper_limit == 0 {
                settings.accounts.upper_limit = i64::MAX;
            }

            settings
        }
        Err(e) => {
            error!("Failed to parse config: {e}");
            exit(1);
        }
    };

    let path = args.db_path;

    let db = match DB::new(path.to_str().unwrap()).await {
        Ok(db) => db,
        Err(err) => {
            error!("Failed to create database: {},", err.to_string());
            exit(1);
        }
    };

    if args.create {
        use tracing::info;

        db.close().await;
        info!("Created database and exiting.");
        exit(0);
    }

    let server_state: ServerState = Arc::new(State {
        db: Mutex::new(db),
        settings,
    });

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(server_state.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

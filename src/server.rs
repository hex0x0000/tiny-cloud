// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{api, config, error::RequestError, plugins::Plugins, utils, webui};
use actix_identity::IdentityMiddleware;
use actix_multipart::form::MultipartFormConfig;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{
    App, HttpServer,
    cookie::{Key, SameSite, time::Duration},
    error, middleware,
    web::{self, Data},
};
use async_sqlite::Pool;
use common_library::error::ErrToResponse;

fn warn_msg(binding: &str) {
    log::info!("Binding to {binding}");
    log::warn!("TLS is disabled.");
    log::warn!(
        "This is safe *ONLY* if you are running this server behind a reverse proxy (with TLS) or if you are running the server locally."
    );
    log::warn!("Any other configuration is *UNSAFE* and may be subject to cyberattacks.");
}

pub async fn start(secret_key: Key, database: Pool, plugins: Plugins) -> Result<(), String> {
    let database = Data::new(database);
    let plugins = Data::new(plugins);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .app_data(Data::clone(&database))
            .app_data(Data::clone(&plugins))
            .app_data(
                web::JsonConfig::default()
                    .limit(*config!(limits.payload_size))
                    .error_handler(|err, _| {
                        let err_msg = err.to_string();
                        error::InternalError::from_response(err, RequestError::Json(err_msg).to_response()).into()
                    }),
            )
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(*config!(limits.file_upload_size))
                    .memory_limit(*config!(limits.payload_size))
                    .error_handler(|err, _| {
                        let err_msg = err.to_string();
                        error::InternalError::from_response(err, RequestError::Multipart(err_msg).to_response()).into()
                    }),
            )
            .app_data(web::QueryConfig::default().error_handler(|err, _| {
                let err_msg = err.to_string();
                error::InternalError::from_response(err, RequestError::Query(err_msg).to_response()).into()
            }))
            .wrap(
                IdentityMiddleware::builder()
                    .login_deadline(config!(duration.login_minutes).map(|d| std::time::Duration::from_secs(d * 60)))
                    .visit_deadline(config!(duration.visit_minutes).map(|d| std::time::Duration::from_secs(d * 60)))
                    .build(),
            )
            .wrap({
                let session_middleware = SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth".to_owned())
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::minutes((*config!(duration.cookie_minutes)).into())),
                    );
                #[cfg(feature = "no-tls")]
                {
                    session_middleware.cookie_secure(false).build()
                }
                #[cfg(not(feature = "no-tls"))]
                {
                    session_middleware.cookie_secure(config!(tls).is_some()).build()
                }
            })
            .service(web::redirect(utils::make_url(""), utils::make_url("/ui")))
            .service(
                web::scope(config!(url_prefix))
                    .service(
                        web::scope("/static")
                            .route("/favicon.ico", web::get().to(webui::images::favicon))
                            .route("/logo.png", web::get().to(webui::images::logo)),
                    )
                    .service(
                        web::scope("/ui")
                            .service(webui::root)
                            .service(webui::register_page)
                            .service(webui::login_page)
                            .service(webui::settings_page),
                    )
                    .service(
                        web::scope("/api")
                            .service(api::info)
                            .service(api::plugins::handler)
                            .service(api::plugins::file)
                            .service(
                                web::scope("/auth")
                                    .service(api::auth::login)
                                    .service(api::auth::register)
                                    .service(api::auth::logout)
                                    .service(api::auth::logoutall)
                                    .service(api::auth::delete)
                                    .service(api::auth::changepwd)
                                    .service(api::auth::changetotp),
                            )
                            .service(
                                web::scope("/token")
                                    .service(api::token::new)
                                    .service(api::token::delete)
                                    .service(api::token::list),
                            ),
                    ),
            )
    });

    // Setting TLS
    let server = {
        let binding = format!("{}:{}", config!(server.host), config!(server.port));
        #[cfg(feature = "openssl")]
        {
            use crate::tls;
            if let Some(config) = config!(tls) {
                log::info!("Binding to {binding} with TLS (openssl)");
                server
                    .bind_openssl(binding, tls::get_openssl_config(config)?)
                    .map_err(|e| format!("Failed to bind server with TLS (openssl): {e}"))?
            } else {
                warn_msg(&binding);
                server.bind(binding).map_err(|e| format!("Failed to bind server: {e}"))?
            }
        }

        #[cfg(feature = "rustls")]
        {
            use crate::tls;
            if let Some(config) = config!(tls) {
                log::info!("Binding to {binding} with TLS (rustls)");
                server
                    .bind_rustls_0_23(binding, tls::get_rustls_config(config)?)
                    .map_err(|e| format!("Failed to bind server with TLS (rustls): {e}"))?
            } else {
                warn_msg(&binding);
                server.bind(binding).map_err(|e| format!("Failed to bind server: {e}"))?
            }
        }

        #[cfg(feature = "no-tls")]
        {
            warn_msg(&binding);
            server.bind(binding).map_err(|e| format!("Failed to bind server: {e}"))?
        }
    };

    log::info!("Starting Tiny Cloud on version {}...", env!("CARGO_PKG_VERSION"),);
    server
        .workers(*config!(server.workers))
        .run()
        .await
        .map_err(|e| format!("Error while running: {e}"))?;
    Ok(())
}

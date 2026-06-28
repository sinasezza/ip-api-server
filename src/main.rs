mod config;
mod models;

use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};
use log::{error, info};

use ipapi::{query_ip, query_own_ip};

use config::Config;
use models::*;

fn convert_ip_info(ip_info: ipapi::IPInfo) -> IPInfoResponse {
    IPInfoResponse {
        ip: ip_info.ip,
        isp: ip_info.isp.map(|isp| ISPDInfo {
            asn: isp.asn,
            org: isp.org,
            isp: isp.isp,
        }),
        location: ip_info.location.map(|loc| LocationInfoD {
            country: loc.country,
            country_code: loc.country_code,
            city: loc.city,
            state: loc.state,
            zipcode: loc.zipcode,
            latitude: loc.latitude,
            longitude: loc.longitude,
            timezone: loc.timezone,
            localtime: loc.localtime,
        }),
        risk: ip_info.risk.map(|risk| RiskInfoD {
            is_mobile: risk.is_mobile,
            is_vpn: risk.is_vpn,
            is_tor: risk.is_tor,
            is_proxy: risk.is_proxy,
            is_datacenter: risk.is_datacenter,
            risk_score: risk.risk_score,
        }),
    }
}

// Health check endpoint
async fn health_check() -> HttpResponse {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: std::env::var("RUST_ENV").unwrap_or_else(|_| "unknown".to_string()),
        timestamp,
    };

    HttpResponse::Ok().json(response)
}

// Get IP information
async fn get_ip_info(web::Query(params): web::Query<QueryParams>) -> HttpResponse {
    let ip_address = match params.ip {
        Some(ip) => {
            if ip.is_empty() {
                return HttpResponse::BadRequest().json(ApiResponse::<String> {
                    success: false,
                    data: None,
                    error: Some("IP address cannot be empty".to_string()),
                    message: "Invalid request".to_string(),
                });
            }
            ip
        }
        None => "8.8.8.8".to_string(), // Default fallback
    };

    info!("Querying IP information for: {}", ip_address);

    match query_ip(&ip_address).await {
        Ok(ip_info) => {
            let converted_info = convert_ip_info(ip_info);
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(converted_info),
                error: None,
                message: "IP information retrieved successfully".to_string(),
            })
        }
        Err(e) => {
            error!("Failed to query IP: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<String> {
                success: false,
                data: None,
                error: Some(format!("Failed to query IP information: {}", e)),
                message: "Internal server error".to_string(),
            })
        }
    }
}

// Get own IP address
async fn get_own_ip() -> HttpResponse {
    match query_own_ip().await {
        Ok(ip) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ip),
            error: None,
            message: "Your public IP address retrieved successfully".to_string(),
        }),
        Err(e) => {
            error!("Failed to get own IP: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<String> {
                success: false,
                data: None,
                error: Some(format!("Failed to get own IP: {}", e)),
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration from .env
    let config = Config::from_env().expect("Failed to load configuration");

    // Initialize logger based on config.log_level
    env_logger::Builder::from_default_env()
        .filter_level(match config.log_level.as_str() {
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        })
        .init();

    info!("Starting IP API Server...");
    info!("Environment: {}", config.rust_env);
    info!(
        "Server will run on: {}:{}",
        config.server_host, config.server_port
    );

    // Clone config variables to move into the HttpServer closure
    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/api/ip", web::get().to(get_ip_info))
            .route("/api/own-ip", web::get().to(get_own_ip))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}

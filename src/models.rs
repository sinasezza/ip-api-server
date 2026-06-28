use serde::{Deserialize, Serialize};

// --- Incoming Request Structs ---

#[derive(Deserialize)]
pub struct QueryParams {
    pub ip: Option<String>,
}

// --- Outgoing Response Structs ---

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct IPInfoResponse {
    pub ip: String,
    pub isp: Option<ISPDInfo>,
    pub location: Option<LocationInfoD>,
    pub risk: Option<RiskInfoD>,
}

#[derive(Serialize)]
pub struct ISPDInfo {
    pub asn: Option<String>,
    pub org: Option<String>,
    pub isp: Option<String>,
}

#[derive(Serialize)]
pub struct LocationInfoD {
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub localtime: Option<String>,
}

#[derive(Serialize)]
pub struct RiskInfoD {
    pub is_mobile: Option<bool>,
    pub is_vpn: Option<bool>,
    pub is_tor: Option<bool>,
    pub is_proxy: Option<bool>,
    pub is_datacenter: Option<bool>,
    pub risk_score: Option<u8>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub environment: String,
    pub timestamp: String,
}

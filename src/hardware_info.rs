use std::fs;
use uuid::{Uuid};
use local_ip_address::local_ip;
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

pub fn  cpu_avg() -> String {
    return read_file("/proc/loadavg")
}

pub fn  mem_usage() -> String {
    return read_file("/proc/meminfo")
}

pub fn  generate_uuid() -> Uuid {
    return Uuid::new_v4();
}

fn  read_file(path: &str) -> String {
    return fs::read_to_string(path).expect(format!("{} {}", "Something went wrong reading the file", path).as_str())
}


#[derive(Serialize, Deserialize)]
struct MyIpLocation {
    pub ip: String,
    pub hostname: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
    pub readme: String,
}

// pub async fn get_location() -> (f32, f32) {
pub async fn get_location() -> MyIpLocation {
    // let my_local_ip = local_ip().unwrap();
    let client = Client::new();
    let my_local_ip = client.get("https://api.myip.com")
        .send().await.unwrap().text().await.ok().unwrap();
    let p: MyIpLocation = serde_json::from_str(my_local_ip)?;

    return p;
}

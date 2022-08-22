use std::collections::HashMap;

use uuid::Uuid;
// use log::{info, warn};

// pub fn send_mmm(id: Uuid, memory: &String, cpu: &String, location: (f32, f32)) {
pub fn send_mmm(id: Uuid, memory: &String, cpu: &String, location: String) {
    let mut map = HashMap::new();
    map.insert("id", id.to_string());
    map.insert("memory", memory.to_string());
    map.insert("cpu", cpu.to_string());
    map.insert("location", location);
    // map.insert("longitude", location.0.to_string());
    // map.insert("lattitude", location.1.to_string());

    print!("body {:?}", map);
    // let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .json(&map)
    //     .send()
    //     .await?;
}
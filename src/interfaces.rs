use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum InfoType {
    Disks,
    CPUs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub info_type: InfoType,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub file_system: String,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CPUInfo {
    pub name: String,
    pub usage: f32,
    pub vendor: String,
    pub brand: String,
    pub frequency: u64,
}

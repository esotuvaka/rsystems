use crate::shared::location::{Location, VehicleId};

#[derive(Debug, Clone)]
pub enum VehicleStatus {
    Idle,
    Loading,
    InTransit,
    Maintenance,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: VehicleId,
    pub name: String,
    pub max_capacity_kg: f32,
    pub current_capacity_kg: f32,
    pub location: Location,
    pub status: VehicleStatus,
}

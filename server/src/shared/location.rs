use uuid::Uuid;

pub type WarehouseId = Uuid;
pub type PackageId = Uuid;
pub type ShipmentId = Uuid;
pub type VehicleId = Uuid;
pub type RouteId = Uuid;

#[derive(Debug, Clone)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

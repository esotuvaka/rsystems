use crate::shared::location::{PackageId, RouteId, ShipmentId, VehicleId};

#[derive(Debug, Clone)]
pub enum ShipmentStatus {
    Pending,
    Loading,
    InTransit,
    Delivered,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Shipment {
    pub id: ShipmentId,
    pub package_ids: Vec<PackageId>,
    pub vehicle_id: VehicleId,
    pub route_id: RouteId,
    pub status: ShipmentStatus,
}

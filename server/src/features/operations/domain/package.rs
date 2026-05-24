use chrono::{DateTime, Utc};

use crate::shared::location::{PackageId, VehicleId, WarehouseId};

#[derive(Debug, Clone)]
pub enum PackageStatus {
    Created,
    AtWarehouse(WarehouseId),
    InTransit(VehicleId),
    Delivered,
    Lost,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub id: PackageId,
    pub weight_kg: f32,
    pub destination: WarehouseId,
    pub status: PackageStatus,
    pub created_at: DateTime<Utc>,
}

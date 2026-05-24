use crate::shared::location::{RouteId, WarehouseId};

#[derive(Debug, Clone)]
pub struct Route {
    pub id: RouteId,
    pub origin: WarehouseId,
    pub destination: WarehouseId,
    pub distance_km: f32,
    pub estimated_minutes: u32,
}

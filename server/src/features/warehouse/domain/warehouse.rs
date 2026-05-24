use uuid::Uuid;

use crate::shared::location::{Location, WarehouseId};

#[derive(Debug, Clone)]
pub struct Warehouse {
    pub id: WarehouseId,
    pub name: String,
    pub location: Location,
    pub capacity: u32,
    pub current_load: u32,
}

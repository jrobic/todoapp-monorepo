use crate::domain::entity::health_entity::Health;

pub struct HealthUsecase {}

impl HealthUsecase {
	pub fn new() -> Self {
		Self {}
	}

	pub fn get_health(&self) -> Health {
		Health {
			status: "OK".to_string(),
			version: "1.0.0".to_string(),
			uptime: 0,
		}
	}
}

use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum _DeviceStatus {
    Done,
    OtherError,
    DbAPIError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceStatus {
    status_code: u8,
    status: _DeviceStatus,
    db_api_status: DbAPIStatus,
}

impl Default for DeviceStatus {
    fn default() -> Self {
        DeviceStatus {
            status_code: 0,
            status: _DeviceStatus::Done,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl StatusTrait for DeviceStatus {
    type StatusCode = u8;
    type Status = _DeviceStatus;
    type DbAPIStatus = DbAPIStatus;
    type _DbAPIStatus = _DbAPIStatus;

    fn set_status(self, status: Self::Status) -> Self {
        DeviceStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    fn set_db_api_status(self, status: Self::DbAPIStatus) -> Self {
        DeviceStatus {
            db_api_status: status,
            ..self
        }
    }

    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self {
        DeviceStatus::default().set_status(_DeviceStatus::DbAPIError)
            .set_db_api_status(DbAPIStatus::new(status, e))
    }

    fn set_db_api_err_simple(status: Self::DbAPIStatus) -> Self {
        DeviceStatus::default().set_status(_DeviceStatus::DbAPIError)
            .set_db_api_status(status)
    }

    fn status_code(&self) -> Self::StatusCode {
        self.status_code
    }

    fn status(&self) -> _DeviceStatus {
        self.status
    }

    fn db_api_status(&self) -> Self::DbAPIStatus {
        DbAPIStatus::clone(&self.db_api_status)
    }
}

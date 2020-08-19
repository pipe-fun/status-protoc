use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _TaskStatus {
    Done,
    TokenHasNoExist,
    OtherError,
    DbAPIError,
}

#[derive(Serialize, Deserialize)]
pub struct TaskStatus {
    status_code: u8,
    status: _TaskStatus,
    db_api_status: DbAPIStatus,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus {
            status_code: 0,
            status: _TaskStatus::Done,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl StatusTrait for TaskStatus {
    type StatusCode = u8;
    type Status = _TaskStatus;
    type DbAPIStatus = DbAPIStatus;
    type _DbAPIStatus = _DbAPIStatus;

    fn set_status(self, status: Self::Status) -> Self {
        TaskStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    fn set_db_api_status(self, status: Self::DbAPIStatus) -> Self {
        TaskStatus {
            db_api_status: status,
            ..self
        }
    }

    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self {
        TaskStatus::default().set_status(_TaskStatus::DbAPIError)
            .set_db_api_status(DbAPIStatus::new(status, e))
    }

    fn set_db_api_err_simple(status: Self::DbAPIStatus) -> Self {
        TaskStatus::default().set_status(_TaskStatus::DbAPIError)
            .set_db_api_status(status)
    }

    fn status_code(&self) -> Self::StatusCode {
        self.status_code
    }

    fn status(&self) -> _TaskStatus {
        self.status
    }

    fn db_api_status(&self) -> Self::DbAPIStatus {
        DbAPIStatus::clone(&self.db_api_status)
    }
}

use crate::{bean::resp::{self, ActivationCode}, status::SystemError};
use chrono::{NaiveDateTime, Utc, Duration};
use core::result::Result;
use sqlx::{query, query_as, PgPool};

pub struct RegistrationSystem {
    db: PgPool,
}

impl RegistrationSystem {
    pub async fn new(database_url: &str) -> Result<Self, SystemError> {
        let db = PgPool::connect(database_url)
            .await
            .map_err(SystemError::FailedToConnect)?;
        Ok(Self { db })
    }

    pub async fn create_activation(&self, code: ActivationCode)-> Result<ActivationCode, SystemError> {
        query_as::<_, resp::ActivationCode>("INSERT INTO activation_codes (code, used, end_hour) VALUES ($1, $2, $3) RETURNING *")
            .bind(code.code)
            .bind(code.used)
            .bind(code.end_hour)
            .fetch_one(&self.db)
            .await
            .map_err(|_|SystemError::ActivationCodeFailedToGen)
    }

    pub async fn create_device(
        &self,
        name: &str,
        serial_number: &str,
    ) -> Result<resp::Device, SystemError> {
        let now = Utc::now().naive_utc();
        let device_id = uuid::Uuid::new_v4().to_string();
        let device = query_as::<_, resp::Device>("INSERT INTO devices (id, name, serial_number, trial_end_date, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(&device_id)
            .bind(name)
            .bind(serial_number)
            .bind(now + Duration::days(1))
            .bind(now)
            .fetch_one(&self.db)
            .await;

        if device.is_err() {
            return self.get_device_by_serial_number(serial_number).await;
        }

        Ok(device.unwrap())
    }

    pub async fn get_device_by_serial_number(
        &self,
        serial_number: &str,
    ) -> Result<resp::Device, SystemError> {
        query_as::<_, resp::Device>("SELECT * FROM devices WHERE serial_number = $1")
            .bind(serial_number)
            .fetch_one(&self.db)
            .await
            .map_err(|_| SystemError::AccountDoesNotExist)
    }

    pub async fn get_device_left(
        &self,
        device_id: &str,
    ) -> Result<(i64, NaiveDateTime), SystemError> {
        let activation = self.get_activation_by_device_id(device_id).await;
        if activation.is_ok() {
            let activation = activation.unwrap();
            let hour = activation.end_hour;
            let now = Utc::now().naive_utc();
            if activation.activated_at.is_none() {
                return Ok((0, now));
            }
            let left = (activation.activated_at.unwrap() + Duration::hours(hour) - Utc::now().naive_utc()).num_milliseconds();
            return Ok((left, now + Duration::hours(hour as i64)));
        }
        let device = self.get_device_by_id(device_id).await?;
        if device.is_in_trial_period() {
            let trial_end_date = device.trial_end_date;
            let left = (trial_end_date - Utc::now().naive_utc()).num_milliseconds();
            return Ok((left, trial_end_date));
        }
        Ok((0, Utc::now().naive_utc()))
    }

    pub async fn unactivate_activation_code(
        &self,
        code: &str,
    ) -> Result<resp::ActivationCode, SystemError> {
        let mut activation_code = self.get_activation_by_code(code).await?;
        activation_code.used = false;
        activation_code.device_id = None;
        activation_code.end_hour = std::cmp::max(0, activation_code.end_hour - 24); 

        return query("UPDATE activation_codes SET used = $1, device_id = $2, end_hour = $3 WHERE id = $4")
                    .bind(activation_code.used)
                    .bind(&activation_code.device_id)
                    .bind(activation_code.end_hour)
                    .bind(activation_code.id)
                    .execute(&self.db)
                    .await
                    .map(|_|activation_code)
                    .map_err(|_|SystemError::ActivationCodeFailedToUpdate);
    }

    pub async fn unactivate_activation_code_by_device_id(
        &self,
        device_id: &str,
    ) -> Result<resp::ActivationCode, SystemError> {
        let activation_code = self.get_activation_by_device_id(device_id).await?;
        self.unactivate_activation_code(&activation_code.code).await
    }

    pub async fn activate_device_with_activation_code(
        &self,
        code: &str,
        device_id: &str,
    ) -> Result<resp::ActivationCode, SystemError> {
        let device = self.get_device_by_id(device_id).await?;

        let mut activation_code = self.get_activation_by_code(code).await?;

        if activation_code.used {
            return Err(SystemError::ActivationCodeAlreadyUsed);
        }

        if activation_code.activated_at.is_some() {
            if activation_code.activated_at.unwrap() + Duration::hours(activation_code.end_hour) < Utc::now().naive_utc() {
                return Err(SystemError::ActivationCodeExpired);
            }
        }

        let _ = self
            .unactivate_activation_code_by_device_id(device_id)
            .await;

        activation_code.activated_at = Some(Utc::now().naive_utc());
        activation_code.used = true;
        activation_code.device_id = Some(device.id.clone());

        return query("UPDATE activation_codes SET activated_at = $1, used = $2, device_id = $3 WHERE id = $4")
                    .bind(activation_code.activated_at)
                    .bind(activation_code.used)
                    .bind(&activation_code.device_id)
                    .bind(activation_code.id).execute(&self.db)
                    .await
                    .map(|_|activation_code)
                    .map_err(|_|SystemError::ActivationCodeFailedToUpdate);
    }

    pub async fn get_device_by_id(&self, device_id: &str) -> Result<resp::Device, SystemError> {
        query_as::<_, resp::Device>("SELECT * FROM devices WHERE id = $1")
            .bind(device_id)
            .fetch_one(&self.db)
            .await
            .map_err(|_| SystemError::AccountDoesNotExist)
    }

    pub async fn get_activation_by_device_id(
        &self,
        device_id: &str,
    ) -> Result<resp::ActivationCode, SystemError> {
        query_as::<_, resp::ActivationCode>("SELECT * FROM activation_codes WHERE device_id = $1")
            .bind(device_id)
            .fetch_one(&self.db)
            .await
            .map_err(|_| SystemError::ActivationCodeDoesNotExist)
    }

    pub async fn get_activation_by_code(
        &self,
        code: &str,
    ) -> Result<resp::ActivationCode, SystemError> {
        query_as::<_, resp::ActivationCode>("SELECT * FROM activation_codes WHERE code = $1")
            .bind(code)
            .fetch_one(&self.db)
            .await
            .map_err(|_| SystemError::ActivationCodeDoesNotExist)
    }
}

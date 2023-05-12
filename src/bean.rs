pub mod req {
    use rocket::serde::Deserialize;
    #[derive(Debug, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct NewDevice {
        pub name: String,
        pub serial_number: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct NewActivationCode {
        pub code: String,
    }
}

pub mod resp {

    use crate::{
        ser::{to_timestamp, to_timestamp_option},
        status::{Message, SystemError},
    };
    use rocket::serde::Serialize;
    use sqlx::FromRow;

    #[derive(Debug, FromRow, Serialize)]
    pub struct Device {
        pub id: String,
        pub name: String,
        pub serial_number: String,

        #[serde(serialize_with = "to_timestamp")]
        pub trial_end_date: chrono::NaiveDateTime,

        #[serde(serialize_with = "to_timestamp")]
        pub created_at: chrono::NaiveDateTime,
    }

    #[derive(Debug, FromRow, Serialize)]
    pub struct ActivationCode {
        pub id: i32,
        pub code: String,
        pub device_id: Option<String>,
        pub used: bool,

        #[serde(serialize_with = "to_timestamp_option")]
        pub activated_at: Option<chrono::NaiveDateTime>,

        pub end_hour: i64,
    }

    #[derive(Debug, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Left {
        pub left: i64,

        #[serde(serialize_with = "to_timestamp")]
        pub ts: chrono::NaiveDateTime,
    }

    #[derive(Debug, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct ResultData<T> {
        pub code: u16,
        pub msg: String,
        pub data: Option<T>,
    }

    impl<T> ResultData<T> {
        pub fn ok(msg: Message, device: T) -> Self {
            let s: String = msg.into();
            Self {
                code: 200,
                msg: s.to_string(),
                data: Some(device),
            }
        }

        pub fn err(msg: SystemError) -> Self {
            Self {
                code: 500,
                msg: msg.into(),
                data: None,
            }
        }
    }

    impl Device {
        pub fn is_in_trial_period(&self) -> bool {
            self.trial_end_date > chrono::Utc::now().naive_utc()
        }
    }
}

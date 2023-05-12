mod bean;
mod db;
mod ser;
mod status;

#[macro_use]
extern crate rocket;
use bean::{req, resp};
use db::RegistrationSystem;
use rocket::{http::Status, serde::json::Json, Config, Request};
use status::{Message, SystemError};
use std::net::Ipv4Addr;

#[post("/device", format = "json", data = "<new_device>")]
async fn create_device(
    new_device: Json<req::NewDevice>,
    registration_system: &rocket::State<RegistrationSystem>,
) -> Result<Json<resp::ResultData<resp::Device>>, Json<resp::ResultData<resp::Device>>> {
    let device = registration_system
        .create_device(&new_device.name, &new_device.serial_number)
        .await
        .map_err(|e| Json(resp::ResultData::err(e)))?;

    Ok(Json(resp::ResultData::ok(Message::RegisterSuccess, device)))
}

#[get("/device/<serial_number>")]
async fn get_device_by_serial_number(
    serial_number: &str,
    registration_system: &rocket::State<RegistrationSystem>,
) -> Result<Json<resp::ResultData<resp::Device>>, Json<resp::ResultData<resp::Device>>> {
    let device = registration_system
        .get_device_by_serial_number(serial_number)
        .await
        .map_err(|e| Json(resp::ResultData::err(e)))?;

    Ok(Json(resp::ResultData::ok(Message::Success, device)))
}

#[get("/device/<device_id>/left")]
async fn get_device_left(
    device_id: &str,
    registration_system: &rocket::State<RegistrationSystem>,
) -> Result<Json<resp::ResultData<resp::Left>>, Json<resp::ResultData<resp::Left>>> {
    let (mut left, ts) = registration_system
        .get_device_left(device_id)
        .await
        .map_err(|e| Json(resp::ResultData::err(e)))?;

    if left < 0 {
        left = 0;
    }

    Ok(Json(resp::ResultData::ok(
        Message::Success,
        resp::Left { left, ts },
    )))
}

#[get("/generator/admin/<hour>")]
async fn generator(
    hour: i64,
    registration_system: &rocket::State<RegistrationSystem>,
) -> Result<
    Json<resp::ResultData<resp::ActivationCode>>,
    Json<resp::ResultData<resp::ActivationCode>>,
> {
    let code = uuid::Uuid::new_v4().to_string();
    let code = registration_system
        .create_activation(resp::ActivationCode {
            id: 0,
            code,
            device_id: None,
            used: false,
            activated_at: None,
            end_hour: hour,
        })
        .await
        .map_err(|_| Json(resp::ResultData::err(SystemError::ActivationCodeNotMatch)))?;

    Ok(Json(resp::ResultData::ok(
        Message::ActivationCodeSuccessToGen,
        code,
    )))
}

#[post(
    "/device/<device_id>/unactivate",
    format = "json",
    data = "<new_activation_code>"
)]
async fn unactivate_device_with_activation_code(
    device_id: &str,
    new_activation_code: Json<req::NewActivationCode>,
    registration_system: &rocket::State<RegistrationSystem>,
) -> Result<
    Json<resp::ResultData<resp::ActivationCode>>,
    Json<resp::ResultData<resp::ActivationCode>>,
> {
    let activation_code = registration_system
        .get_activation_by_device_id(device_id)
        .await
        .map_err(|_| Json(resp::ResultData::err(SystemError::ActivationCodeNotMatch)))?;

    if activation_code.code != new_activation_code.code {
        return Err(Json(resp::ResultData::err(
            SystemError::ActivationCodeNotMatch,
        )));
    }

    let code = registration_system
        .unactivate_activation_code_by_device_id(device_id)
        .await
        .map_err(|e| Json(resp::ResultData::err(e)))?;

    Ok(Json(resp::ResultData::ok(Message::UnactivateSuccess, code)))
}

#[post(
    "/device/<device_id>/activate",
    format = "json",
    data = "<new_activation_code>"
)]
async fn activate_device_with_activation_code(
    device_id: &str,
    new_activation_code: Json<req::NewActivationCode>,
    registration_system: &rocket::State<RegistrationSystem>,
) -> Result<
    Json<resp::ResultData<resp::ActivationCode>>,
    Json<resp::ResultData<resp::ActivationCode>>,
> {
    let code = registration_system
        .activate_device_with_activation_code(&new_activation_code.code, device_id)
        .await
        .map_err(|e| Json(resp::ResultData::err(e)))?;

    Ok(Json(resp::ResultData::ok(Message::ActivateSuccess, code)))
}

#[catch(default)]
fn exception(status: Status, req: &Request) -> Json<resp::ResultData<resp::ActivationCode>> {
    Json(resp::ResultData {
        code: status.code,
        msg: req.uri().to_string(),
        data: None,
    })
}

#[launch]
async fn rocket() -> _ {
    let registration_system =
        RegistrationSystem::new("postgres://postgres:password@192.168.120.137/registration_system")
            .await
            .expect("Failed to create registration system");

    rocket::build()
        .register("/", catchers![exception])
        .configure(rocket::Config {
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::debug_default()
        })
        .manage(registration_system)
        .mount(
            "/api",
            routes![
                activate_device_with_activation_code,
                create_device,
                generator,
                get_device_by_serial_number,
                get_device_left,
                unactivate_device_with_activation_code
            ],
        )
}

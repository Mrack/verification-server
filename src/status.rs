use thiserror::Error;

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("error")]
    Error(String),

    #[error("连接数据库失败")]
    FailedToConnect(sqlx::Error),

    #[error("账号不存在")]
    AccountDoesNotExist,

    #[error("激活码更新失败")]
    ActivationCodeFailedToUpdate,

    #[error("激活码已经被使用")]
    ActivationCodeAlreadyUsed,

    #[error("激活码不存在")]
    ActivationCodeDoesNotExist,

    #[error("激活码生成失败")]
    ActivationCodeFailedToGen,

    #[error("激活码已经过期")]
    ActivationCodeExpired,

    #[error("未找到激活记录")]
    ActivationCodeNotFound,

    #[error("激活码不匹配")]
    ActivationCodeNotMatch,

    #[error("解绑失败")]
    UnactivationCodeFailed,
}

impl Into<String> for SystemError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

#[derive(Debug)]
pub enum Message {
    Success,
    RegisterSuccess,
    ActivateSuccess,
    UnactivateSuccess,
    ActivationCodeSuccessToGen,
}

impl Into<String> for Message {
    fn into(self) -> String {
        match self {
            Message::Success => "success",
            Message::RegisterSuccess => "登录成功",
            Message::ActivateSuccess => "激活成功",
            Message::UnactivateSuccess => "解绑成功",
            Message::ActivationCodeSuccessToGen => "激活码生成成功",
            _ => "",
        }
        .to_string()
    }
}

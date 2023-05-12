pub fn to_timestamp_option<S>(
    value: &Option<chrono::NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    if value.is_none() {
        return serializer.serialize_i64(-1);
    }
    to_timestamp(&value.unwrap(), serializer)
}

pub fn to_timestamp<S>(value: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    serializer.serialize_i64(value.timestamp())
}

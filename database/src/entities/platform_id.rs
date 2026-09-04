use std::{num::ParseIntError, str::FromStr};

use sea_orm::{
    prelude::StringLen,
    sea_query::{ArrayType, ValueType, ValueTypeErr},
    *,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformId {
    Fluxer {
        snowflake: u32,
        instance_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BackendIdParseError {
    #[error("missing backend platform")]
    MissingPlatform,

    #[error("unknown backend platform `{0}`")]
    UnknownPlatform(String),

    #[error("missing Fluxer instance name")]
    MissingInstanceName,

    #[error("invalid Fluxer snowflake")]
    InvalidSnowflake(#[from] ParseIntError),
}

impl std::fmt::Display for PlatformId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            PlatformId::Fluxer {
                snowflake,
                instance_name,
            } => write!(f, "fluxer:{snowflake}:{instance_name}"),
        };
    }
}

impl FromStr for PlatformId {
    type Err = BackendIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (platform, rest) = s
            .split_once(':')
            .ok_or(BackendIdParseError::MissingPlatform)?;

        return match platform {
            "fluxer" => {
                let (snowflake, instance_name) = rest
                    .split_once(':')
                    .ok_or(BackendIdParseError::MissingInstanceName)?;

                Ok(Self::Fluxer {
                    snowflake: snowflake.parse()?,
                    instance_name: instance_name.to_owned(),
                })
            }
            other => Err(BackendIdParseError::UnknownPlatform(other.to_owned())),
        };
    }
}

impl ValueType for PlatformId {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        return match v {
            Value::String(Some(value)) => value.parse().map_err(|_| ValueTypeErr),

            Value::Char(Some(value)) => value.to_string().parse().map_err(|_| ValueTypeErr),

            _ => Err(ValueTypeErr),
        };
    }

    fn type_name() -> String {
        return "BackendId".to_owned();
    }

    fn array_type() -> ArrayType {
        return ArrayType::String;
    }

    fn column_type() -> ColumnType {
        return ColumnType::String(StringLen::None);
    }
}

impl TryGetable for PlatformId {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index)?;
        return value
            .parse()
            .map_err(|err| DbErr::Type(format!("invalid BackendId: {err}")))
            .map_err(TryGetError::DbErr);
    }
}

impl From<PlatformId> for Value {
    fn from(value: PlatformId) -> Self {
        return Value::String(Some(value.to_string()));
    }
}

impl TryFromU64 for PlatformId {
    fn try_from_u64(_: u64) -> Result<Self, DbErr> {
        return Err(DbErr::ConvertFromU64(
            "BackendId cannot be converted from u64.",
        ));
    }
}

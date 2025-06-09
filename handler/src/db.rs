use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use sqlx::{
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres,
};
use twilight_model::id::Id;

#[repr(transparent)]
// derive what we can from twilight_model::id::Id
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DbId<T>(pub Id<T>);

impl<T> Deref for DbId<T> {
    type Target = Id<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Display for DbId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// sqlx traits
impl<T> Encode<'_, Postgres> for DbId<T> {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, BoxDynError> {
        let val = self.0.get() as i64;
        Encode::<Postgres>::encode_by_ref(&val, buf)
    }
}

impl<T> Decode<'_, Postgres> for DbId<T> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let decoded: i64 = Decode::<Postgres>::decode(value)?;
        Ok(Self(Id::<T>::new(decoded as u64)))
    }
}

impl<T> sqlx::Type<Postgres> for DbId<T> {
    fn type_info() -> PgTypeInfo {
        <i64 as sqlx::Type<Postgres>>::type_info()
    }
}

impl<T> PgHasArrayType for DbId<T> {
    fn array_type_info() -> PgTypeInfo {
        <i64 as PgHasArrayType>::array_type_info()
    }
}

impl<T> From<i64> for DbId<T> {
    fn from(value: i64) -> Self {
        Self(Id::<T>::new(value as u64))
    }
}
impl<T> From<DbId<T>> for i64 {
    fn from(value: DbId<T>) -> Self {
        value.0.get() as Self
    }
}

mod tests {
    assert_impl_all!(DbId<GenericMarker>:
        Clone, Copy, Debug, Deserialize<'static>, Display, Eq, From<NonZeroU64>,
        FromStr, Hash, Into<NonZeroU64>, Into<u64>, Ord, PartialEq, PartialEq<i64>, PartialEq<u64>, PartialOrd, Send, Serialize, Sync,
        TryFrom<i64>, TryFrom<u64>
    );
}

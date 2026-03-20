use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "primes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub prime_index: i64,
    #[sea_orm(unique)]
    pub value: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

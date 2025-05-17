use entity::permissions;
use sea_orm_migration::{
    sea_orm::{ActiveValue::Set, DeriveMigrationName, EntityTrait},
    DbErr, MigrationTrait,
};
use serde::Deserialize;
use tokio::fs;

#[derive(DeriveMigrationName)]
pub struct PermissionSeeder;

#[derive(Deserialize)]
pub struct PermissionSeed {
    name: String,
    description: Option<String>,
}

#[async_trait::async_trait]
impl MigrationTrait for PermissionSeeder {
    async fn up(
        &self,
        manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        let permission_seeds = fs::read_to_string("migration/data/permissions.json")
            .await
            .map_err(|err| DbErr::Custom(format!("Cannot read permissions.json file: {}", err)))?;

        let permission_seeds: Vec<PermissionSeed> = serde_json::from_str(&permission_seeds)
            .map_err(|err| {
                DbErr::Custom(format!("Unable to deserialize permissions json: {}", err))
            })?;

        let mut active_models: Vec<permissions::ActiveModel> =
            Vec::with_capacity(permission_seeds.len());

        for permission_seed in permission_seeds {
            active_models.push(permissions::ActiveModel {
                name: Set(permission_seed.name),
                description: Set(permission_seed.description),
                ..Default::default()
            });
        }

        let db = manager.get_connection();

        permissions::Entity::insert_many(active_models)
            .exec(db)
            .await?;

        Ok(())
    }

    async fn down(
        &self,
        _manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        Ok(())
    }
}

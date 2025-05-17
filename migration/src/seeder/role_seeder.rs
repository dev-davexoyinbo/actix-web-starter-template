use entity::roles;
use sea_orm_migration::{
    sea_orm::{ActiveValue::Set, DeriveMigrationName, EntityTrait},
    DbErr, MigrationTrait,
};
use serde::Deserialize;
use tokio::fs;

#[derive(DeriveMigrationName)]
pub struct RoleSeeder;

#[derive(Deserialize)]
pub struct RoleSeed {
    name: String,
    description: Option<String>,
}

#[async_trait::async_trait]
impl MigrationTrait for RoleSeeder {
    async fn up(
        &self,
        manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        let role_seeds = fs::read_to_string("migration/data/roles.json")
            .await
            .map_err(|err| DbErr::Custom(format!("Cannot read roles.json file: {}", err)))?;

        let role_seeds: Vec<RoleSeed> = serde_json::from_str(&role_seeds)
            .map_err(|err| DbErr::Custom(format!("Unable to deserialize roles json: {}", err)))?;

        let mut active_models: Vec<roles::ActiveModel> = Vec::with_capacity(role_seeds.len());

        for role_seed in role_seeds {
            active_models.push(roles::ActiveModel {
                name: Set(role_seed.name),
                description: Set(role_seed.description),
                ..Default::default()
            });
        }

        let db = manager.get_connection();

        roles::Entity::insert_many(active_models).exec(db).await?;

        Ok(())
    }

    async fn down(
        &self,
        _manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        Ok(())
    }
}

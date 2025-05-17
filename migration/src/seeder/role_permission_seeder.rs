use entity::{permissions, role_permissions, roles};
use sea_orm_migration::{
    sea_orm::{ActiveValue::Set, ColumnTrait, DeriveMigrationName, EntityTrait, QueryFilter},
    DbErr, MigrationTrait,
};
use serde::Deserialize;
use tokio::fs;

#[derive(DeriveMigrationName)]
pub struct RolePermissionSeeder;

#[derive(Deserialize)]
pub struct RolePermissionSeed {
    role: String,
    permissions: Vec<String>,
}

#[async_trait::async_trait]
impl MigrationTrait for RolePermissionSeeder {
    async fn up(
        &self,
        manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        let role_permission_seeds = fs::read_to_string("migration/data/role_permission.json")
            .await
            .map_err(|err| {
                DbErr::Custom(format!("Cannot read role_permissions.json file: {}", err))
            })?;

        let role_permission_seeds: Vec<RolePermissionSeed> =
            serde_json::from_str(&role_permission_seeds).map_err(|err| {
                DbErr::Custom(format!(
                    "Unable to deserialize role permissions json: {}",
                    err
                ))
            })?;

        let mut active_models: Vec<role_permissions::ActiveModel> =
            Vec::with_capacity(role_permission_seeds.len());

        let db = manager.get_connection();

        for role_permission_seed in role_permission_seeds {
            let role = roles::Entity::find()
                .filter(roles::Column::Name.eq(&role_permission_seed.role))
                .one(db)
                .await?
                .ok_or(DbErr::Custom(format!(
                    "Role '{}' not found",
                    role_permission_seed.role
                )))?;

            for permission_name in role_permission_seed.permissions {
                let permission = permissions::Entity::find()
                    .filter(permissions::Column::Name.eq(&permission_name))
                    .one(db)
                    .await?
                    .ok_or(DbErr::Custom(format!(
                        "Permission '{}' not found",
                        permission_name
                    )))?;

                active_models.push(role_permissions::ActiveModel {
                    role_id: Set(role.id),
                    permission_id: Set(permission.id),
                    ..Default::default()
                });
            }
        }

        role_permissions::Entity::insert_many(active_models)
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

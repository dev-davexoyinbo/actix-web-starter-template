use sea_orm_migration::{sea_orm::DeriveMigrationName, MigrationTrait};

#[derive(DeriveMigrationName)]
pub struct PermissionSeeder;

#[async_trait::async_trait]
impl MigrationTrait for PermissionSeeder {
    async fn up(
        &self,
        manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        println!("Running the permission seeder");

        // Here you would typically insert permissions into your database
        // For example:
        // permissions::Entity::insert_many(permissions).exec(manager.get_connection()).await?;

        Ok(())
    }
}

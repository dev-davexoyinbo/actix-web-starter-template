use permission_seeder::PermissionSeeder;
use role_permission_seeder::RolePermissionSeeder;
use role_seeder::RoleSeeder;
use sea_orm_migration::{MigrationName, MigrationTrait};
use user_seeder::UserSeeder;

mod permission_seeder;
mod role_permission_seeder;
mod role_seeder;
mod user_seeder;

pub struct Seeder;

impl Seeder {
    fn seeders(&self) -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(RoleSeeder),
            Box::new(PermissionSeeder),
            Box::new(RolePermissionSeeder),
            Box::new(UserSeeder),
        ]
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Seeder {
    async fn up(
        &self,
        manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        for seeder in self.seeders() {
            seeder.up(manager).await?;
        }

        Ok(())
    }

    async fn down(
        &self,
        _manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        Ok(())
    }
}

impl MigrationName for Seeder {
    fn name(&self) -> &str {
        "seeder-migration"
    }
}

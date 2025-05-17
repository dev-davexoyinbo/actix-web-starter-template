use role_seeder::RoleSeeder;
use sea_orm_migration::{MigrationName, MigrationTrait};

mod role_seeder;

pub struct Seeder;

impl Seeder {
    fn seeders(&self) -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(RoleSeeder)]
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
}

impl MigrationName for Seeder {
    fn name(&self) -> &str {
        "seeder-migration"
    }
}

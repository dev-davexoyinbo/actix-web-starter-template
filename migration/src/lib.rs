pub use sea_orm_migration::prelude::*;
use seeder::Seeder;

mod m20250324_002438_create_users_table;
mod m20250329_214625_create_roles_table;
mod m20250329_214907_create_permissions_table;
mod m20250329_215624_create_role_permissions_table;
mod m20250413_192405_create_auth_token_table;
mod m20250517_114647_create_user_role_table;
mod m20250517_115636_create_user_permission_table;
mod seeder;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250324_002438_create_users_table::Migration),
            Box::new(m20250329_214625_create_roles_table::Migration),
            Box::new(m20250329_214907_create_permissions_table::Migration),
            Box::new(m20250329_215624_create_role_permissions_table::Migration),
            Box::new(m20250413_192405_create_auth_token_table::Migration),
            Box::new(m20250517_114647_create_user_role_table::Migration),
            Box::new(m20250517_115636_create_user_permission_table::Migration),
            Box::new(Seeder),
        ]
    }
}

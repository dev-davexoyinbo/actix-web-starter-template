use common::{AppConfig, PasswordUtils};
use entity::{permissions, roles, user_permission, user_role, users};
use sea_orm_migration::{
    sea_orm::{
        sqlx::types::chrono::Utc, ActiveValue::Set, ColumnTrait, DeriveMigrationName, EntityTrait,
        QueryFilter, TransactionTrait, Statement, DatabaseBackend, ConnectionTrait,
    },
    DbErr, MigrationTrait,
};
use serde::Deserialize;
use tokio::fs;

#[derive(DeriveMigrationName)]
pub struct UserSeeder;

#[derive(Deserialize)]
pub struct UserSeed {
    id: i64,
    name: String,
    email: String,
    roles: Vec<String>,
    permissions: Vec<String>,
    password: Option<String>,
    password_env: Option<String>,
}

#[async_trait::async_trait]
impl MigrationTrait for UserSeeder {
    async fn up(
        &self,
        manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        let db = manager.get_connection();
        let config = AppConfig::initialize()
            .await
            .map_err(|err| DbErr::Custom(format!("Error initializing app config: {}", err)))?;

        let password_utils = PasswordUtils::initialize(config.password.secret)
            .map_err(|err| DbErr::Custom(format!("Error initializing password utils: {}", err)))?;

        let user_seeds = fs::read_to_string("migration/data/users.json")
            .await
            .map_err(|err| DbErr::Custom(format!("Cannot read users.json file: {}", err)))?;

        let user_seeds: Vec<UserSeed> = serde_json::from_str(&user_seeds)
            .map_err(|err| DbErr::Custom(format!("Unable to deserialize users json: {}", err)))?;

        let mut active_models: Vec<users::ActiveModel> = Vec::with_capacity(user_seeds.len());

        let mut user_role_active_models: Vec<user_role::ActiveModel> = Vec::new();
        let mut user_permission_active_models: Vec<user_permission::ActiveModel> = Vec::new();

        let all_roles = user_seeds
            .iter()
            .flat_map(|user_seed| user_seed.roles.clone())
            .collect::<Vec<String>>();

        let all_permissions = user_seeds
            .iter()
            .flat_map(|user_seed| user_seed.permissions.clone())
            .collect::<Vec<String>>();

        let db_roles = roles::Entity::find()
            .filter(roles::Column::Name.is_in(all_roles))
            .all(db)
            .await
            .map_err(|err| DbErr::Custom(format!("Error fetching roles: {}", err)))?;

        let db_permissions = permissions::Entity::find()
            .filter(permissions::Column::Name.is_in(all_permissions))
            .all(db)
            .await
            .map_err(|err| DbErr::Custom(format!("Error fetching permissions: {}", err)))?;

        for user_seed in user_seeds {
            let password = if let Some(env_var) = user_seed.password_env {
                std::env::var(&env_var).map_err(|err| {
                    DbErr::Custom(format!(
                        "Failed to read password from env '{}': {}",
                        env_var, err
                    ))
                })?
            } else {
                user_seed.password.unwrap_or_default()
            };

            let password = password_utils
                .hash_password(&password)
                .map_err(|err| DbErr::Custom(format!("Error hashing password: {err}")))?;

            active_models.push(users::ActiveModel {
                id: Set(user_seed.id),
                name: Set(user_seed.name),
                email: Set(user_seed.email),
                password: Set(password),
                email_verified_at: Set(Some(Utc::now().into())),
                ..Default::default()
            });

            for role in user_seed.roles {
                let db_role = db_roles
                    .iter()
                    .find(|db_role| db_role.name == role)
                    .ok_or_else(|| {
                        DbErr::Custom(format!("Role '{}' not found in database", role))
                    })?;

                user_role_active_models.push(user_role::ActiveModel {
                    user_id: Set(user_seed.id),
                    role_id: Set(db_role.id),
                });
            }

            for permission in user_seed.permissions {
                let db_permission = db_permissions
                    .iter()
                    .find(|db_permission| db_permission.name == permission)
                    .ok_or_else(|| {
                        DbErr::Custom(format!("Permission '{}' not found in database", permission))
                    })?;

                user_permission_active_models.push(user_permission::ActiveModel {
                    user_id: Set(user_seed.id),
                    permission_id: Set(db_permission.id),
                });
            }
        }

        let tx = db.begin().await?;

        users::Entity::insert_many(active_models).exec(&tx).await?;
        user_role::Entity::insert_many(user_role_active_models)
            .exec(&tx)
            .await?;
        user_permission::Entity::insert_many(user_permission_active_models)
            .exec(&tx)
            .await?;

        // Reset sequences after inserting seeded data
        Self::reset_sequences(&tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn down(
        &self,
        _manager: &sea_orm_migration::SchemaManager,
    ) -> Result<(), sea_orm_migration::DbErr> {
        Ok(())
    }
}

impl UserSeeder {    /// Reset database sequences to prevent primary key conflicts
    async fn reset_sequences(
        db: &sea_orm_migration::sea_orm::DatabaseTransaction,
    ) -> Result<(), DbErr> {
        // List of sequences to reset
        let sequences = vec![
            ("users_id_seq", "users"),
            ("roles_id_seq", "roles"),
            ("permissions_id_seq", "permissions"),
            ("auth_tokens_id_seq", "auth_tokens"),
        ];

        for (sequence_name, table_name) in sequences {
            let sql = format!(
                "SELECT setval('{}', COALESCE((SELECT MAX(id) FROM {}), 1))",
                sequence_name, table_name
            );

            let statement = Statement::from_string(DatabaseBackend::Postgres, sql);
            
            db.execute(statement)
                .await
                .map_err(|err| {
                    DbErr::Custom(format!(
                        "Failed to reset sequence '{}': {}",
                        sequence_name, err
                    ))
                })?;
        }

        println!("Database sequences reset successfully");
        Ok(())
    }
}

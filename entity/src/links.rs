use sea_orm::{Linked, RelationTrait};

use crate::{permissions, role_permissions, user_role, users};

pub struct UserRoleToPermission;

impl Linked for UserRoleToPermission {
    type FromEntity = user_role::Entity;
    type ToEntity = permissions::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            user_role::Relation::Roles.def(),
            role_permissions::Relation::Roles.def().rev(),
            role_permissions::Relation::Permissions.def(),
        ]
    }
}

pub struct UserToPermissionsOfRole;

impl Linked for UserToPermissionsOfRole {
    type FromEntity = users::Entity;
    type ToEntity = permissions::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            users::Relation::UserRole.def(),
            user_role::Relation::Roles.def(),
            role_permissions::Relation::Roles.def().rev(),
            role_permissions::Relation::Permissions.def(),
        ]
    }
}

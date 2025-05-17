use sea_orm::{Linked, RelationTrait};

use crate::{role_permissions, roles, user_role};

pub struct UserRoleToPermission;

impl Linked for UserRoleToPermission {
    type FromEntity = user_role::Entity;
    type ToEntity = user_role::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            user_role::Relation::Roles.def(),
            roles::Relation::RolePermissions.def(),
            role_permissions::Relation::Permissions.def(),
        ]
    }
}

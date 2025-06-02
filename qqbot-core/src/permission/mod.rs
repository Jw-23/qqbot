use crate::config::APPCONFIG;

pub fn check_permission(user_id: i64) -> bool {
    APPCONFIG.admins.contains(&user_id)
}

use std::sync::LazyLock;

use crate::database::User;

pub const SYS_USER: LazyLock<User> = LazyLock::new(|| User {
    id: "system".to_string(),
    name: "System".to_string(),
    is_system: true,
});

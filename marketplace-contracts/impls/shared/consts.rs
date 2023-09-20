/// SPDX-License-Identifier: MIT
use openbrush::contracts::access_control::RoleType;

/// The role type of the admin.
pub const ADMIN: RoleType = ink::selector_id!("ADMIN");

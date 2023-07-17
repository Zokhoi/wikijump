/*
 * services/interaction/mod.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! A service to manage "interactions", which are relationships between entities.
//!
//! An "interaction" is a pair of two IDs and the interaction type. This is a flexible
//! system first designed by bluesoul to describe a large number of relations between objects.
//! For instance, instead of creating a separate table for user blocks, we can instead define
//! a number of relations using the interaction system.
//!
//! * `user` / `block` / `user` &mdash; User blocks
//! * `user` / `watch` / `user` &mdash; Following a user
//! * `user` / `watch` / `page` &mdash; Watching a page
//! * `user` / `star` / `page` &mdash; Starring a page

mod prelude {
    pub use super::super::prelude::*;
    pub use super::structs::*;
    pub use crate::models::sea_orm_active_enums::{
        InteractionObjectType, InteractionType,
    };
}

mod service;
mod structs;

pub use self::service::InteractionService;
pub use self::structs::*;

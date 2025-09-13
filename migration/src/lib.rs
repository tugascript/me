// Copyright (c) 2025 Afonso Barracha
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use sea_orm_migration::prelude::*;

mod m20250908_030112_create_users;
mod m20250909_050520_create_portfolio;
mod m20250911_185836_create_cv;
mod m20250911_205500_create_blog;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250908_030112_create_users::Migration),
            Box::new(m20250909_050520_create_portfolio::Migration),
            Box::new(m20250911_185836_create_cv::Migration),
            Box::new(m20250911_205500_create_blog::Migration),
        ]
    }
}

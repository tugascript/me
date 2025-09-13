// Copyright (c) 2025 Afonso Barracha
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string_len(200).not_null())
                    .col(ColumnDef::new(User::Email).string_len(250).not_null())
                    .col(
                        ColumnDef::new(User::Version)
                            .integer()
                            .default(1)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::IsStaff)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::IsAdmin)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthProvider::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthProvider::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AuthProvider::Name)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(ColumnDef::new(AuthProvider::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(AuthProvider::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthProvider::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("auth_provider_user_id_fk")
                            .from(AuthProvider::Table, AuthProvider::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("auth_provider_name_user_id_uidx")
                    .table(AuthProvider::Table)
                    .col(AuthProvider::Name)
                    .col(AuthProvider::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("auth_provider_user_id_idx")
                    .table(AuthProvider::Table)
                    .col(AuthProvider::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserSuspension::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSuspension::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserSuspension::SuspendedUntil)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSuspension::SuspendedReason)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSuspension::Version)
                            .integer()
                            .default(1)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSuspension::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSuspension::IsBanned)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserSuspension::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(UserSuspension::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSuspension::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_suspension_user_id_fk")
                            .from(UserSuspension::Table, UserSuspension::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .if_not_exists()
                            .name("user_suspension_user_id_uidx")
                            .unique()
                            .col(UserSuspension::UserId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(UserSuspension::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(AuthProvider::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Version,
    IsActive,
    IsStaff,
    IsAdmin,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AuthProvider {
    Table,
    Id,
    Name,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserSuspension {
    Table,
    Id,
    SuspendedUntil,
    SuspendedReason,
    Version,
    IsActive,
    IsBanned,
    UserId,
    CreatedAt,
    UpdatedAt,
}

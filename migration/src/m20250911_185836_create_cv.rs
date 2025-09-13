// Copyright (c) 2025 Afonso Barracha
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    sea_orm::{EnumIter, Iterable},
};

use super::{m20250908_030112_create_users::User, m20250909_050520_create_portfolio::Technology};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(CodeSourceEnum)
                    .values(CodeSource::iter())
                    .to_owned(),
            )
            .await?;

        // Create summaries table
        manager
            .create_table(
                Table::create()
                    .table(Summary::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Summary::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Summary::Summary).text().not_null())
                    .col(ColumnDef::new(Summary::CreatedById).integer().not_null())
                    .col(ColumnDef::new(Summary::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Summary::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("summary_created_by_fk")
                            .from(Summary::Table, Summary::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for summaries table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("summary_created_by_idx")
                    .table(Summary::Table)
                    .col(Summary::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create experiences table
        manager
            .create_table(
                Table::create()
                    .table(Experience::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Experience::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Experience::Company)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Experience::Location)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Experience::Title).string_len(200).not_null())
                    .col(ColumnDef::new(Experience::StartDate).date().not_null())
                    .col(ColumnDef::new(Experience::EndDate).date().null())
                    .col(ColumnDef::new(Experience::Achievements).text().null())
                    .col(
                        ColumnDef::new(Experience::IsCurrent)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Experience::CreatedById).integer().not_null())
                    .col(ColumnDef::new(Experience::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Experience::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("experience_created_by_fk")
                            .from(Experience::Table, Experience::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for experiences table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("experience_created_by_idx")
                    .table(Experience::Table)
                    .col(Experience::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create educations table
        manager
            .create_table(
                Table::create()
                    .table(Education::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Education::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Education::Institution)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Education::Location)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Education::Degree).string_len(200).not_null())
                    .col(ColumnDef::new(Education::StartDate).date().not_null())
                    .col(ColumnDef::new(Education::EndDate).date().null())
                    .col(ColumnDef::new(Education::Grade).string_len(50).null())
                    .col(ColumnDef::new(Education::Description).text().null())
                    .col(ColumnDef::new(Education::CreatedById).integer().not_null())
                    .col(ColumnDef::new(Education::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Education::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("education_created_by_fk")
                            .from(Education::Table, Education::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for educations table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("education_created_by_idx")
                    .table(Education::Table)
                    .col(Education::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create certificates table
        manager
            .create_table(
                Table::create()
                    .table(Certificate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Certificate::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Certificate::Title)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Certificate::Issuer)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Certificate::IssueDate).date().not_null())
                    .col(ColumnDef::new(Certificate::ExpirationDate).date().null())
                    .col(
                        ColumnDef::new(Certificate::VerificationCode)
                            .string_len(100)
                            .null(),
                    )
                    .col(ColumnDef::new(Certificate::Description).text().null())
                    .col(
                        ColumnDef::new(Certificate::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Certificate::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Certificate::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("certificate_created_by_fk")
                            .from(Certificate::Table, Certificate::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for certificates table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("certificate_created_by_idx")
                    .table(Certificate::Table)
                    .col(Certificate::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create skills table
        manager
            .create_table(
                Table::create()
                    .table(Skill::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Skill::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Skill::TechnologyId).integer().not_null())
                    .col(ColumnDef::new(Skill::CreatedById).integer().not_null())
                    .col(ColumnDef::new(Skill::InitialUsageDate).date().not_null())
                    .col(ColumnDef::new(Skill::LastUsageDate).date().null())
                    .col(
                        ColumnDef::new(Skill::IsCurrentlyUsed)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Skill::Description).text().null())
                    .col(ColumnDef::new(Skill::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Skill::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("skill_created_by_fk")
                            .from(Skill::Table, Skill::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("skill_technology_id_fk")
                            .from(Skill::Table, Skill::TechnologyId)
                            .to(Technology::Table, Technology::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for skills table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("skill_created_by_idx")
                    .table(Skill::Table)
                    .col(Skill::CreatedById)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("skill_technology_id_idx")
                    .table(Skill::Table)
                    .col(Skill::TechnologyId)
                    .to_owned(),
            )
            .await?;

        // Create open_source_contributions table
        manager
            .create_table(
                Table::create()
                    .table(OpenSourceContribution::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OpenSourceContribution::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::Url)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::Name)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::Source)
                            .enumeration(CodeSourceEnum, CodeSource::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OpenSourceContribution::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("open_source_contribution_created_by_fk")
                            .from(
                                OpenSourceContribution::Table,
                                OpenSourceContribution::CreatedById,
                            )
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for open_source_contributions table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("open_source_contribution_created_by_idx")
                    .table(OpenSourceContribution::Table)
                    .col(OpenSourceContribution::CreatedById)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to handle foreign key constraints
        manager
            .drop_table(
                Table::drop()
                    .table(OpenSourceContribution::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Skill::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Certificate::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Education::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Experience::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Summary::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(CodeSourceEnum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "summaries")]
enum Summary {
    Table,
    Id,
    Summary,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "experiences")]
enum Experience {
    Table,
    Id,
    Company,
    Location,
    Title,
    StartDate,
    EndDate,
    Achievements,
    IsCurrent,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "educations")]
enum Education {
    Table,
    Id,
    Institution,
    Location,
    Degree,
    StartDate,
    EndDate,
    Grade,
    Description,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "certificates")]
enum Certificate {
    Table,
    Id,
    Title,
    Issuer,
    IssueDate,
    ExpirationDate,
    VerificationCode,
    Description,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "skills")]
enum Skill {
    Table,
    Id,
    TechnologyId,
    CreatedById,
    InitialUsageDate,
    LastUsageDate,
    IsCurrentlyUsed,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub struct CodeSourceEnum;

#[derive(Iden, EnumIter)]
pub enum CodeSource {
    #[iden = "github"]
    Github,
    #[iden = "gitlab"]
    Gitlab,
    #[iden = "crates_io"]
    CratesIo,
    #[iden = "npm"]
    Npm,
    #[iden = "pypi"]
    Pypi,
    #[iden = "other"]
    Other,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "open_source_contributions")]
enum OpenSourceContribution {
    Table,
    Id,
    Url,
    Name,
    Source,
    Description,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

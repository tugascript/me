// Copyright (c) 2025 Afonso Barracha
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use sea_orm_migration::{
    prelude::extension::postgres::Type,
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

use super::m20250908_030112_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TechnologyTypeEnum)
                    .values(TechnologyType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectStatusEnum)
                    .values(ProjectStatus::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectLinkTypeEnum)
                    .values(ProjectLinkType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Technology::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Technology::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Technology::Name).string_len(200).not_null())
                    .col(ColumnDef::new(Technology::Icon).text().not_null())
                    .col(
                        ColumnDef::new(Technology::TechType)
                            .enumeration(TechnologyTypeEnum, TechnologyType::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Technology::Description).text().null())
                    .col(ColumnDef::new(Technology::CreatedById).integer().not_null())
                    .col(ColumnDef::new(Technology::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Technology::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("technology_created_by_fk")
                            .from(Technology::Table, Technology::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RelatedTechnology::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelatedTechnology::ParentTechnologyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelatedTechnology::ChildTechnologyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelatedTechnology::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name(
                                "related_technologies_parent_technology_id_child_technology_id_pk",
                            )
                            .col(RelatedTechnology::ParentTechnologyId)
                            .col(RelatedTechnology::ChildTechnologyId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("related_technologies_parent_technology_id_fk")
                            .from(
                                RelatedTechnology::Table,
                                RelatedTechnology::ParentTechnologyId,
                            )
                            .to(Technology::Table, Technology::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("related_technologies_child_technology_id_fk")
                            .from(
                                RelatedTechnology::Table,
                                RelatedTechnology::ChildTechnologyId,
                            )
                            .to(Technology::Table, Technology::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Title).string_len(200).not_null())
                    .col(ColumnDef::new(Project::Subtitle).string_len(300).null())
                    .col(ColumnDef::new(Project::Description).text().not_null())
                    .col(
                        ColumnDef::new(Project::Status)
                            .enumeration(ProjectStatusEnum, ProjectStatus::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Project::StartDate).date().not_null())
                    .col(ColumnDef::new(Project::EndDate).date().null())
                    .col(ColumnDef::new(Project::DropDate).date().null())
                    .col(ColumnDef::new(Project::CreatedById).integer().not_null())
                    .col(ColumnDef::new(Project::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Project::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_created_by_fk")
                            .from(Project::Table, Project::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectTechnology::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectTechnology::ProjectId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectTechnology::TechnologyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectTechnology::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("project_technology_project_id_technology_id_pk")
                            .col(ProjectTechnology::ProjectId)
                            .col(ProjectTechnology::TechnologyId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_technology_project_id_fk")
                            .from(ProjectTechnology::Table, ProjectTechnology::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_technology_technology_id_fk")
                            .from(ProjectTechnology::Table, ProjectTechnology::TechnologyId)
                            .to(Technology::Table, Technology::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectLink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectLink::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProjectLink::ProjectId).integer().not_null())
                    .col(
                        ColumnDef::new(ProjectLink::LinkUrl)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectLink::LinkType)
                            .enumeration(ProjectLinkTypeEnum, ProjectLinkType::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectLink::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectLink::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectLink::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_link_project_id_fk")
                            .from(ProjectLink::Table, ProjectLink::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_link_created_by_fk")
                            .from(ProjectLink::Table, ProjectLink::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectImage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectImage::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProjectImage::ProjectId).integer().not_null())
                    .col(
                        ColumnDef::new(ProjectImage::ImageUrl)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectImage::ImageType)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectImage::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectImage::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectImage::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_image_project_id_fk")
                            .from(ProjectImage::Table, ProjectImage::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_image_created_by_fk")
                            .from(ProjectImage::Table, ProjectImage::CreatedById)
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
                    .name("project_image_project_id_idx")
                    .table(ProjectImage::Table)
                    .col(ProjectImage::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("project_image_created_by_idx")
                    .table(ProjectImage::Table)
                    .col(ProjectImage::CreatedById)
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
                    .table(ProjectTechnology::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(ProjectLink::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(ProjectImage::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Project::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(RelatedTechnology::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Technology::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(TechnologyTypeEnum).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(ProjectStatusEnum).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(ProjectLinkTypeEnum)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
struct TechnologyTypeEnum;

#[derive(Iden, EnumIter)]
pub enum TechnologyType {
    #[iden = "language"]
    Language,
    #[iden = "framework"]
    Framework,
    #[iden = "database"]
    Database,
    #[iden = "platform"]
    Platform,
    #[iden = "tool"]
    Tool,
    #[iden = "other"]
    Other,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "technologies")]
pub enum Technology {
    Table,
    Id,
    Name,
    Icon,
    TechType,
    Description,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "related_technologies")]
enum RelatedTechnology {
    Table,
    ParentTechnologyId,
    ChildTechnologyId,
    CreatedAt,
}

#[derive(DeriveIden)]
struct ProjectStatusEnum;

#[derive(Iden, EnumIter)]
pub enum ProjectStatus {
    #[iden = "ongoing"]
    Ongoing,
    #[iden = "completed"]
    Completed,
    #[iden = "maintenance"]
    Maintenance,
    #[iden = "archived"]
    Archived,
    #[iden = "on_hold"]
    OnHold,
    #[iden = "dropped"]
    Dropped,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "projects")]
pub enum Project {
    Table,
    Id,
    Title,
    Subtitle,
    Description,
    Status,
    StartDate,
    EndDate,
    DropDate,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "project_technologies")]
enum ProjectTechnology {
    Table,
    ProjectId,
    TechnologyId,
    CreatedAt,
}

#[derive(DeriveIden)]
struct ProjectLinkTypeEnum;

#[derive(Iden, EnumIter)]
pub enum ProjectLinkType {
    #[iden = "website"]
    Website,
    #[iden = "github"]
    Github,
    #[iden = "gitlab"]
    Gitlab,
    #[iden = "dev_to"]
    DevTo,
    #[iden = "linkedin"]
    Linkedin,
    #[iden = "youtube"]
    Youtube,
    #[iden = "other"]
    Other,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "project_links")]
enum ProjectLink {
    Table,
    Id,
    ProjectId,
    LinkUrl,
    LinkType,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "project_images")]
enum ProjectImage {
    Table,
    Id,
    ProjectId,
    ImageUrl,
    ImageType,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

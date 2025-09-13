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
                    .as_enum(BlogReactionsEnum)
                    .values(BlogReactions::iter())
                    .to_owned(),
            )
            .await?;

        // Create common_tags table
        manager
            .create_table(
                Table::create()
                    .table(CommonTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommonTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CommonTag::Name).string_len(100).not_null())
                    .col(ColumnDef::new(CommonTag::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(CommonTag::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create unique index for common_tags table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("common_tag_name_uidx")
                    .table(CommonTag::Table)
                    .col(CommonTag::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create blog_posts table
        manager
            .create_table(
                Table::create()
                    .table(BlogPost::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPost::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BlogPost::Title).string_len(200).not_null())
                    .col(ColumnDef::new(BlogPost::Content).text().not_null())
                    .col(ColumnDef::new(BlogPost::CreatedById).integer().not_null())
                    .col(
                        ColumnDef::new(BlogPost::CoverImageUrl)
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(BlogPost::FirstPublishedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(ColumnDef::new(BlogPost::PublishedAt).timestamp().null())
                    .col(
                        ColumnDef::new(BlogPost::IsPublished)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPost::IsFeatured)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(BlogPost::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(BlogPost::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_created_by_fk")
                            .from(BlogPost::Table, BlogPost::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for blog_posts table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_created_by_idx")
                    .table(BlogPost::Table)
                    .col(BlogPost::CreatedById)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_published_at_idx")
                    .table(BlogPost::Table)
                    .col(BlogPost::PublishedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_is_published_idx")
                    .table(BlogPost::Table)
                    .col(BlogPost::IsPublished)
                    .to_owned(),
            )
            .await?;

        // Create blog_post_tags table
        manager
            .create_table(
                Table::create()
                    .table(BlogPostTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPostTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BlogPostTag::BlogPostId).integer().not_null())
                    .col(ColumnDef::new(BlogPostTag::Tag).string_len(100).not_null())
                    .col(ColumnDef::new(BlogPostTag::CommonTagId).integer().null())
                    .col(
                        ColumnDef::new(BlogPostTag::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostTag::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostTag::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_tag_blog_post_id_fk")
                            .from(BlogPostTag::Table, BlogPostTag::BlogPostId)
                            .to(BlogPost::Table, BlogPost::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_tag_common_tag_id_fk")
                            .from(BlogPostTag::Table, BlogPostTag::CommonTagId)
                            .to(CommonTag::Table, CommonTag::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_tag_created_by_fk")
                            .from(BlogPostTag::Table, BlogPostTag::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for blog_post_tags table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_tag_blog_post_id_idx")
                    .table(BlogPostTag::Table)
                    .col(BlogPostTag::BlogPostId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_tag_common_tag_id_idx")
                    .table(BlogPostTag::Table)
                    .col(BlogPostTag::CommonTagId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_tag_created_by_idx")
                    .table(BlogPostTag::Table)
                    .col(BlogPostTag::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create blog_post_images table
        manager
            .create_table(
                Table::create()
                    .table(BlogPostImage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPostImage::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BlogPostImage::BlogPostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostImage::ImageUrl)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostImage::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostImage::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostImage::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_image_blog_post_id_fk")
                            .from(BlogPostImage::Table, BlogPostImage::BlogPostId)
                            .to(BlogPost::Table, BlogPost::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_image_created_by_fk")
                            .from(BlogPostImage::Table, BlogPostImage::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for blog_post_images table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_image_blog_post_id_idx")
                    .table(BlogPostImage::Table)
                    .col(BlogPostImage::BlogPostId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_image_created_by_idx")
                    .table(BlogPostImage::Table)
                    .col(BlogPostImage::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create blog_post_technologies table
        manager
            .create_table(
                Table::create()
                    .table(BlogPostTechnology::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPostTechnology::BlogPostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostTechnology::TechnologyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostTechnology::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("blog_post_technology_pk")
                            .col(BlogPostTechnology::BlogPostId)
                            .col(BlogPostTechnology::TechnologyId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_technology_blog_post_id_fk")
                            .from(BlogPostTechnology::Table, BlogPostTechnology::BlogPostId)
                            .to(BlogPost::Table, BlogPost::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_technology_technology_id_fk")
                            .from(BlogPostTechnology::Table, BlogPostTechnology::TechnologyId)
                            .to(Technology::Table, Technology::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create blog_series table
        manager
            .create_table(
                Table::create()
                    .table(BlogSeries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogSeries::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BlogSeries::Title).string_len(200).not_null())
                    .col(ColumnDef::new(BlogSeries::Subtitle).string_len(300).null())
                    .col(
                        ColumnDef::new(BlogSeries::CoverImageUrl)
                            .string_len(500)
                            .null(),
                    )
                    .col(ColumnDef::new(BlogSeries::CreatedById).integer().not_null())
                    .col(ColumnDef::new(BlogSeries::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(BlogSeries::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_series_created_by_fk")
                            .from(BlogSeries::Table, BlogSeries::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for blog_series table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_series_created_by_idx")
                    .table(BlogSeries::Table)
                    .col(BlogSeries::CreatedById)
                    .to_owned(),
            )
            .await?;

        // Create blog_series_posts table
        manager
            .create_table(
                Table::create()
                    .table(BlogSeriesPost::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogSeriesPost::BlogSeriesId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogSeriesPost::BlogPostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogSeriesPost::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("blog_series_post_pk")
                            .col(BlogSeriesPost::BlogSeriesId)
                            .col(BlogSeriesPost::BlogPostId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_series_post_blog_series_id_fk")
                            .from(BlogSeriesPost::Table, BlogSeriesPost::BlogSeriesId)
                            .to(BlogSeries::Table, BlogSeries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_series_post_blog_post_id_fk")
                            .from(BlogSeriesPost::Table, BlogSeriesPost::BlogPostId)
                            .to(BlogPost::Table, BlogPost::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create blog_series_technologies table
        manager
            .create_table(
                Table::create()
                    .table(BlogSeriesTechnology::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogSeriesTechnology::BlogSeriesId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogSeriesTechnology::TechnologyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogSeriesTechnology::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("blog_series_technology_pk")
                            .col(BlogSeriesTechnology::BlogSeriesId)
                            .col(BlogSeriesTechnology::TechnologyId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_series_technology_blog_series_id_fk")
                            .from(
                                BlogSeriesTechnology::Table,
                                BlogSeriesTechnology::BlogSeriesId,
                            )
                            .to(BlogSeries::Table, BlogSeries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_series_technology_technology_id_fk")
                            .from(
                                BlogSeriesTechnology::Table,
                                BlogSeriesTechnology::TechnologyId,
                            )
                            .to(Technology::Table, Technology::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create blog_post_comments table
        manager
            .create_table(
                Table::create()
                    .table(BlogPostComment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPostComment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BlogPostComment::BlogPostId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(BlogPostComment::Comment).text().not_null())
                    .col(
                        ColumnDef::new(BlogPostComment::IsHidden)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostComment::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostComment::ParentCommentId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostComment::ImageUrl)
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostComment::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostComment::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_comment_blog_post_id_fk")
                            .from(BlogPostComment::Table, BlogPostComment::BlogPostId)
                            .to(BlogPost::Table, BlogPost::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_comment_created_by_fk")
                            .from(BlogPostComment::Table, BlogPostComment::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_comment_parent_comment_id_fk")
                            .from(BlogPostComment::Table, BlogPostComment::ParentCommentId)
                            .to(BlogPostComment::Table, BlogPostComment::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for blog_post_comments table
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_comment_blog_post_id_idx")
                    .table(BlogPostComment::Table)
                    .col(BlogPostComment::BlogPostId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_comment_created_by_idx")
                    .table(BlogPostComment::Table)
                    .col(BlogPostComment::CreatedById)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("blog_post_comment_parent_comment_id_idx")
                    .table(BlogPostComment::Table)
                    .col(BlogPostComment::ParentCommentId)
                    .to_owned(),
            )
            .await?;

        // Create blog_post_reactions table
        manager
            .create_table(
                Table::create()
                    .table(BlogPostReaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPostReaction::BlogPostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostReaction::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostReaction::Reaction)
                            .enumeration(BlogReactionsEnum, BlogReactions::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostReaction::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_reaction_blog_post_id_fk")
                            .from(BlogPostReaction::Table, BlogPostReaction::BlogPostId)
                            .to(BlogPost::Table, BlogPost::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_reaction_created_by_fk")
                            .from(BlogPostReaction::Table, BlogPostReaction::CreatedById)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("blog_post_reaction_pk")
                            .col(BlogPostReaction::BlogPostId)
                            .col(BlogPostReaction::CreatedById)
                            .primary(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create blog_post_comments_reactions table
        manager
            .create_table(
                Table::create()
                    .table(BlogPostCommentReaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlogPostCommentReaction::BlogPostCommentId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostCommentReaction::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostCommentReaction::Reaction)
                            .enumeration(BlogReactionsEnum, BlogReactions::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlogPostCommentReaction::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_comment_reaction_blog_post_comment_id_fk")
                            .from(
                                BlogPostCommentReaction::Table,
                                BlogPostCommentReaction::BlogPostCommentId,
                            )
                            .to(BlogPostComment::Table, BlogPostComment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("blog_post_comment_reaction_created_by_fk")
                            .from(
                                BlogPostCommentReaction::Table,
                                BlogPostCommentReaction::CreatedById,
                            )
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("blog_post_comment_reaction_pk")
                            .col(BlogPostCommentReaction::BlogPostCommentId)
                            .col(BlogPostCommentReaction::CreatedById)
                            .primary(),
                    )
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
                    .if_exists()
                    .table(BlogPostComment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogSeriesTechnology::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogSeriesPost::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogSeries::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogPostTechnology::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogPostImage::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogPostTag::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(BlogPost::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(CommonTag::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogPostCommentReaction::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(BlogPostReaction::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(BlogReactionsEnum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "common_tags")]
enum CommonTag {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_posts")]
enum BlogPost {
    Table,
    Id,
    Title,
    Content,
    CreatedById,
    CoverImageUrl,
    FirstPublishedAt,
    PublishedAt,
    IsPublished,
    IsFeatured,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_post_tags")]
enum BlogPostTag {
    Table,
    Id,
    BlogPostId,
    Tag,
    CommonTagId,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_post_images")]
enum BlogPostImage {
    Table,
    Id,
    BlogPostId,
    ImageUrl,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_post_technologies")]
enum BlogPostTechnology {
    Table,
    BlogPostId,
    TechnologyId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub struct BlogReactionsEnum;

#[derive(Iden, EnumIter)]
pub enum BlogReactions {
    #[iden = "hate"]
    Hate,
    #[iden = "dislike"]
    Dislike,
    #[iden = "ok"]
    Ok,
    #[iden = "like"]
    Like,
    #[iden = "love"]
    Love,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_post_reactions")]
enum BlogPostReaction {
    Table,
    BlogPostId,
    CreatedById,
    Reaction,
    CreatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_series")]
enum BlogSeries {
    Table,
    Id,
    Title,
    Subtitle,
    CoverImageUrl,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_series_posts")]
enum BlogSeriesPost {
    Table,
    BlogSeriesId,
    BlogPostId,
    CreatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_series_technologies")]
enum BlogSeriesTechnology {
    Table,
    BlogSeriesId,
    TechnologyId,
    CreatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_post_comments")]
enum BlogPostComment {
    Table,
    Id,
    BlogPostId,
    Comment,
    ImageUrl,
    ParentCommentId,
    IsHidden,
    CreatedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "blog_post_comments_reactions")]
enum BlogPostCommentReaction {
    Table,
    BlogPostCommentId,
    CreatedById,
    Reaction,
    CreatedAt,
}

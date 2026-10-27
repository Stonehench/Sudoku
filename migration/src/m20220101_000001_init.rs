use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(char_len("user_id", 36).primary_key())
                    .col(string("username"))
                    .col(string("password"))
                    .col(string("passsalt"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("DailyChallenges")
                    .if_not_exists()
                    .col(
                        date("dato")
                            .default(Expr::current_date())
                            .primary_key()
                            .not_null(),
                    )
                    .col(text("puzzle").not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("scores")
                    .if_not_exists()
                    .col(big_pk_auto("score_id").big_unsigned())
                    .col(char_len("user_id", 36).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col("user_id")
                            .to("users", "user_id"),
                    )
                    .col(integer("value"))
                    .col(timestamp("submittet").default(Expr::current_timestamp()).not_null())
                    .col(date("daily_dato").null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col("daily_dato")
                            .to("DailyChallenges", "dato"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("scores").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("DailyChallenges").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await?;

        Ok(())
    }
}

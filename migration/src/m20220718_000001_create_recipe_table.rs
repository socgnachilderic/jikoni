use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220718_000001_create_recipe_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Recipes::table_create_statement();
        manager.create_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Recipes::drop_create_statement();
        manager.drop_table(stmt).await
    }
}

#[derive(Iden)]
pub enum Recipes {
    Table,
    Id,
    Title,
    Description,
    Image,
}

impl Recipes {
    fn table_create_statement() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .col(&mut Self::def_col_id())
            .col(&mut Self::def_col_title())
            .col(&mut Self::def_col_description())
            .col(&mut &mut Self::def_col_image())
            .to_owned()
    }

    fn drop_create_statement() -> TableDropStatement {
        Table::drop().table(Recipes::Table).to_owned()
    }

    fn def_col_id() -> ColumnDef {
        let mut col = ColumnDef::new(Self::Id);
        col.string_len(40).not_null().primary_key();
        col
    }

    fn def_col_title() -> ColumnDef {
        let mut col = ColumnDef::new(Self::Title);
        col.string_len(200).not_null();
        col
    }
    
    fn def_col_description() -> ColumnDef {
        let mut col = ColumnDef::new(Self::Description);
        col.string().null();
        col
    }
    
    fn def_col_image() -> ColumnDef {
        let mut col = ColumnDef::new(Self::Image);
        col.string_len(50).null();
        col
    }
}

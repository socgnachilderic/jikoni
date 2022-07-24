use async_graphql::Result;

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{generate_code, AppCode};

use super::recipe_model;
use super::recipe_schemas::{CreateRecipeInput, Recipe};

pub struct RecipeService<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> RecipeService<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn create_recipe(
        &self,
        recipe: CreateRecipeInput,
        recipe_image: Option<String>,
    ) -> Result<Recipe> {
        let code = generate_code(AppCode::Recipe);
        let recipe_model = recipe_model::ActiveModel {
            id: Set(code),
            title: Set(recipe.title),
            description: Set(recipe.description),
            image: Set(recipe_image),
        };

        let recipe_saved = recipe_model.insert(self.conn).await?;
        let recipe = Recipe::from(recipe_saved);

        Ok(recipe)
    }

    pub async fn get_all_recipes(&self) -> Result<Vec<Recipe>> {
        let recipes = recipe_model::Entity::find()
            .all(self.conn)
            .await?
            .into_iter()
            .map(|recipe| Recipe::from(recipe))
            .collect();

        Ok(recipes)
    }
}

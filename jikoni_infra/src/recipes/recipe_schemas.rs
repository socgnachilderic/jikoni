use async_graphql::{Context, InputObject, Object, Result, SimpleObject, Upload, ID};

use super::{recipe_model, recipe_service::RecipeService};
use crate::{AppState, ImageUtils};

#[derive(Default)]
pub(crate) struct RecipeQuery;

#[Object]
impl RecipeQuery {
    async fn get_recipes(&self, ctx: &Context<'_>) -> Result<Vec<Recipe>> {
        let conn = ctx.data::<AppState>().unwrap().conn.clone();
        let recipe_service = RecipeService::new(&conn);

        recipe_service.get_all_recipes().await
    }
}

#[derive(Default)]
pub(crate) struct RecipeMutation;

#[Object]
impl RecipeMutation {
    async fn create_recipe(
        &self,
        ctx: &Context<'_>,
        input: CreateRecipeInput,
        recipe_image: Upload,
    ) -> Result<Recipe> {
        let conn = ctx.data::<AppState>().unwrap().conn.clone();
        let recipe_service = RecipeService::new(conn.as_ref());
        let upload = recipe_image.value(ctx).unwrap();
        let image_name = ImageUtils::generate_image_name();

        ImageUtils::save_image(upload.content, &image_name)?;
        recipe_service.create_recipe(input, Some(image_name)).await
    }
}

#[derive(Debug, InputObject)]
pub struct CreateRecipeInput {
    pub title: String,
    pub description: Option<String>,
}

#[derive(SimpleObject)]
pub struct Recipe {
    #[graphql(name = "_id")]
    pub id: ID,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

impl From<recipe_model::Model> for Recipe {
    fn from(model: recipe_model::Model) -> Self {
        let image_url = model.image.as_ref().map(|image| format!("/files/images/{}", image));

        Self {
            id: ID(model.id),
            title: model.title,
            description: model.description,
            image_url,
        }
    }
}

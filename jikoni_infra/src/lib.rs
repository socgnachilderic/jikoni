use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::{get, web, Responder};
use image::ImageFormat;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use uuid::Uuid;

use crate::configs::Env;

pub mod configs;
pub mod graphql;

mod recipes;

#[get("/files/{dir}/{name}.{ext}")]
pub async fn get_file(path: web::Path<FilePath>, app_state: web::Data<AppState>) -> impl Responder {
    let file_path = path.into_inner().get_full_path(&app_state.env.media_root);
    NamedFile::open_async(file_path).await
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub env: Arc<Env>,
}

#[derive(Serialize, Deserialize)]
pub struct FilePath {
    dir: String,
    name: String,
    ext: FileExtensions,
}

impl FilePath {
    fn get_full_path(&self, media_root: &str) -> String {
        let extension = to_variant_name(&self.ext).unwrap();
        format!("{}/{}/{}.{}", media_root, self.dir, self.name, extension)
    }
}

pub(crate) struct ImageUtils;

impl ImageUtils {
    fn generate_image_name() -> String {
        format!("IMG_{}.png", generate_code(AppCode::Recipe))
    }

    fn save_image(file: File, image_name: &str) -> image::ImageResult<()> {
        let reader = BufReader::new(file);
        let image_loaded = image::load(reader, ImageFormat::Png)?;
        image_loaded.save(format!("data/images/{}", &image_name))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum FileExtensions {
    Png,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum AppCode {
    #[serde(rename = "REC")]
    Recipe,
}

pub(crate) fn generate_code(app_code: AppCode) -> String {
    let code = to_variant_name(&app_code).unwrap();
    let uuid = Uuid::new_v4().simple();

    format!("{}_{}", code, uuid)
}

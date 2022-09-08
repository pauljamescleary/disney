//! Model for different types in the home screen and related json
//!
//! This JSON is difficult to parse, and should have a custom
//! deserializer for everything.  I would use a streaming
//! deserializer so that we can begin loading the screen
//! as soon as the json arrives and is parsed
//!
//! This relies purely on strongly typed Serde
//! for serialization, which has a lot of warts
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetRef {
    data: SetRefContainer,
}
impl SetRef {
    pub fn content_set(self) -> Option<ContentSet> {
        self.data.content_set()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetRefContainer {
    // TODO: This is a hack, we need to intelligently parse the JSON
    curated_set: Option<ContentSet>,
    trending_set: Option<ContentSet>,
    personalized_curated_set: Option<ContentSet>,
}
impl SetRefContainer {
    pub fn content_set(self) -> Option<ContentSet> {
        self.curated_set
            .or(self.trending_set)
            .or(self.personalized_curated_set)
    }
}

#[derive(Deserialize)]
pub struct HomeRoot {
    data: HomeData,
}
impl HomeRoot {
    pub fn home_screen(self) -> HomeScreen {
        self.data.standard_collection
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeData {
    standard_collection: HomeScreen,
}

#[derive(Deserialize)]
pub struct HomeScreen {
    containers: Vec<Container>,
}
impl HomeScreen {
    pub fn content_sets(self) -> impl Iterator<Item = ContentSet> {
        self.containers.into_iter().map(|c| c.content_set())
    }
}

#[derive(Clone, Deserialize)]
pub struct Container {
    set: ContentSet,
}
impl Container {
    pub fn content_set(self) -> ContentSet {
        self.set
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSet {
    text: ContentSetTitle,
    ref_id: Option<String>, // will be set if we need to load this separately

    #[serde(default)]
    items: Vec<ContentSetItem>, // represents all of the programs in a curated set
}
impl ContentSet {
    pub fn set_title(&self, title: &String) -> ContentSet {
        let mut new = self.clone();
        new.text.title.full.set.default.content = title.clone();
        new
    }
    pub fn title(&self) -> &String {
        &self.text.title.full.set.default.content
    }

    pub fn items(self) -> Vec<ContentSetItem> {
        self.items
    }

    pub fn items_iter(&self) -> impl Iterator<Item = &ContentSetItem> {
        self.items.iter()
    }

    pub fn ref_id(&self) -> Option<&String> {
        self.ref_id.as_ref()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSetItem {
    image: ContentItemTileImage,
}
impl ContentSetItem {
    pub fn tile_image_url(&self, size: &str) -> Option<&String> {
        self.image
            .tile
            .get(size)
            .and_then(|ti| ti.default_image())
            .map(|di| &di.default.url)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContentItemTileImage {
    tile: HashMap<String, TileImage>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileImage {
    series: Option<DefaultImage>,
    program: Option<DefaultImage>,
    default: Option<DefaultImage>,
}
impl TileImage {
    pub fn default_image(&self) -> Option<&DefaultImage> {
        self.series
            .as_ref()
            .or(self.program.as_ref())
            .or(self.default.as_ref())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DefaultImage {
    default: Image,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContentSetTitle {
    title: ContentSetTitleText,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContentSetTitleText {
    full: FullSetText,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FullSetText {
    set: DefaultText,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DefaultText {
    default: TextContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TextContent {
    content: String,
}

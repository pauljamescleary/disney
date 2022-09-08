use bytes::Bytes;

pub struct ImageLoadBatchEvent {
    pub events: Vec<ImageLoadEvent>,
}

pub struct ImageLoadEvent {
    pub img_url: String,
    pub bytes: Bytes,
    pub content_set_title: String,
}

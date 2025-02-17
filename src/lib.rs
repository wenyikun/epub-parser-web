use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use epub::doc::{EpubDoc, NavPoint};
use serde::Serialize;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// 定义一个可序列化的导航点结构体
#[derive(Serialize)]
struct SerializableNavPoint {
    label: String,
    content: String, // 将PathBuf转换为字符串
    children: Vec<SerializableNavPoint>,
    play_order: usize,
}

// 将NavPoint转换为SerializableNavPoint
fn convert_navpoint(nav_point: &NavPoint) -> SerializableNavPoint {
    SerializableNavPoint {
        label: nav_point.label.clone(),
        content: nav_point.content.to_string_lossy().into_owned(),
        children: nav_point.children.iter().map(convert_navpoint).collect(),
        play_order: nav_point.play_order,
    }
}

// 定义一个EpubParser结构体，用于解析EPUB文件
#[wasm_bindgen]
pub struct EpubParser {
    toc: Vec<SerializableNavPoint>, // 目录
    spine: Vec<String>,             // 书脊
    unique_identifier: String,      // 唯一标识符
    doc: EpubDoc<Cursor<Vec<u8>>>,  // EPUB文档
}

#[wasm_bindgen]
impl EpubParser {
    // 构造函数，接收文件字节数组并返回EpubParser实例
    #[wasm_bindgen(constructor)]
    pub fn new(file: &[u8]) -> Result<EpubParser, JsValue> {
        let cursor = Cursor::new(file.to_vec());
        let doc = EpubDoc::from_reader(cursor).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let toc = doc.toc.iter().map(convert_navpoint).collect();
        let spine = doc.spine.clone();
        let unique_identifier = doc.unique_identifier.clone().unwrap_or_else(|| {
            let md5 = md5::compute(file);
            format!("{:x}", md5)
        });
        Ok(EpubParser {
            doc,
            toc,
            spine,
            unique_identifier,
        })
    }

    // 获取书名
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.doc.mdata("title").unwrap_or_default()
    }

    // 获取作者
    #[wasm_bindgen(getter)]
    pub fn author(&self) -> String {
        self.doc.mdata("creator").unwrap_or_default()
    }

    // 获取语言
    #[wasm_bindgen(getter)]
    pub fn language(&self) -> String {
        self.doc.mdata("language").unwrap_or_default()
    }

    // 获取描述
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.doc.mdata("description").unwrap_or_default()
    }

    // 获取封面图像的Base64编码
    #[wasm_bindgen(getter)]
    pub fn cover(&mut self) -> String {
        self.get_cover_base64().unwrap_or_default()
    }

    // 获取目录的JSON表示
    #[wasm_bindgen(getter)]
    pub fn toc(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.toc).unwrap()
    }

    // 获取书脊
    #[wasm_bindgen(getter)]
    pub fn spine(&self) -> Vec<String> {
        self.spine.clone()
    }

    // 获取唯一标识符
    #[wasm_bindgen(getter)]
    pub fn unique_identifier(&self) -> String {
        self.unique_identifier.clone()
    }

    // #[wasm_bindgen(getter)]
    // pub fn resources(&self) -> JsValue {
    //     serde_wasm_bindgen::to_value(&self.doc.resources).unwrap()
    // }

    // 获取资源的Base64编码
    #[wasm_bindgen]
    pub fn get_resource_base64(&mut self, id: &str) -> Option<String> {
        if let Some((data, mime)) = self.doc.get_resource(id) {
            Some(format!("data:{};base64,{}", mime, STANDARD.encode(data)))
            // // 将 Vec<u8> 转换为 Uint8Array
            // let uint8_array = Uint8Array::from(&data[..]);

            // // 创建 Blob
            // let blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&uint8_array)).ok()?;

            // // 创建一个 blob URL
            // let url = Url::create_object_url_with_blob(&blob).ok()?;

            // Some(url)
        } else {
            None
        }
    }

    // 通过路径获取资源的Base64编码
    #[wasm_bindgen]
    pub fn get_resource_base64_by_path(&mut self, path: &str) -> Option<String> {
        if let Some(data) = self.doc.get_resource_by_path(path) {
            let mime = self.doc.get_resource_mime_by_path(path)?;
            Some(format!("data:{};base64,{}", mime, STANDARD.encode(data)))
        } else {
            None
        }
    }

    // 获取资源的字符串表示
    #[wasm_bindgen]
    pub fn get_resource_str(&mut self, id: &str) -> Option<String> {
        if let Some((data, _mime)) = self.doc.get_resource_str(id) {
            Some(data)
        } else {
            None
        }
    }

    // 通过路径获取资源的字符串表示
    #[wasm_bindgen]
    pub fn get_resource_str_by_path(&mut self, path: &str) -> Option<String> {
        if let Some(data) = self.doc.get_resource_str_by_path(path) {
            Some(data)
        } else {
            None
        }
    }

    // 将资源URI转换为章节索引
    #[wasm_bindgen]
    pub fn resource_uri_to_chapter(&mut self, uri: &str) -> isize {
        self.doc
            .resource_uri_to_chapter(&std::path::PathBuf::from(uri))
            .map(|v| v as isize)
            .unwrap_or(-1)
    }

    #[wasm_bindgen]
    pub fn get_chapter_with_epub_uris(&mut self, id: &str) -> Option<String> {
        let page = self.doc.resource_id_to_chapter(id)?;
        self.doc.set_current_page(page);
        let current = self.doc.get_current_with_epub_uris().ok()?;
        // let mime = self.doc.get_current_mime()?;

        Some(String::from_utf8_lossy(&current).to_string())
    }

    // 添加 css
    #[wasm_bindgen]
    pub fn add_extra_css(&mut self, css: &str) {
        self.doc.add_extra_css(css);
    }


    // 获取封面图像的Base64编码
    fn get_cover_base64(&mut self) -> Option<String> {
        let (cover_data, mime) = self.doc.get_cover()?;
        Some(format!(
            "data:{};base64,{}",
            mime,
            STANDARD.encode(cover_data)
        ))
    }
}

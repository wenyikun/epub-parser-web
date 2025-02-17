# EpubParserWeb

EpubParserWeb 是一个基于 Rust EPUB 解析库构建的 WebAssembly 库，用于浏览器端解析 EPUB 文件。

## 功能特性

- 解析 EPUB 文件
- 获取书籍元数据（书名、作者、语言、描述）
- 获取封面图片的 Base64 编码
- 获取目录 (TOC) 并转换为 JSON 结构
- 获取书脊 (Spine) 内容
- 获取 EPUB 内部资源，并支持 Base64 编码
- 将资源 URI 转换为章节索引

## 使用示例

### 安装 npm 包

```sh
npm install epub-parser-web
```

### 初始化 `EpubParser`

```javascript
import init, { EpubParser } from 'epub-parser-web'

async function loadEpub(file) {
  await init()
  const arrayBuffer = await file.arrayBuffer()
  const parser = new EpubParser(new Uint8Array(arrayBuffer))

  console.log('Title:', parser.title)
  console.log('Author:', parser.author)
  console.log('Language:', parser.language)
  console.log('Description:', parser.description)
  console.log('Cover Image Base64:', parser.cover)
  console.log('TOC:', parser.toc)
  console.log('Spine:', parser.spine)
  console.log('Unique Identifier:', parser.unique_identifier)
}
```

### 通过 ID 获取 EPUB 资源

```javascript
const resourceBase64 = parser.get_resource_base64('resource_id')
if (resourceBase64) {
  console.log('Resource Base64:', resourceBase64)
}
```

### 通过路径获取 EPUB 资源

```javascript
const resourceBase64 = parser.get_resource_base64_by_path('path')
if (resourceBase64) {
  console.log('Resource Base64:', resourceBase64)
}
```

### 通过 ID 获取资源字符串

```javascript
const resourceStr = parser.get_resource_str('resource_id')
if (resourceStr) {
  console.log('Resource String:', resourceStr)
}
```

### 通过路径获取资源字符串

```javascript
const resourceStr = parser.get_resource_str_by_path('path')
if (resourceStr) {
  console.log('Resource String:', resourceStr)
}
```

### 资源 URI 转章节索引

```javascript
const chapterIndex = parser.resource_uri_to_chapter('path/to/resource.xhtml')
console.log('Chapter Index:', chapterIndex)
```

### 获取章节内容

```javascript
const chapterContent = parser.get_chapter_with_epub_uris('resource_id')
if (chapterContent) {
  console.log('Chapter Content:', chapterContent)
}
```

### 注入 CSS

```javascript
parser.add_extra_css(`body { background-color: black; color: white }`)
```

## 依赖

- `epub` (EPUB 解析库)
- `wasm-bindgen` (Rust to WASM 绑定)
- `serde` & `serde_json` (用于 JSON 序列化)
- `base64` (Base64 编码支持)
- `md5` (用于计算唯一标识符)

## 许可

本项目基于 MIT 许可协议。

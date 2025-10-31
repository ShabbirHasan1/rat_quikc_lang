# rat_quikc_lang

基于 rat_embed_lang 的多语言包装器，提供简洁的API和自动文件加载功能。

## 特性

- 基于 rat_embed_lang v0.1.1 核心实现
- 支持参数化翻译（如 "欢迎，{username}！"）
- 支持文件夹和单文件两种组织形式
- 自动文件扫描和加载
- 内存缓存，高性能翻译查询
- 线程安全
- 简洁的函数调用API（无宏）

## 文件组织支持

### 文件夹形式
```
lang/
├── common/
│   ├── zh_CN.toml
│   ├── en_US.toml
│   └── ja_JP.toml
├── errors/
│   ├── zh_CN.toml
│   └── en_US.toml
```

### 单文件形式
```
lang/
├── ui.toml          # 包含多个语言的翻译
└── menu.toml        # 包含多个语言的翻译
```

## 依赖

```toml
[dependencies]
rat_embed_lang = "0.1.1"
rat_quikc_lang = "0.1.1"
```

## 核心 API

### 基础翻译
- `load_translations(base_dir: &str) -> Result<(), LangError>` - 加载语言文件
- `t(key: &str) -> String` - 获取翻译文本
- `set_language(lang: &str) -> Result<(), LangError>` - 设置当前语言
- `current_language() -> String` - 获取当前语言
- `has_translation(key: &str) -> bool` - 检查翻译是否存在
- `get_all_keys() -> Vec<String>` - 获取所有翻译键
- `get_supported_languages(key: &str) -> Vec<String>` - 获取指定键支持的语言
- `reload(base_dir: &str) -> Result<(), LangError>` - 重新加载语言文件

### 参数化翻译（新增）
- `tf(key: &str, args: &[(&str, &str)]) -> String` - 获取参数化翻译文本
- `tf_with_lang(key: &str, lang: &str, args: &[(&str, &str)]) -> String` - 获取指定语言的参数化翻译文本

## 快速开始

查看 `examples/` 目录中的示例代码：

- `basic_usage.rs` - 基础翻译功能
- `parameterized_translation.rs` - 参数化翻译功能

运行示例：
```bash
cargo run --example basic_usage
cargo run --example parameterized_translation
```

## 文件格式

### 文件夹形式 - common/zh_CN.toml
```toml
welcome = "欢迎"
buttons.save = "保存"
buttons.cancel = "取消"
```

### 单文件形式 - ui.toml
```toml
[zh_CN]
title = "标题"
exit = "退出"

[en_US]
title = "Title"
exit = "Exit"
```

### 参数化翻译示例
支持在翻译文本中使用 `{参数名}` 格式的占位符：

```toml
[zh_CN]
welcome_user = "欢迎，{username}！"
login_message = "{user}在{time}登录了系统"

[en_US]
welcome_user = "Welcome, {username}!"
login_message = "{user} logged in at {time}"
```

## 语言代码自动标准化

支持多种格式的语言代码输入：
- `zh`, `zh_cn`, `zh-cn` → `zh-CN`
- `en`, `en_us`, `en-us` → `en-US`
- `ja`, `ja_jp`, `ja-jp` → `ja-JP`

## 线程安全

基于 rat_embed_lang 的 `RwLock` 实现，支持多线程并发读取。

## 版本历史

### v0.1.1
- 升级到 rat_embed_lang v0.1.1
- 新增参数化翻译功能支持
- 添加 `tf()` 和 `tf_with_lang()` API

### v0.1.0
- 初始版本发布
- 基础翻译功能
- 文件夹和单文件支持
/// 错误类型定义
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum LangError {
    #[error("语言目录不存在: {path}")]
    DirectoryNotFound { path: String },

    #[error("模块不存在: {module}")]
    ModuleNotFound { module: String },

    #[error("文件加载失败: {path}: {message}")]
    FileLoadError {
        path: String,
        message: String,
    },

    #[error("TOML解析失败: {file}: {message}")]
    ParseError {
        file: String,
        message: String,
    },

    #[error("不支持的语言代码: {lang}")]
    UnsupportedLanguage { lang: String },

    #[error("翻译键重复: {key}")]
    DuplicateKey { key: String },

    #[error("初始化失败: {reason}")]
    InitializationFailed { reason: String },
}
/// 文件加载器 - 负责扫描和解析语言文件
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::LangError;

/// 支持的文件格式类型
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// 文件夹形式: lang/common/zh_CN.toml
    Folder,
    /// 单文件形式: lang/ui.toml (包含多个语言)
    SingleFile,
}

/// 文件加载器
pub struct FileLoader {
    base_dir: PathBuf,
}

impl FileLoader {
    /// 创建新的文件加载器
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// 扫描并加载所有翻译模块
    pub fn scan_and_load_all(&self) -> Result<HashMap<String, HashMap<String, String>>, LangError> {
        if !self.base_dir.exists() {
            return Err(LangError::DirectoryNotFound {
                path: self.base_dir.to_string_lossy().to_string(),
            });
        }

        // 先收集所有模块的翻译数据，然后合并为 rat_embed_lang 期望的格式
        let mut module_translations: HashMap<String, HashMap<String, String>> = HashMap::new();

        // 扫描目录中的所有条目
        for entry in std::fs::read_dir(&self.base_dir)
            .map_err(|e| LangError::FileLoadError {
                path: self.base_dir.to_string_lossy().to_string(),
                message: e.to_string(),
            })?
        {
            let entry = entry.map_err(|e| LangError::FileLoadError {
                path: self.base_dir.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;
            let path = entry.path();

            if path.is_dir() {
                // 文件夹形式
                let module_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| LangError::ModuleNotFound {
                        module: "invalid_module_name".to_string(),
                    })?;

                let translations = self.load_folder_module(module_name)?;
                // 将数据从 语言 -> 翻译 转换为 翻译键 -> 语言 -> 翻译文本
                self.merge_translations(&mut module_translations, translations);
            } else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                // 单文件形式
                let module_name = path.file_stem()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| LangError::ModuleNotFound {
                        module: "invalid_module_name".to_string(),
                    })?;

                let translations = self.load_single_file_module(module_name)?;
                self.merge_translations(&mut module_translations, translations);
            }
        }

        Ok(module_translations)
    }

    /// 合并翻译数据到最终格式
    fn merge_translations(
        &self,
        module_translations: &mut HashMap<String, HashMap<String, String>>,
        lang_data: HashMap<String, HashMap<String, String>>,
    ) {
        for (lang_code, translations) in lang_data {
            for (translation_key, translation_text) in translations {
                // 获取或创建这个翻译键的语言映射
                let lang_map = module_translations.entry(translation_key.clone()).or_insert_with(HashMap::new);
                lang_map.insert(lang_code.clone(), translation_text);
            }
        }
    }

    /// 加载文件夹形式的模块
    fn load_folder_module(&self, module: &str) -> Result<HashMap<String, HashMap<String, String>>, LangError> {
        let module_dir = self.base_dir.join(module);
        let mut all_translations = HashMap::new();

        if !module_dir.is_dir() {
            return Err(LangError::ModuleNotFound {
                module: module.to_string(),
            });
        }

        // 扫描文件夹中的所有 .toml 文件
        for entry in std::fs::read_dir(&module_dir)
            .map_err(|e| LangError::FileLoadError {
                path: module_dir.to_string_lossy().to_string(),
                message: e.to_string(),
            })?
        {
            let entry = entry.map_err(|e| LangError::FileLoadError {
                path: module_dir.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                // 从文件名提取语言代码，如 zh_CN.toml -> zh-CN
                let lang_code = path.file_stem()
                    .and_then(|n| n.to_str())
                    .map(|s| normalize_language_code(s))
                    .ok_or_else(|| LangError::UnsupportedLanguage {
                        lang: "unknown".to_string(),
                    })?;

                let translations = self.parse_toml_file(&path)?;
                let prefixed_translations = self.add_module_prefix(translations, module);

                all_translations.insert(lang_code, prefixed_translations);
            }
        }

        Ok(all_translations)
    }

    /// 加载单文件形式的模块
    fn load_single_file_module(&self, module: &str) -> Result<HashMap<String, HashMap<String, String>>, LangError> {
        let file_path = self.base_dir.join(format!("{}.toml", module));

        if !file_path.exists() {
            return Err(LangError::ModuleNotFound {
                module: module.to_string(),
            });
        }

        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| LangError::FileLoadError {
                path: file_path.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;

        let parsed: toml::Value = toml::from_str(&content)
            .map_err(|e| LangError::ParseError {
                file: file_path.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;

        let mut all_translations = HashMap::new();

        if let Some(table) = parsed.as_table() {
            for (lang_code, lang_data) in table {
                if let Some(lang_table) = lang_data.as_table() {
                    let mut translations = HashMap::new();
                    for (key, value) in lang_table {
                        if let Some(text) = value.as_str() {
                            translations.insert(key.clone(), text.to_string());
                        }
                    }

                    let prefixed_translations = self.add_module_prefix(translations, module);
                    all_translations.insert(normalize_language_code(lang_code), prefixed_translations);
                }
            }
        }

        Ok(all_translations)
    }

    /// 解析单个 TOML 文件
    fn parse_toml_file(&self, file_path: &Path) -> Result<HashMap<String, String>, LangError> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| LangError::FileLoadError {
                path: file_path.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;

        let parsed: toml::Value = toml::from_str(&content)
            .map_err(|e| LangError::ParseError {
                file: file_path.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;

        let mut translations = HashMap::new();

        if let Some(table) = parsed.as_table() {
            for (key, value) in table {
                if let Some(text) = value.as_str() {
                    translations.insert(key.clone(), text.to_string());
                }
            }
        }

        Ok(translations)
    }

    /// 为所有翻译键添加模块前缀
    fn add_module_prefix(&self, translations: HashMap<String, String>, module: &str) -> HashMap<String, String> {
        let mut prefixed = HashMap::new();
        for (key, value) in translations {
            let full_key = format!("{}.{}", module, key);
            prefixed.insert(full_key, value);
        }
        prefixed
    }
}

// 使用 rat_embed_lang 的语言标准化函数
use rat_embed_lang::normalize_language_code;
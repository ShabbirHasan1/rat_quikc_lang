/// rat_quikc_lang - 基于rat_embed_lang的多语言包装器
///
/// 提供简洁的API和自动文件加载功能
pub mod error;
pub mod loader;

pub use error::LangError;

use std::sync::OnceLock;
use crate::loader::FileLoader;

static INIT_RESULT: OnceLock<Result<(), LangError>> = OnceLock::new();

/// 加载多语言文件
///
/// # 参数
/// * `base_dir` - 语言文件的基础目录，如 "lang"
pub fn load_translations(base_dir: &str) -> Result<(), LangError> {
    let result = INIT_RESULT.get_or_init(|| {
        let loader = FileLoader::new(base_dir);
        let translations = loader.scan_and_load_all()?;

        // 注册翻译数据到 rat_embed_lang
        rat_embed_lang::register_translations(translations);

        // 尝试从环境变量获取语言设置
        let lang = rat_embed_lang::get_language_from_env();
        rat_embed_lang::set_language(&lang);

        println!("多语言系统加载完成，当前语言: {}", lang);
        Ok(())
    });

    result.clone()
}

/// 初始化多语言系统（保持向后兼容）
#[deprecated(note = "使用 load_translations 替代")]
pub fn init(base_dir: &str) -> Result<(), LangError> {
    load_translations(base_dir)
}

/// 获取翻译文本
///
/// # 参数
/// * `key` - 翻译键，格式为 "module.key"，如 "common.welcome"
///
/// # 返回值
/// 返回翻译文本，如果找不到则返回 \[key\]
pub fn t(key: &str) -> String {
    rat_embed_lang::t(key)
}

/// 设置当前语言
///
/// # 参数
/// * `lang` - 语言代码，如 "zh-CN", "en-US"
pub fn set_language(lang: &str) -> Result<(), LangError> {
    rat_embed_lang::set_language(lang);
    Ok(())
}

/// 获取当前语言
pub fn current_language() -> String {
    rat_embed_lang::current_language()
}

/// 检查是否存在指定key的翻译
pub fn has_translation(key: &str) -> bool {
    rat_embed_lang::has_translation(key)
}

/// 获取所有支持的key列表
pub fn get_all_keys() -> Vec<String> {
    rat_embed_lang::get_all_keys()
}

/// 获取指定key支持的所有语言
pub fn get_supported_languages(key: &str) -> Vec<String> {
    rat_embed_lang::get_supported_languages(key)
}

/// 重新加载语言文件（用于热重载）
///
/// # 参数
/// * `base_dir` - 语言文件的基础目录
///
/// # 注意
/// 这会清除现有的翻译数据并重新加载所有文件
pub fn reload(base_dir: &str) -> Result<(), LangError> {
    // 清除现有翻译
    rat_embed_lang::clear_translations();

    // 重新加载
    let loader = FileLoader::new(base_dir);
    let translations = loader.scan_and_load_all()?;

    // 注册新的翻译数据
    rat_embed_lang::register_translations(translations);

    println!("语言文件重新加载完成");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_basic_functionality() {
        // 清理之前可能存在的测试目录
        let _ = fs::remove_dir_all("test_lang");

        // 创建测试目录和文件
        fs::create_dir_all("test_lang/common").unwrap();
        fs::write("test_lang/common/zh_CN.toml", r#"
welcome = "欢迎"
buttons.save = "保存"
"#).unwrap();

        fs::write("test_lang/common/en_US.toml", r#"
welcome = "Welcome"
buttons.save = "Save"
"#).unwrap();

        // 测试单文件形式
        fs::write("test_lang/ui.toml", r#"
[zh_CN]
title = "标题"

[en_US]
title = "Title"
"#).unwrap();

        // 直接加载测试，不使用全局初始化
        let loader = FileLoader::new("test_lang");
        let translations = loader.scan_and_load_all().unwrap();

        // 清除之前的翻译
        rat_embed_lang::clear_translations();

        // 注册翻译数据
        rat_embed_lang::register_translations(translations);

        // 测试翻译 - 应该从环境变量检测到英语
        let welcome = t("common.welcome");

        // 切换到中文
        set_language("zh-CN").unwrap();
        let welcome_zh = t("common.welcome");

        // 验证中文翻译
        assert_eq!(welcome_zh, "欢迎");

        // 切换回英语
        set_language("en-US").unwrap();
        let welcome_en = t("common.welcome");
        assert_eq!(welcome_en, "Welcome");

        // 测试单文件
        let title = t("ui.title");
        assert!(!title.is_empty());
        assert!(title == "标题" || title == "Title");

        // 清理
        fs::remove_dir_all("test_lang").unwrap();
    }

    #[test]
    fn test_file_loader() {
        // 清理之前可能存在的测试目录
        let _ = fs::remove_dir_all("test_lang2");

        // 测试文件夹形式
        fs::create_dir_all("test_lang2/module1").unwrap();
        fs::write("test_lang2/module1/zh_CN.toml", "test = \"测试\"").unwrap();
        fs::write("test_lang2/module1/en_US.toml", "test = \"Test\"").unwrap();

        let loader = FileLoader::new("test_lang2");
        let translations = loader.scan_and_load_all().unwrap();

        // 现在数据格式是：翻译键 -> 语言 -> 翻译文本
        assert!(translations.contains_key("module1.test"));

        // 验证语言映射
        let lang_map = &translations["module1.test"];
        assert!(lang_map.contains_key("zh-CN"));
        assert!(lang_map.contains_key("en-US"));
        assert_eq!(lang_map["zh-CN"], "测试");
        assert_eq!(lang_map["en-US"], "Test");

        // 清理
        fs::remove_dir_all("test_lang2").unwrap();
    }
}
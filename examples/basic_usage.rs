// 完整使用示例
use rat_quikc_lang::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 准备示例语言文件
    setup_example_files()?;

    // 加载多语言文件
    load_translations("example_lang")?;

    println!("=== 多语言系统演示 ===");

    // 显示当前语言
    println!("当前语言: {}", current_language());

    // 使用翻译
    println!("欢迎信息: {}", t("common.welcome"));
    println!("保存按钮: {}", t("common.buttons.save"));
    println!("取消按钮: {}", t("common.buttons.cancel"));
    println!("错误信息: {}", t("errors.file_not_found"));

    println!("\n--- 切换到英语 ---");

    // 切换语言
    set_language("en-US")?;
    println!("当前语言: {}", current_language());

    // 使用英语翻译
    println!("Welcome message: {}", t("common.welcome"));
    println!("Save button: {}", t("common.buttons.save"));
    println!("Cancel button: {}", t("common.buttons.cancel"));
    println!("Error message: {}", t("errors.file_not_found"));

    println!("\n--- 测试单文件模块 ---");

    // 测试单文件形式的翻译
    println!("界面标题: {}", t("ui.title"));
    println!("退出按钮: {}", t("ui.exit"));

    // 显示所有可用的翻译键
    println!("\n--- 所有翻译键 ---");
    for key in get_all_keys() {
        println!("  {}", key);
    }

    // 清理示例文件
    cleanup_example_files();

    Ok(())
}

fn setup_example_files() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    // 创建语言目录结构
    fs::create_dir_all("example_lang/common")?;
    fs::create_dir_all("example_lang/errors")?;

    // 文件夹形式 - common 模块
    fs::write("example_lang/common/zh_CN.toml", r#"welcome = "欢迎使用我们的应用"
buttons.save = "保存"
buttons.cancel = "取消"
"#)?;

    fs::write("example_lang/common/en_US.toml", r#"welcome = "Welcome to our application"
buttons.save = "Save"
buttons.cancel = "Cancel"
"#)?;

    // 文件夹形式 - errors 模块
    fs::write("example_lang/errors/zh_CN.toml", r#"file_not_found = "文件未找到"
network_error = "网络连接错误"
"#)?;

    fs::write("example_lang/errors/en_US.toml", r#"file_not_found = "File not found"
network_error = "Network connection error"
"#)?;

    // 单文件形式 - ui 模块
    fs::write("example_lang/ui.toml", r#"[zh_CN]
title = "用户界面"
exit = "退出"

[en_US]
title = "User Interface"
exit = "Exit"
"#)?;

    Ok(())
}

fn cleanup_example_files() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    let _ = fs::remove_dir_all("example_lang");
    Ok(())
}
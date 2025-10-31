/// 参数化翻译示例

fn main() {
    println!("=== 参数化翻译示例 ===\n");

    // 初始化翻译系统
    let current_dir = std::env::current_dir().unwrap();
    let lang_path = current_dir.join("examples/lang");

    if let Err(e) = rat_quikc_lang::load_translations(lang_path.to_str().unwrap()) {
        eprintln!("加载翻译失败: {}", e);
        return;
    }

    // 测试参数化翻译
    let languages = vec!["zh-CN", "en-US"];

    for lang in languages {
        rat_quikc_lang::set_language(lang).unwrap();
        println!("=== 语言: {} ===", lang);

        // 基本翻译
        let welcome = rat_quikc_lang::t("welcome.welcome");
        println!("基本欢迎: {}", welcome);

        // 参数化翻译 - 用户名
        let welcome_user = rat_quikc_lang::tf("welcome.welcome_user", &[("username", "张三")]);
        println!("欢迎用户: {}", welcome_user);

        // 参数化翻译 - 登录消息
        let login = rat_quikc_lang::tf("welcome.login_message", &[("user", "admin"), ("time", "14:30")]);
        println!("登录消息: {}", login);

        // 参数化翻译 - 错误消息
        let error = rat_quikc_lang::tf("welcome.error_message", &[("code", "404"), ("desc", "文件未找到")]);
        println!("错误消息: {}", error);

        println!();
    }
}
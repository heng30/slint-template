use crate::config;
use std::collections::HashMap;

pub fn tr(text: &str) -> String {
    if config::ui().language == "cn" {
        return text.to_string();
    }

    let mut items: HashMap<&str, &str> = HashMap::new();
    items.insert("出错", "Error");
    items.insert("原因", "Reason");
    items.insert("取消", "Cancel");
    items.insert("确认", "Confirm");
    items.insert("确定", "Confirm");
    items.insert("编辑", "Edit");
    items.insert("删除", "Delete");
    items.insert("清空", "Clear");
    items.insert("发送", "Send");

    items.insert("删除成功", "Delete success");
    items.insert("删除失败", "Delete failed");
    items.insert("添加成功", "Add success");
    items.insert("添加失败", "Add failed");
    items.insert("复制失败", "Copy failed");
    items.insert("复制成功", "Copy success");
    items.insert("清空失败", "Delete failed");
    items.insert("清空成功", "Delete success");
    items.insert("保存失败", "Save failed");
    items.insert("保存成功", "Save success");
    items.insert("重置成功", "Reset success");
    items.insert("刷新成功", "Refresh success");
    items.insert("刷新失败", "Refresh failed");
    items.insert("发送失败", "Send failed");
    items.insert("下载成功", "Download success");
    items.insert("下载失败", "Download failed");
    items.insert("加载失败", "Load failed");
    items.insert("非法输入", "Invalid input");
    items.insert("打开链接失败", "Open link failed");

    items.insert("新建成功", "New success");
    items.insert("新建失败", "New failed");
    items.insert("编辑成功", "Edit success");
    items.insert("编辑失败", "Edit failed");

    items.insert("微信支付", "Wechat pay");
    items.insert("小狐狸（加密）支付", "MetaMask crypto pay");

    items.insert("收藏成功", "Favorite success");
    items.insert("收藏失败", "Favorite failed");
    items.insert("取消收藏成功", "Cancel favorite success");
    items.insert("取消收藏失败", "Cancel favorite failed");

    items.insert("正在刷新...", "Refreshing...");
    items.insert("正在同步...", "Syncing...");
    items.insert("同步成功", "Refresh success");
    items.insert("同步完成", "Refresh finished");
    items.insert("查找完成", "Search finish");
    items.insert("返回为空", "Empty data");

    items.insert("是否删除？", "Delete or not?");
    items.insert("是否删除全部？", "Delete all entrys or not?");
    items.insert("是否删除全部缓存？", "Delete all cache or not?");
    items.insert("清除缓存失败", "Remove cache failed");
    items.insert("清除缓存成功", "Remove cache success");
    items.insert("超过1000字数限制", "Over the limit of 2048 word counts");

    items.insert("界 面", "UI");
    items.insert("偏好设置", "Preference");
    items.insert("账户管理", "Account");
    items.insert("阅 读", "Reading");
    items.insert("同 步", "Sync");
    items.insert("代 理", "Proxy");
    items.insert("缓 存", "Cache");
    items.insert("关 于", "About");
    items.insert("帮 助", "Help");
    items.insert("反 馈", "Feedback");
    items.insert("捐 赠", "Donate");

    items.insert("新建", "New");
    items.insert("没有订阅", "No RSS");
    items.insert("RSS名称和图标", "RSS name and icon");
    items.insert("请输入RSS名称", "Please input RSS name");
    items.insert("RSS源地址", "RSS URL");
    items.insert("请输入RSS源地址", "Please input RSS URL");
    items.insert("RSS源格式", "RSS format");
    items.insert("已启用Http代理", "Enabled Http proxy");
    items.insert("未启用Http代理", "Disable Http proxy");
    items.insert("已启用Socks5代理", "Enabled Socks5 proxy");
    items.insert("未启用Socks5代理", "Disable Socks5 proxy");
    items.insert("已收藏", "Star");
    items.insert("未收藏", "Not star");
    items.insert("图标库", "Icons");
    items.insert("请选择条目", "Please select entry");
    items.insert("请添加RSS源", "Please add RSS URL");
    items.insert("选择浏览器", "Select browser");
    items.insert("已启用阅后即焚", "Enabled delete after reading");
    items.insert("未启用阅后即焚", "Disable delete after reading");

    items.insert("字体大小", "Font size");
    items.insert("字体样式", "Font family");
    items.insert("选择语言", "Choose language");
    items.insert("同步时间间隔(分钟)", "Sync time interval(minute)");
    items.insert("请输入时间间隔", "Please input time interval");
    items.insert("同步超时(秒)", "Sync timeout(second)");
    items.insert("请输入同步超时", "Please input sync timeout");
    items.insert("已启用自动同步", "Enabled auto sync");
    items.insert("未启用自动同步", "Disable auto sync");
    items.insert(
        "程序启动时，马上进行一次同步",
        "Starting sync once, when application starting",
    );
    items.insert(
        "程序启动时，不马上进行一次同步",
        "Don't start sync, when application starting",
    );
    items.insert("代理地址", "Proxy address");
    items.insert("代理端口", "Proxy port");

    items.insert("警告", "Warning");
    items.insert("订阅", "RSS");
    items.insert("收藏夹", "Collection");
    items.insert("发现", "Find");
    items.insert("添加", "Add");
    items.insert("设置", "Setting");

    items.insert("成功移除黑名单", "Remove from blacklist success");
    items.insert("没有数据", "No Data");
    items.insert("没有消息", "No Message");
    items.insert("输入关键字", "Input keyword");

    items.insert("备份与恢复", "Backup and recover");
    items.insert("API 令牌", "API token");
    items.insert("请输入API令牌", "Please input API token");
    items.insert("备份与恢复选项", "Backup and recover options");
    items.insert("RSS列表", "RSS list");
    items.insert("用户设置", "User setting");
    items.insert("备份", "Backup");
    items.insert("恢复", "Recover");
    items.insert("备份成功", "Backup success");
    items.insert("备份失败", "Backup failed");
    items.insert("恢复成功", "Recover success");
    items.insert("恢复失败", "Recover failed");
    items.insert("是否备份？", "Backup or not?");
    items.insert("是否恢复？", "Recover or not?");

    items.insert("获取最新版本", "Latest version");
    items.insert("版本信息", "Current version");
    items.insert("当前版本", "Latest version");
    items.insert("更新信息", "Update detail");
    items.insert("下载最新版本", "Download");
    items.insert("选择主题", "Choose Theme");
    items.insert("白天", "Light");
    items.insert("黑暗", "Dark");
    items.insert("跳过", "Skip");
    items.insert("下一步", "Next");
    items.insert("完成", "Finish");
    items.insert("返回", "Back");
    items.insert("请选择语言", "Please select language");
    items.insert("没有记录", "No record");
    items.insert("没有地址", "No address");
    items.insert("请输入用户名", "Please enter username");
    items.insert("用户名", "Username");
    items.insert("请输入密码", "Please enter password");
    items.insert("请再次输入密码", "Please enter password again");
    items.insert("至少8个字符", "At least 8 chars");
    items.insert("创建账户", "Create account");
    items.insert("恢复账户", "Recover account");
    items.insert("生成组记词失败", "New mnemonic failed");
    items.insert("登录", "Login");
    items.insert(
        "组记词数量不对，仅支持12和24个组记词",
        "Mnemonic counts is no correct. Only support 12 or 24 word counts mnemonic",
    );
    items.insert("非法组记词", "Invalid mnemonic");
    items.insert("用户名不能为空", "Username can not be empty");
    items.insert("密码不相同", "Two passwords is different");
    items.insert("密码不能小于8位", "Password can not less than 8 chars");
    items.insert("更新账户成功", "Update account success");
    items.insert(
        "更新账户失败. 账户不存在",
        "Update account failed. The account don't exist",
    );
    items.insert("不允许删除当前用户", "Not allow delete current account");
    items.insert("不允许删除主账号", "Not allow delete the main account");
    items.insert("删除账户成功", "Delete account success");
    items.insert("移除账户成功", "Remove account success");
    items.insert("切换账户成功", "Switch account success");
    items.insert(
        "切换账户失败. 账户不存在",
        "Switch account failed. The account don't exist",
    );
    items.insert(
        "创建用户失败. 非法密码",
        "Create account failed. Invalid password",
    );
    items.insert("创建用户成功", "Create account success");
    items.insert("创建用户失败", "Create account failed");
    items.insert("账户管理", "Account Management");
    items.insert("是否删除账户？", "Delete account or not?");
    items.insert("内部错误", "Internal error");
    items.insert("密码错误", "Password is wrong");
    items.insert("组记词", "Mnemonic");
    items.insert("地址簿", "Address Book");
    items.insert("移除地址", "Delete address");
    items.insert("是否删除地址？", "Delete address or not?");
    items.insert("地址名称", "Address name");
    items.insert("账户地址", "Account address");
    items.insert("地 址", "Address");
    items.insert("删除地址成功", "Delete address success");
    items.insert("更新地址成功", "Update address success");
    items.insert("更新地址失败", "Update address failed");
    items.insert("切换账户", "Switch account");
    items.insert("是否删除所有账户？", "Delete all account or not?");
    items.insert("删除所有账户成功", "Delete all accounts success");
    items.insert("打开成功", "Open link success");
    items.insert("打开失败", "Open link failed");
    items.insert("安全与隐私", "Security & Privacy ");
    items.insert("重置账户", "Reset account");
    items.insert("是否重置账户？", "Reset account or not?");
    items.insert("请输入旧密码", "Please input old password");
    items.insert("请输入新密码", "Please input new password");
    items.insert("请再次输入新密码", "Please input new password again");
    items.insert("测试模式", "Test model");
    items.insert("主网络", "Main Network");
    items.insert("测试网络", "Test Network");
    items.insert("开发网络", "Dev Network");
    items.insert("未知网络", "Unknown Network");
    items.insert("注意: 当前处于", "Warning: Current Network is ");
    items.insert("主页", "Home");
    items.insert("接收", "Recipient");
    items.insert("接收代币", "Recipient");
    items.insert("历史", "History");
    items.insert("历史记录", "History");
    items.insert("开发者模式", "Developer Mode");
    items.insert("账户名称", "Account name");
    items.insert("显示组记词", "Show mnemonics");
    items.insert("移除账户", "Remove account");
    items.insert("更改密码", "Change password");
    items.insert("位组记词", "mnemonics");
    items.insert("刷新完成", "Refresh finished");
    items.insert("管理代币", "Manage token");
    items.insert("发送代币", "Send token");
    items.insert("获取Token失败", "Fetch account tokens failed");
    items.insert("更新账户余额成功", "Refresh account balance success");
    items.insert("请求空投", "Request airdrop");
    items.insert("请求空投成功", "Request airdrop success");
    items.insert("请求空投失败", "Request airdrop failed");
    items.insert("请耐心等待...", "It may takes a long time. Please wait...");
    items.insert("估计交易费用失败", "Evaluating transaction fee failed");
    items.insert("正在估计交易费用...", "Evaluating gas fee...");
    items.insert("交易记录", "Transaction signature");
    items.insert("区块网络", "Blockchain network");
    items.insert("发送数量", "Send amount");
    items.insert("交易记录", "Transaction history");
    items.insert("关闭", "Close");
    items.insert("交易失败", "Transaction failed");
    items.insert("等待交易确认...", "Waiting transaction confirmed...");
    items.insert("交易信息", "Transaction detail");
    items.insert("发送地址", "Send address");
    items.insert("接收地址", "Recipient address");
    items.insert("交易费用", "Transaction fee");
    items.insert("交易已经确认", "Transaction has been confirmed");
    items.insert("交易成功", "Transaction success");
    items.insert("非法地址", "Invalid address");
    items.insert("账户", "Account");
    items.insert("创建账户费用", "Create token account fee");
    items.insert("创建组记词", "Create mnemonic");
    items.insert("恢复账户", "Recover account");
    items.insert("二维码", "QrCode");
    items.insert("高级设置", "Advance setting");
    items.insert("备注", "Memo");
    items.insert("优先费用", "Prioritization fee");
    items.insert("基础费用", "Base fee");
    items.insert("最大优先费用", "Max prioritization fee");
    items.insert("最大优先费用为", "Max prioritization fee is");
    items.insert("慢", "Slow");
    items.insert("正常", "Normal");
    items.insert("快", "Fast");
    items.insert("非法优先费用", "Invalid prioritization fee");
    items.insert("请设置更大的优先费用", "Please setting max prioritization fee");
    items.insert("登出账户", "Logout account");
    items.insert("密码错误", "Wrong password");
    items.insert("内部错误. 密码不存在", "Internal error. Password not exist");
    items.insert(
        "更新账户余额失败. 账户不存在",
        "Refresh account balance failed. The account is not found",
    );

    items.insert(
        "创建账户和使用组记词恢复账户",
        "Create account and recover account from mnemonics",
    );
    items.insert(
        "查看、发送和接收Sol代币和Solana的通证",
        "Show, send and receive Sol and tokens of Solana",
    );
    items.insert(
        "欢迎使用，享受你的加密之旅",
        "Welcome! Enjoying you journey of crypto",
    );

    if let Some(txt) = items.get(text) {
        return txt.to_string();
    }

    text.to_string()
}

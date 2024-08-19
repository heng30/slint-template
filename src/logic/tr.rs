use crate::config;
use std::collections::HashMap;

pub fn tr(text: &str) -> String {
    if config::ui().language == "en" {
        return text.to_string();
    }

    let items: HashMap<&str, &str> = HashMap::from([
        ("Error", "出错"),
        ("Reason", "原因"),
        ("Cancel", "取消"),
        ("Confirm", "确认"),
        ("Confirm", "确定"),
        ("Edit", "编辑"),
        ("Delete", "删除"),
        ("Clear", "清空"),
        ("Send", "发送"),
        ("Delete success", "删除成功"),
        ("Delete failed", "删除失败"),
        ("Add success", "添加成功"),
        ("Add failed", "添加失败"),
        ("Copy failed", "复制失败"),
        ("Copy success", "复制成功"),
        ("Paste failed", "粘贴失败"),
        ("Delete failed", "清空失败"),
        ("Delete success", "清空成功"),
        ("Save failed", "保存失败"),
        ("Save success", "保存成功"),
        ("Reset success", "重置成功"),
        ("Refresh success", "刷新成功"),
        ("Refresh failed", "刷新失败"),
        ("Send failed", "发送失败"),
        ("Download success", "下载成功"),
        ("Download failed", "下载失败"),
        ("Load failed", "加载失败"),
        ("Invalid input", "非法输入"),
        ("Open link failed", "打开链接失败"),
        ("Input can not be empty", "输入不能为空"),
        ("New success", "新建成功"),
        ("New failed", "新建失败"),
        ("Edit success", "编辑成功"),
        ("Edit failed", "编辑失败"),
        ("Wechat pay", "微信支付"),
        ("MetaMask crypto pay", "小狐狸（加密）支付"),
        ("Favorite success", "收藏成功"),
        ("Favorite failed", "收藏失败"),
        ("Cancel favorite success", "取消收藏成功"),
        ("Cancel favorite failed", "取消收藏失败"),
        ("Refreshing...", "正在刷新..."),
        ("Syncing...", "正在同步..."),
        ("Refresh success", "同步成功"),
        ("Refresh finished", "同步完成"),
        ("Search finish", "查找完成"),
        ("Empty data", "返回为空"),
        ("Delete or not?", "是否删除？"),
        ("Delete all entrys or not?", "是否删除全部？"),
        ("Delete all cache or not?", "是否删除全部缓存？"),
        ("Remove cache failed", "清除缓存失败"),
        ("Remove cache success", "清除缓存成功"),
        ("Over the limit of 2048 word counts", "超过1000字数限制"),
        ("UI", "界 面"),
        ("Preference", "偏 好"),
        ("Account", "账户管理"),
        ("Reading", "阅 读"),
        ("Sync", "同 步"),
        ("Proxy", "代 理"),
        ("Cache", "缓 存"),
        ("About", "关 于"),
        ("Help", "帮 助"),
        ("Feedback", "反 馈"),
        ("Donate", "捐 赠"),
        ("New", "新建"),
        ("No RSS", "没有订阅"),
        ("RSS name and icon", "RSS名称和图标"),
        ("Please input RSS name", "请输入RSS名称"),
        ("RSS URL", "RSS源地址"),
        ("Please input RSS URL", "请输入RSS源地址"),
        ("RSS format", "RSS源格式"),
        ("Enabled Http proxy", "已启用Http代理"),
        ("Disable Http proxy", "未启用Http代理"),
        ("Enabled Socks5 proxy", "已启用Socks5代理"),
        ("Disable Socks5 proxy", "未启用Socks5代理"),
        ("Star", "已收藏"),
        ("Not star", "未收藏"),
        ("Icons", "图标库"),
        ("Please select entry", "请选择条目"),
        ("Please add RSS URL", "请添加RSS源"),
        ("Select browser", "选择浏览器"),
        ("Enabled delete after reading", "已启用阅后即焚"),
        ("Disable delete after reading", "未启用阅后即焚"),
        ("Font size", "字体大小"),
        ("Font family", "字体样式"),
        ("Choose language", "选择语言"),
        ("Sync time interval(minute)", "同步时间间隔(分钟)"),
        ("Please input time interval", "请输入时间间隔"),
        ("Sync timeout(second)", "同步超时(秒)"),
        ("Please input sync timeout", "请输入同步超时"),
        ("Enabled auto sync", "已启用自动同步"),
        ("Disable auto sync", "未启用自动同步"),
        (
            "Starting sync once, when application starting",
            "程序启动时，马上进行一次同步",
        ),
        (
            "Donot start sync, when application starting",
            "程序启动时，不马上进行一次同步",
        ),
        ("Proxy address", "代理地址"),
        ("Proxy port", "代理端口"),
        ("Warning", "警告"),
        ("RSS", "订阅"),
        ("Collection", "收藏夹"),
        ("Find", "发现"),
        ("Add", "添加"),
        ("Setting", "设置"),
        ("Remove from blacklist success", "成功移除黑名单"),
        ("No Data", "没有数据"),
        ("No Message", "没有消息"),
        ("Input keyword", "输入关键字"),
        ("Backup and recover", "备份与恢复"),
        ("API token", "API 令牌"),
        ("Please input API token", "请输入API令牌"),
        ("Backup and recover options", "备份与恢复选项"),
        ("RSS list", "RSS列表"),
        ("User setting", "用户设置"),
        ("Backup", "备份"),
        ("Recover", "恢复"),
        ("Backup success", "备份成功"),
        ("Backup failed", "备份失败"),
        ("Recover success", "恢复成功"),
        ("Recover failed", "恢复失败"),
        ("Backup or not?", "是否备份？"),
        ("Recover or not?", "是否恢复？"),
        ("Latest version", "获取最新版本"),
        ("Current version", "版本信息"),
        ("Latest version", "当前版本"),
        ("Update detail", "更新信息"),
        ("Download", "下载最新版本"),
        ("Choose Theme", "选择主题"),
        ("Light", "白天"),
        ("Dark", "黑暗"),
        ("Skip", "跳过"),
        ("Next", "下一步"),
        ("Finish", "完成"),
        ("Back", "返回"),
        ("Please select language", "请选择语言"),
        ("No record", "没有记录"),
        ("No address", "没有地址"),
        ("Please enter username", "请输入用户名"),
        ("Username", "用户名"),
        ("Please enter password", "请输入密码"),
        ("Please enter password again", "请再次输入密码"),
        ("At least 8 chars", "至少8个字符"),
        ("Create account", "创建账户"),
        ("Recover account", "恢复账户"),
        ("New mnemonic failed", "生成组记词失败"),
        ("Login", "登录"),
        (
            "Mnemonic counts is no correct. Only support 12 or 24 word counts mnemonic",
            "组记词数量不对，仅支持12和24个组记词",
        ),
        ("Invalid mnemonic", "非法组记词"),
        ("Username can not be empty", "用户名不能为空"),
        ("Two passwords is different", "密码不相同"),
        ("Password can not less than 8 chars", "密码不能小于8位"),
        ("Update account success", "更新账户成功"),
        (
            "Update account failed. The account do not exist",
            "更新账户失败. 账户不存在",
        ),
        ("Not allow delete current account", "不允许删除当前用户"),
        ("Not allow delete the main account", "不允许删除主账号"),
        ("Delete account success", "删除账户成功"),
        ("Remove account success", "移除账户成功"),
        ("Switch account success", "切换账户成功"),
        (
            "Switch account failed. The account do not exist",
            "切换账户失败. 账户不存在",
        ),
        (
            "Create account failed. Invalid password",
            "创建用户失败. 非法密码",
        ),
        ("Create account success", "创建用户成功"),
        ("Create account failed", "创建用户失败"),
        ("Account Management", "账户管理"),
        ("Delete account or not?", "是否删除账户？"),
        ("Internal error", "内部错误"),
        ("Password is wrong", "密码错误"),
        ("Mnemonic", "组记词"),
        ("Address Book", "地址簿"),
        ("Delete address", "移除地址"),
        ("Delete address or not?", "是否删除地址？"),
        ("Address name", "地址名称"),
        ("Account address", "账户地址"),
        ("Address", "地 址"),
        ("Delete address success", "删除地址成功"),
        ("Update address success", "更新地址成功"),
        ("Update address failed", "更新地址失败"),
        ("Switch account", "切换账户"),
        ("Delete all account or not?", "是否删除所有账户？"),
        ("Delete all accounts success", "删除所有账户成功"),
        ("Open link success", "打开成功"),
        ("Open link failed", "打开失败"),
        ("Security & Privacy ", "安全与隐私"),
        ("Reset account", "重置账户"),
        ("Reset account or not?", "是否重置账户？"),
        ("Please input old password", "请输入旧密码"),
        ("Please input new password", "请输入新密码"),
        ("Please input new password again", "请再次输入新密码"),
        ("Test model", "测试模式"),
        ("Main Network", "主网络"),
        ("Test Network", "测试网络"),
        ("Dev Network", "开发网络"),
        ("Unknown Network", "未知网络"),
        ("Warning: Current Network is ", "注意: 当前处于"),
        ("Home", "主页"),
        ("Recipient", "接收"),
        ("Recipient", "接收代币"),
        ("History", "历史"),
        ("History", "历史记录"),
        ("Developer Mode", "开发者模式"),
        ("Account name", "账户名称"),
        ("Show mnemonics", "显示组记词"),
        ("Remove account", "移除账户"),
        ("Change password", "更改密码"),
        ("mnemonics", "位组记词"),
        ("Refresh finished", "刷新完成"),
        ("Manage token", "管理代币"),
        ("Send token", "发送代币"),
        ("Fetch account tokens failed", "获取Token失败"),
        ("Refresh account balance success", "更新账户余额成功"),
        ("Request airdrop", "请求空投"),
        ("Request airdrop success", "请求空投成功"),
        ("Request airdrop failed", "请求空投失败"),
        ("It may takes a long time. Please wait...", "请耐心等待..."),
        ("Evaluating transaction fee failed", "估计交易费用失败"),
        ("Evaluating gas fee...", "正在估计交易费用..."),
        ("Transaction signature", "交易记录"),
        ("Blockchain network", "区块网络"),
        ("Send amount", "发送数量"),
        ("Transaction history", "交易记录"),
        ("Close", "关闭"),
        ("Transaction failed", "交易失败"),
        ("Waiting transaction confirmed...", "等待交易确认..."),
        ("Transaction detail", "交易信息"),
        ("Send address", "发送地址"),
        ("Recipient address", "接收地址"),
        ("Transaction fee", "交易费用"),
        ("Transaction has been confirmed", "交易已经确认"),
        ("Transaction success", "交易成功"),
        ("Invalid address", "非法地址"),
        ("Account", "账户"),
        ("Create token account fee", "创建账户费用"),
        ("Create mnemonic", "创建组记词"),
        ("Recover account", "恢复账户"),
        ("QrCode", "二维码"),
        ("Advance setting", "高级设置"),
        ("Memo", "备注"),
        ("Prioritization fee", "优先费用"),
        ("Base fee", "基础费用"),
        ("Max prioritization fee", "最大优先费用"),
        ("Max prioritization fee is", "最大优先费用为"),
        ("Slow", "慢"),
        ("Normal", "正常"),
        ("Fast", "快"),
        ("Enable system status bar", "启用系统状态栏"),
        ("Invalid prioritization fee", "非法优先费用"),
        (
            "Please setting max prioritization fee",
            "请设置更大的优先费用",
        ),
        ("Logout account", "登出账户"),
        ("Wrong password", "密码错误"),
        ("Internal error. Password not exist", "内部错误. 密码不存在"),
        (
            "Refresh account balance failed. The account is not found",
            "更新账户余额失败. 账户不存在",
        ),
        (
            "Create account and recover account from mnemonics",
            "创建账户和使用组记词恢复账户",
        ),
        (
            "Show, send and receive Sol and tokens of Solana",
            "查看、发送和接收Sol代币和Solana的通证",
        ),
        (
            "Welcome! Enjoying you journey of crypto",
            "欢迎使用，享受你的加密之旅",
        ),
    ]);

    if let Some(txt) = items.get(text) {
        return txt.to_string();
    }

    text.to_string()
}

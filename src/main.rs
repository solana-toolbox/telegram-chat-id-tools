use teloxide::{prelude::*, utils::command::BotCommands};
use dotenvy::dotenv;
use tracing_subscriber;

// 定义命令（可选，这里仅用一个简单的 /start 命令）
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "可用命令:")]
enum Command {
    #[command(description = "启动机器人")]
    Start,
}

#[tokio::main]
async fn main() {
    // 设置日志级别为最详细级别（TRACE）
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .init();
    dotenv().ok(); // 加载 .env 文件
    // 自动设置 HYPER_HTTP_KEEP_ALIVE_TIMEOUT 到系统环境变量，确保 hyper 能读取
    if let Ok(timeout) = std::env::var("HYPER_HTTP_KEEP_ALIVE_TIMEOUT") {
        unsafe {
            std::env::set_var("HYPER_HTTP_KEEP_ALIVE_TIMEOUT", timeout);
        }
    }
    tracing::info!("启动 Telegram Bot");
    // 从环境变量获取 Telegram Bot Token
    let bot = Bot::from_env();

    // 设置命令处理器、消息处理器和频道消息处理器
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_handler)
        )
        .branch(
            Update::filter_message()
                .endpoint(message_handler)
        )
        .branch(
            Update::filter_channel_post()
                .endpoint(channel_post_handler)
        );

    // 启动机器人
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

// 处理 /start 命令
async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            let chat_id = msg.chat.id;
            bot.send_message(msg.chat.id, format!("此聊天的 Chat ID 是：{}", chat_id))
                .await?;
        }
    }
    Ok(())
}

// 处理所有消息
async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    tracing::debug!("收到消息: {:?}", msg);
    let chat_id = msg.chat.id;
    // 确定聊天类型
    let chat = &msg.chat;
    let chat_type = if chat.is_private() {
        "私聊"
    } else if chat.is_group() {
        "群组"
    } else if chat.is_supergroup() {
        "超级群组"
    } else if chat.is_channel() {
        "频道"
    } else {
        "未知"
    };

    // 回复 Chat ID 和聊天类型
    bot.send_message(
        msg.chat.id,
        format!("此{}的 Chat ID 是：{}", chat_type, chat_id),
    )
    .await?;

    Ok(())
}

// 处理频道消息
async fn channel_post_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    tracing::debug!("收到频道消息: {:?}", msg);
    let chat_id = msg.chat.id;
    bot.send_message(
        chat_id,
        format!("此频道的 Chat ID 是：{}", chat_id),
    )
    .await?;
    Ok(())
}
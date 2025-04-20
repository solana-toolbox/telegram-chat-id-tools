# Telegram ChatID Tools

本项目是一个基于 Rust 和 [teloxide](https://github.com/teloxide/teloxide) 的 Telegram 机器人，用于查询各种聊天（私聊、群组、超级群组、频道）的 Chat ID。

## 功能简介
- 支持 /start 命令，回复当前聊天的 Chat ID
- 支持普通消息自动回复 Chat ID 和聊天类型
- 支持频道消息自动回复频道的 Chat ID（需将机器人加为管理员）
- 支持详细日志输出，便于调试
- 支持消息处理超时控制，加快机器人关闭速度

## 快速开始

### 1. 环境准备
- 安装 [Rust](https://www.rust-lang.org/tools/install)
- 安装 [Telegram](https://telegram.org/) 并创建自己的 Bot，获取 Token

### 2. 克隆代码
```bash
git clone <your_repo_url>
cd telegram-chatid-tools
```

### 3. 配置环境变量
在根目录下创建 `.env` 文件，内容如下：
```
TELOXIDE_TOKEN=你的bot token
```

### 4. 编译并运行
```bash
cargo build
# 推荐带日志和更快关闭：
HYPER_HTTP_KEEP_ALIVE_TIMEOUT=2 RUST_LOG=trace cargo run
```

### 5. 添加机器人到群组/频道
- 私聊：直接发送消息给机器人
- 群组/超级群组：将机器人添加为成员
- 频道：必须将机器人设置为管理员，并给予“发布消息”权限

## 主要文件说明
- `src/main.rs`：主程序，包含所有逻辑
- `.env`：存放 Telegram Bot Token
- `Cargo.toml`：Rust 依赖配置

## 重要说明
- 频道消息必须是通过机器人发布的，或机器人为管理员时才能收到。
- 若关闭机器人慢，可设置 `HYPER_HTTP_KEEP_ALIVE_TIMEOUT=2` 环境变量。
- 消息处理已加超时包裹，避免网络阻塞导致关闭缓慢。

## 日志与调试
- 日志级别可通过 `RUST_LOG` 环境变量控制，推荐 `trace` 以便调试。
- 若需完整 backtrace，可用 `RUST_BACKTRACE=full`。

## 依赖
- teloxide = "0.12"
- dotenvy = "0.15"
- tokio = { version = "1.38", features = ["full"] }
- tracing = "0.1.40"
- tracing-subscriber = "0.3.19"

## 常见问题
- **频道没有回复？** 请检查机器人是否为管理员，且有“发布消息”权限。
- **关闭慢？** 请设置 `HYPER_HTTP_KEEP_ALIVE_TIMEOUT=2`。
- **日志不详细？** 请设置 `RUST_LOG=trace`。

## License
MIT

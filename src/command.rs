use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "你好，欢迎使用此 bot\n\
                   该 bot 用于查询 JMS 相关的信息\n\n\
                   北京时间 20 号 12 点自动查询本月应缴额"
)]

pub enum Command {
    #[command(description = "显示该帮助信息")]
    Help,
    #[command(description = "查询当前汇率")]
    ExchangeRate,
    #[command(description = "查询当前流量")]
    DataUsed,
    #[command(description = "查询当前流量")]
    ChargeCNY,
}

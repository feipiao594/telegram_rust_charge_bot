# Tg_Charge_Notion_bot

## 支持平台

|           | Windows | Linux | MacOS |
| :-------: | :-----: | :---: | :---: |
| **amd64** |   Yes   |  Yes  |  Yes  |
| **arm64** |   Yes   |  Yes  |  Yes  |


## 简介
基于`mobot`的跨平台tgbot，负责订阅活动的查询与提醒

## 使用方式

### 配置文件
按照`config_template.json`的写法填写配置文件，填完后**重命名**为`config.json`放置在相同路径下

- subscribed_money: 订阅流量的月租
- subscribed_count: 订阅人数
- expand_rate: 额外的转换系数，通常为1.0
- bot_token: Bot的token
- group_chat_id: 群聊chatid
- used_data_url: 流量使用情况的查询api
- exchange_rate_url: 查询汇率的api
- event_trigger_time: 触发notion的时间，cron格式
- max_store_charge_num: 历史记录存储的最大条数
- subscribed_id_list: 数组，能使用该bot功能的人的chatid
- help_message: 键入 /help 命令后提示的前排文字

# SMSG_SERVER_FIRST_ACHIEVEMENT

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm#L8).
```rust,ignore
smsg SMSG_SERVER_FIRST_ACHIEVEMENT = 0x0498 {
    CString name;
    Guid player;
    u32 achievement;
    AchievementNameLinkType link_type;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | - / - | CString | name |  |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | player |  |  |
| - | 4 / Little | u32 | achievement |  |  |
| - | 1 / - | [AchievementNameLinkType](achievementnamelinktype.md) | link_type |  |  |


# SMSG_GUILD_COMMAND_RESULT

## Client Version 1

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:114`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L114).
```rust,ignore
smsg SMSG_GUILD_COMMAND_RESULT = 0x0093 {
    (u32)GuildCommand command;
    CString string;
    (u32)GuildCommandResult result;
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
| 0x04 | 4 / - | [GuildCommand](guildcommand.md) | command |  |  |
| 0x08 | - / - | CString | string |  |  |
| - | 4 / - | [GuildCommandResult](guildcommandresult.md) | result |  |  |

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:114`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L114).
```rust,ignore
smsg SMSG_GUILD_COMMAND_RESULT = 0x0093 {
    (u32)GuildCommand command;
    CString string;
    (u32)GuildCommandResult result;
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
| 0x04 | 4 / - | [GuildCommand](guildcommand.md) | command |  |  |
| 0x08 | - / - | CString | string |  |  |
| - | 4 / - | [GuildCommandResult](guildcommandresult.md) | result |  |  |

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:114`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L114).
```rust,ignore
smsg SMSG_GUILD_COMMAND_RESULT = 0x0093 {
    (u32)GuildCommand command;
    CString string;
    (u32)GuildCommandResult result;
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
| 0x04 | 4 / - | [GuildCommand](guildcommand.md) | command |  |  |
| 0x08 | - / - | CString | string |  |  |
| - | 4 / - | [GuildCommandResult](guildcommandresult.md) | result |  |  |


# SMSG_LFG_PLAYER_INFO

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm#L22).
```rust,ignore
smsg SMSG_LFG_PLAYER_INFO = 0x036F {
    u8 amount_of_available_dungeons;
    LfgAvailableDungeon[amount_of_available_dungeons] available_dungeons;
    u8 amount_of_locked_dungeons;
    LfgJoinLockedDungeon[amount_of_locked_dungeons] locked_dungeons;
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
| 0x04 | 1 / - | u8 | amount_of_available_dungeons |  |  |
| 0x05 | ? / - | [LfgAvailableDungeon](lfgavailabledungeon.md)[amount_of_available_dungeons] | available_dungeons |  |  |
| - | 1 / - | u8 | amount_of_locked_dungeons |  |  |
| - | ? / - | [LfgJoinLockedDungeon](lfgjoinlockeddungeon.md)[amount_of_locked_dungeons] | locked_dungeons |  |  |


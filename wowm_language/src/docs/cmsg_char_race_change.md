# CMSG_CHAR_RACE_CHANGE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/character_screen/cmsg_char_race_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_race_change.wowm#L1).
```rust,ignore
cmsg CMSG_CHAR_RACE_CHANGE = 0x04F8 {
    Guid player;
    CString name;
    Gender gender;
    u8 skin_color;
    u8 hair_color;
    u8 hair_style;
    u8 facial_hair;
    u8 face;
    Race race;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | player |  |  |
| 0x0E | - / - | CString | name |  |  |
| - | 1 / - | [Gender](gender.md) | gender |  |  |
| - | 1 / - | u8 | skin_color |  |  |
| - | 1 / - | u8 | hair_color |  |  |
| - | 1 / - | u8 | hair_style |  |  |
| - | 1 / - | u8 | facial_hair |  |  |
| - | 1 / - | u8 | face |  |  |
| - | 1 / - | [Race](race.md) | race |  |  |


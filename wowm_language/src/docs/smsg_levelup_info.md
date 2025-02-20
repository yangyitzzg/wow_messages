# SMSG_LEVELUP_INFO

## Client Version 1, Client Version 2

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm#L1).
```rust,ignore
smsg SMSG_LEVELUP_INFO = 0x01D4 {
    Level32 new_level;
    u32 health;
    u32 mana;
    u32 rage;
    u32 focus;
    u32 energy;
    u32 happiness;
    u32 strength;
    u32 agility;
    u32 stamina;
    u32 intellect;
    u32 spirit;
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
| 0x04 | 4 / Little | Level32 | new_level |  |  |
| 0x08 | 4 / Little | u32 | health |  |  |
| 0x0C | 4 / Little | u32 | mana |  |  |
| 0x10 | 4 / Little | u32 | rage |  |  |
| 0x14 | 4 / Little | u32 | focus |  |  |
| 0x18 | 4 / Little | u32 | energy |  |  |
| 0x1C | 4 / Little | u32 | happiness |  |  |
| 0x20 | 4 / Little | u32 | strength |  |  |
| 0x24 | 4 / Little | u32 | agility |  |  |
| 0x28 | 4 / Little | u32 | stamina |  |  |
| 0x2C | 4 / Little | u32 | intellect |  |  |
| 0x30 | 4 / Little | u32 | spirit |  |  |

## Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_levelup_info.wowm#L18).
```rust,ignore
smsg SMSG_LEVELUP_INFO = 0x01D4 {
    Level32 new_level;
    u32 health;
    u32 mana;
    u32 rage;
    u32 focus;
    u32 energy;
    u32 happiness;
    u32 rune;
    u32 runic_power;
    u32 strength;
    u32 agility;
    u32 stamina;
    u32 intellect;
    u32 spirit;
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
| 0x04 | 4 / Little | Level32 | new_level |  |  |
| 0x08 | 4 / Little | u32 | health |  |  |
| 0x0C | 4 / Little | u32 | mana |  |  |
| 0x10 | 4 / Little | u32 | rage |  |  |
| 0x14 | 4 / Little | u32 | focus |  |  |
| 0x18 | 4 / Little | u32 | energy |  |  |
| 0x1C | 4 / Little | u32 | happiness |  |  |
| 0x20 | 4 / Little | u32 | rune |  |  |
| 0x24 | 4 / Little | u32 | runic_power |  |  |
| 0x28 | 4 / Little | u32 | strength |  |  |
| 0x2C | 4 / Little | u32 | agility |  |  |
| 0x30 | 4 / Little | u32 | stamina |  |  |
| 0x34 | 4 / Little | u32 | intellect |  |  |
| 0x38 | 4 / Little | u32 | spirit |  |  |


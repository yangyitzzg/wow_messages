# SMSG_HIGHEST_THREAT_UPDATE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/threat/smsg_highest_threat_update.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_highest_threat_update.wowm#L8).
```rust,ignore
smsg SMSG_HIGHEST_THREAT_UPDATE = 0x0482 {
    PackedGuid unit;
    PackedGuid new_victim;
    u32 amount_of_units;
    ThreatUpdateUnit[amount_of_units] units;
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
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | unit |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | new_victim |  |  |
| - | 4 / Little | u32 | amount_of_units |  |  |
| - | ? / - | [ThreatUpdateUnit](threatupdateunit.md)[amount_of_units] | units |  |  |


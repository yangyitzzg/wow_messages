# SMSG_UPDATE_ACCOUNT_DATA_COMPLETE

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/login_logout/smsg_update_account_data_complete.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_update_account_data_complete.wowm#L1).
```rust,ignore
smsg SMSG_UPDATE_ACCOUNT_DATA_COMPLETE = 0x0463 {
    u32 data_type;
    u32 unknown1;
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
| 0x04 | 4 / Little | u32 | data_type |  |  |
| 0x08 | 4 / Little | u32 | unknown1 |  | mangostwo hardcodes this to 0 |


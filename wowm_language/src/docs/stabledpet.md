# StabledPet

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm#L3).
```rust,ignore
struct StabledPet {
    u32 pet_number;
    u32 entry;
    Level32 level;
    CString name;
    u32 loyalty;
    u8 slot;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | pet_number |  |  |
| 0x04 | 4 / Little | u32 | entry |  |  |
| 0x08 | 4 / Little | Level32 | level |  |  |
| 0x0C | - / - | CString | name |  |  |
| - | 4 / Little | u32 | loyalty |  |  |
| - | 1 / - | u8 | slot |  | vmangos/mangoszero/cmangos: client slot 1 == current pet (0) |


# SMSG_PAGE_TEXT_QUERY_RESPONSE

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/smsg_page_text_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_page_text_query_response.wowm#L3).
```rust,ignore
smsg SMSG_PAGE_TEXT_QUERY_RESPONSE = 0x005B {
    u32 page_id;
    CString text;
    u32 next_page_id;
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
| 0x04 | 4 / Little | u32 | page_id |  |  |
| 0x08 | - / - | CString | text |  |  |
| - | 4 / Little | u32 | next_page_id |  |  |


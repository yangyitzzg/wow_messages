# CMSG_COMPLAIN

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/cmsg_complain.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_complain.wowm#L26).
```rust,ignore
cmsg CMSG_COMPLAIN = 0x03C6 {
    SpamType complaint_type;
    Guid offender;
    if (complaint_type == MAIL) {
        u32 unknown1;
        u32 mail_id;
        u32 unknown2;
    }
    else if (complaint_type == CHAT) {
        u32 language;
        u32 message_type;
        u32 channel_id;
        u32 time;
        CString description;
    }
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
| 0x06 | 1 / - | [SpamType](spamtype.md) | complaint_type |  |  |
| 0x07 | 8 / Little | [Guid](../spec/packed-guid.md) | offender |  |  |

If complaint_type is equal to `MAIL`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0F | 4 / Little | u32 | unknown1 |  |  |
| 0x13 | 4 / Little | u32 | mail_id |  |  |
| 0x17 | 4 / Little | u32 | unknown2 |  |  |

Else If complaint_type is equal to `CHAT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x1B | 4 / Little | u32 | language |  |  |
| 0x1F | 4 / Little | u32 | message_type |  |  |
| 0x23 | 4 / Little | u32 | channel_id |  |  |
| 0x27 | 4 / Little | u32 | time |  |  |
| 0x2B | - / - | CString | description |  |  |

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/cmsg_complain.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_complain.wowm#L8).
```rust,ignore
cmsg CMSG_COMPLAIN = 0x03C7 {
    SpamType complaint_type;
    Guid offender;
    if (complaint_type == MAIL) {
        u32 unknown1;
        u32 mail_id;
        u32 unknown2;
    }
    else if (complaint_type == CHAT) {
        u32 language;
        u32 message_type;
        u32 channel_id;
        u32 time;
        CString description;
    }
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
| 0x06 | 1 / - | [SpamType](spamtype.md) | complaint_type |  |  |
| 0x07 | 8 / Little | [Guid](../spec/packed-guid.md) | offender |  |  |

If complaint_type is equal to `MAIL`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0F | 4 / Little | u32 | unknown1 |  |  |
| 0x13 | 4 / Little | u32 | mail_id |  |  |
| 0x17 | 4 / Little | u32 | unknown2 |  |  |

Else If complaint_type is equal to `CHAT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x1B | 4 / Little | u32 | language |  |  |
| 0x1F | 4 / Little | u32 | message_type |  |  |
| 0x23 | 4 / Little | u32 | channel_id |  |  |
| 0x27 | 4 / Little | u32 | time |  |  |
| 0x2B | - / - | CString | description |  |  |


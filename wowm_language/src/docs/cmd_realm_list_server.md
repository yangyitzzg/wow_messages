# CMD_REALM_LIST_Server

## Protocol Version 2, Protocol Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/login/cmd_realm/server.wowm:66`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L66).
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;
    u32 header_padding = 0;
    u8 number_of_realms;
    Realm[number_of_realms] realms;
    u16 footer_padding = 0;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | 2 / Little | u16 | size |  |  |
| 0x03 | 4 / Little | u32 | header_padding |  |  |
| 0x07 | 1 / - | u8 | number_of_realms |  |  |
| 0x08 | ? / - | [Realm](realm.md)[number_of_realms] | realms |  |  |
| - | 2 / Little | u16 | footer_padding |  |  |

### Examples

#### Example 1

```c
16, // opcode (16)
23, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, // number_of_realms: u8
0, 0, 0, 0, // [0].Realm.realm_type: RealmType PLAYER_VS_ENVIRONMENT (0)
0, // [0].Realm.flag: RealmFlag  NONE (0)
65, 0, // [0].Realm.name: CString
65, 0, // [0].Realm.address: CString
0, 0, 200, 67, // [0].Realm.population: Population RED_FULL (0x43c80000)
1, // [0].Realm.number_of_characters_on_realm: u8
0, // [0].Realm.category: RealmCategory DEFAULT (0x0)
2, // [0].Realm.realm_id: u8
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
#### Example 2

```c
16, // opcode (16)
23, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, // number_of_realms: u8
0, 0, 0, 0, // [0].Realm.realm_type: RealmType PLAYER_VS_ENVIRONMENT (0)
3, // [0].Realm.flag: RealmFlag  INVALID| OFFLINE (3)
65, 0, // [0].Realm.name: CString
65, 0, // [0].Realm.address: CString
0, 0, 200, 67, // [0].Realm.population: Population RED_FULL (0x43c80000)
1, // [0].Realm.number_of_characters_on_realm: u8
0, // [0].Realm.category: RealmCategory DEFAULT (0x0)
2, // [0].Realm.realm_id: u8
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
## Protocol Version 5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/login/cmd_realm/server.wowm:90`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L90).
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;
    u32 header_padding = 0;
    u8 number_of_realms;
    Realm[number_of_realms] realms;
    u16 footer_padding = 0;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | 2 / Little | u16 | size |  |  |
| 0x03 | 4 / Little | u32 | header_padding |  |  |
| 0x07 | 1 / - | u8 | number_of_realms |  |  |
| 0x08 | ? / - | [Realm](realm.md)[number_of_realms] | realms |  |  |
| - | 2 / Little | u16 | footer_padding |  |  |

## Protocol Version 6, Protocol Version 7

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/login/cmd_realm/server.wowm:100`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L100).
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;
    u32 header_padding = 0;
    u16 number_of_realms;
    Realm[number_of_realms] realms;
    u16 footer_padding = 0;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | 2 / Little | u16 | size |  |  |
| 0x03 | 4 / Little | u32 | header_padding |  |  |
| 0x07 | 2 / Little | u16 | number_of_realms |  |  |
| 0x09 | ? / - | [Realm](realm.md)[number_of_realms] | realms |  |  |
| - | 2 / Little | u16 | footer_padding |  |  |

## Protocol Version 8

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/login/cmd_realm/server.wowm:191`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L191).
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;
    u32 header_padding = 0;
    u16 number_of_realms;
    Realm[number_of_realms] realms;
    u16 footer_padding = 0;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | 2 / Little | u16 | size |  |  |
| 0x03 | 4 / Little | u32 | header_padding |  |  |
| 0x07 | 2 / Little | u16 | number_of_realms |  |  |
| 0x09 | ? / - | [Realm](realm.md)[number_of_realms] | realms |  |  |
| - | 2 / Little | u16 | footer_padding |  |  |

### Examples

#### Example 1

```c
16, // opcode (16)
22, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, 0, // number_of_realms: u16
0, // [0].Realm.realm_type: RealmType PLAYER_VS_ENVIRONMENT (0)
0, // [0].Realm.locked: Bool
3, // [0].Realm.flag: RealmFlag  INVALID| OFFLINE (3)
65, 0, // [0].Realm.name: CString
65, 0, // [0].Realm.address: CString
0, 0, 200, 67, // [0].Realm.population: Population RED_FULL (0x43c80000)
1, // [0].Realm.number_of_characters_on_realm: u8
0, // [0].Realm.category: RealmCategory DEFAULT (0x0)
2, // [0].Realm.realm_id: u8
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
#### Example 2

```c
16, // opcode (16)
27, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, 0, // number_of_realms: u16
0, // [0].Realm.realm_type: RealmType PLAYER_VS_ENVIRONMENT (0)
0, // [0].Realm.locked: Bool
4, // [0].Realm.flag: RealmFlag  SPECIFY_BUILD (4)
65, 0, // [0].Realm.name: CString
65, 0, // [0].Realm.address: CString
0, 0, 200, 67, // [0].Realm.population: Population RED_FULL (0x43c80000)
1, // [0].Realm.number_of_characters_on_realm: u8
0, // [0].Realm.category: RealmCategory DEFAULT (0x0)
2, // [0].Realm.realm_id: u8
1, // Version.major: u8
12, // Version.minor: u8
1, // Version.patch: u8
243, 22, // Version.build: u16
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```

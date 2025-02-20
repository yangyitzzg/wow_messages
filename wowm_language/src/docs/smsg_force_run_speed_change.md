# SMSG_FORCE_RUN_SPEED_CHANGE

## Client Version 1.12

### Description

Tells the client that the running speed has changed.

Client replies with [CMSG_FORCE_RUN_SPEED_CHANGE_ACK](./cmsg_force_run_speed_change_ack.md).

vmangos sends this message to the client being changed and [SMSG_SPLINE_SET_RUN_SPEED](./smsg_spline_set_run_speed.md) to others.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/smsg/smsg_force_run_speed_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_run_speed_change.wowm#L1).
```rust,ignore
smsg SMSG_FORCE_RUN_SPEED_CHANGE = 0x00E2 {
    PackedGuid guid;
    u32 move_event;
    f32 speed;
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
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |
| - | 4 / Little | u32 | move_event |  | cmangos/mangoszero/vmangos: set to 0<br/>cmangos/mangoszero/vmangos: moveEvent, NUM_PMOVE_EVTS = 0x39 |
| - | 4 / Little | f32 | speed |  |  |

### Examples

#### Example 1

##### Description

Force speed to 7

```c
0, 12, // size
226, 0, // opcode (226)
1, 6, // guid: PackedGuid
0, 0, 0, 0, // move_event: u32
0, 0, 224, 64, // speed: f32
```
## Client Version 2.4.3, Client Version 3

### Description

Tells the client that the running speed has changed.

Client replies with [CMSG_FORCE_RUN_SPEED_CHANGE_ACK](./cmsg_force_run_speed_change_ack.md).

vmangos sends this message to the client being changed and [SMSG_SPLINE_SET_RUN_SPEED](./smsg_spline_set_run_speed.md) to others.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/smsg/smsg_force_run_speed_change.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_run_speed_change.wowm#L30).
```rust,ignore
smsg SMSG_FORCE_RUN_SPEED_CHANGE = 0x00E2 {
    PackedGuid guid;
    u32 move_event;
    u8 unknown;
    f32 speed;
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
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |
| - | 4 / Little | u32 | move_event |  | cmangos/mangoszero/vmangos: set to 0<br/>cmangos/mangoszero/vmangos: moveEvent, NUM_PMOVE_EVTS = 0x39 |
| - | 1 / - | u8 | unknown |  | mangosone sets to 0<br/>mangosone: new 2.1.0 |
| - | 4 / Little | f32 | speed |  |  |


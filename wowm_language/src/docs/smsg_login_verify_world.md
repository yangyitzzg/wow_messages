# SMSG_LOGIN_VERIFY_WORLD

## Client Version 1.12

### Description

Message to the client that is has successfully logged into the world and that it should load the map and coordinates.

### Comment

The positions and orientations do not matter since they can be overwritten in the [SMSG_UPDATE_OBJECT](./smsg_update_object.md), but the map determines which map the client loads and this is not changeable in [SMSG_UPDATE_OBJECT](./smsg_update_object.md).

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm#L2).
```rust,ignore
smsg SMSG_LOGIN_VERIFY_WORLD = 0x0236 {
    Map map;
    Vector3d position;
    f32 orientation;
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
| 0x04 | 4 / - | [Map](map.md) | map |  |  |
| 0x08 | 12 / - | [Vector3d](vector3d.md) | position |  |  |
| 0x14 | 4 / Little | f32 | orientation |  |  |

### Examples

#### Example 1

```c
0, 22, // size
54, 2, // opcode (566)
0, 0, 0, 0, // map: Map EASTERN_KINGDOMS (0)
205, 215, 11, 198, // Vector3d.x: f32
53, 126, 4, 195, // Vector3d.y: f32
249, 15, 167, 66, // Vector3d.z: f32
0, 0, 0, 0, // orientation: f32
```
## Client Version 2.4.3

### Description

Message to the client that is has successfully logged into the world and that it should load the map and coordinates.

### Comment

The positions and orientations do not matter since they can be overwritten in the [SMSG_UPDATE_OBJECT](./smsg_update_object.md), but the map determines which map the client loads and this is not changeable in [SMSG_UPDATE_OBJECT](./smsg_update_object.md).

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm#L2).
```rust,ignore
smsg SMSG_LOGIN_VERIFY_WORLD = 0x0236 {
    Map map;
    Vector3d position;
    f32 orientation;
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
| 0x04 | 4 / - | [Map](map.md) | map |  |  |
| 0x08 | 12 / - | [Vector3d](vector3d.md) | position |  |  |
| 0x14 | 4 / Little | f32 | orientation |  |  |

## Client Version 3.3.5

### Description

Message to the client that is has successfully logged into the world and that it should load the map and coordinates.

### Comment

The positions and orientations do not matter since they can be overwritten in the [SMSG_UPDATE_OBJECT](./smsg_update_object.md), but the map determines which map the client loads and this is not changeable in [SMSG_UPDATE_OBJECT](./smsg_update_object.md).

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm#L2).
```rust,ignore
smsg SMSG_LOGIN_VERIFY_WORLD = 0x0236 {
    Map map;
    Vector3d position;
    f32 orientation;
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
| 0x04 | 4 / - | [Map](map.md) | map |  |  |
| 0x08 | 12 / - | [Vector3d](vector3d.md) | position |  |  |
| 0x14 | 4 / Little | f32 | orientation |  |  |


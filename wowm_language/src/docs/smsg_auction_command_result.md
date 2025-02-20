# SMSG_AUCTION_COMMAND_RESULT

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm:47`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm#L47).
```rust,ignore
smsg SMSG_AUCTION_COMMAND_RESULT = 0x025B {
    u32 auction_id;
    AuctionCommandAction action;
    if (action == BID_PLACED) {
        AuctionCommandResult result;
        if (result == OK) {
            u32 auction_outbid1;
        }
        else if (result == ERR_INVENTORY) {
            InventoryResult inventory_result;
        }
        else if (result == ERR_HIGHER_BID) {
            Guid higher_bidder;
            u32 new_bid;
            u32 auction_outbid2;
        }
    }
    else {
        AuctionCommandResultTwo result2;
        if (result2 == ERR_INVENTORY) {
            InventoryResult inventory_result2;
        }
        else if (result2 == ERR_HIGHER_BID) {
            Guid higher_bidder2;
            u32 new_bid2;
            u32 auction_outbid3;
        }
    }
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
| 0x04 | 4 / Little | u32 | auction_id |  |  |
| 0x08 | 4 / - | [AuctionCommandAction](auctioncommandaction.md) | action |  |  |

If action is equal to `BID_PLACED`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0C | 4 / - | [AuctionCommandResult](auctioncommandresult.md) | result |  |  |

If result is equal to `OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x10 | 4 / Little | u32 | auction_outbid1 |  |  |

Else If result is equal to `ERR_INVENTORY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x14 | 1 / - | [InventoryResult](inventoryresult.md) | inventory_result |  |  |

Else If result is equal to `ERR_HIGHER_BID`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x15 | 8 / Little | [Guid](../spec/packed-guid.md) | higher_bidder |  |  |
| 0x1D | 4 / Little | u32 | new_bid |  |  |
| 0x21 | 4 / Little | u32 | auction_outbid2 |  |  |

Else: 
| 0x25 | 4 / - | [AuctionCommandResultTwo](auctioncommandresulttwo.md) | result2 |  |  |

If result2 is equal to `ERR_INVENTORY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x29 | 1 / - | [InventoryResult](inventoryresult.md) | inventory_result2 |  |  |

Else If result2 is equal to `ERR_HIGHER_BID`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x2A | 8 / Little | [Guid](../spec/packed-guid.md) | higher_bidder2 |  |  |
| 0x32 | 4 / Little | u32 | new_bid2 |  |  |
| 0x36 | 4 / Little | u32 | auction_outbid3 |  |  |

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm:47`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm#L47).
```rust,ignore
smsg SMSG_AUCTION_COMMAND_RESULT = 0x025B {
    u32 auction_id;
    AuctionCommandAction action;
    if (action == BID_PLACED) {
        AuctionCommandResult result;
        if (result == OK) {
            u32 auction_outbid1;
        }
        else if (result == ERR_INVENTORY) {
            InventoryResult inventory_result;
        }
        else if (result == ERR_HIGHER_BID) {
            Guid higher_bidder;
            u32 new_bid;
            u32 auction_outbid2;
        }
    }
    else {
        AuctionCommandResultTwo result2;
        if (result2 == ERR_INVENTORY) {
            InventoryResult inventory_result2;
        }
        else if (result2 == ERR_HIGHER_BID) {
            Guid higher_bidder2;
            u32 new_bid2;
            u32 auction_outbid3;
        }
    }
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
| 0x04 | 4 / Little | u32 | auction_id |  |  |
| 0x08 | 4 / - | [AuctionCommandAction](auctioncommandaction.md) | action |  |  |

If action is equal to `BID_PLACED`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x0C | 4 / - | [AuctionCommandResult](auctioncommandresult.md) | result |  |  |

If result is equal to `OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x10 | 4 / Little | u32 | auction_outbid1 |  |  |

Else If result is equal to `ERR_INVENTORY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x14 | 1 / - | [InventoryResult](inventoryresult.md) | inventory_result |  |  |

Else If result is equal to `ERR_HIGHER_BID`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x15 | 8 / Little | [Guid](../spec/packed-guid.md) | higher_bidder |  |  |
| 0x1D | 4 / Little | u32 | new_bid |  |  |
| 0x21 | 4 / Little | u32 | auction_outbid2 |  |  |

Else: 
| 0x25 | 4 / - | [AuctionCommandResultTwo](auctioncommandresulttwo.md) | result2 |  |  |

If result2 is equal to `ERR_INVENTORY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x29 | 1 / - | [InventoryResult](inventoryresult.md) | inventory_result2 |  |  |

Else If result2 is equal to `ERR_HIGHER_BID`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x2A | 8 / Little | [Guid](../spec/packed-guid.md) | higher_bidder2 |  |  |
| 0x32 | 4 / Little | u32 | new_bid2 |  |  |
| 0x36 | 4 / Little | u32 | auction_outbid3 |  |  |

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm:109`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm#L109).
```rust,ignore
smsg SMSG_AUCTION_COMMAND_RESULT = 0x025B {
    u32 auction_id;
    AuctionCommandAction action;
    AuctionCommandResult result;
    if (result == ERR_INVENTORY) {
        InventoryResult inventory_result;
    }
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
| 0x04 | 4 / Little | u32 | auction_id |  |  |
| 0x08 | 4 / - | [AuctionCommandAction](auctioncommandaction.md) | action |  |  |
| 0x0C | 4 / - | [AuctionCommandResult](auctioncommandresult.md) | result |  |  |

If result is equal to `ERR_INVENTORY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x10 | 1 / - | [InventoryResult](inventoryresult.md) | inventory_result |  |  |


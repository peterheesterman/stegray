
# stegray

steg(anography) - array

I need an intermediate data abstraction for [steg](https://github.com/peterheesterman/steg).

This is a place to store thoughts about it until i build it. 

```
data: [header_type][header][message]
header_type: [u4]
header: 
  original_file_type: [u4],
  message_length: u32 <--this won't fit video though;
  shasum: [u8; ??]
message: [u8]
```

header_type, original_file_type enums

copied from steg:
```
Reduce for a format:
  - Payload -> byte array
  - byte array -> payload

Construct from a package:
  - byte array -> package with carrier type
  - package with carrier type -> byte array
```
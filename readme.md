# A rust z80 emulation library

* Should be cycle accurate. 
* Passes zexall except for:
```
bit n,<b,c,d,e,h,l,(hl),a>....  ERROR **** crc expected:5e020e98 found:d1c30f4c
<daa,cpl,scf,ccf>.............  ERROR **** crc expected:6d2dd213 found:9b4ba675
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
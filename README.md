# NESmulator

A Nintendo Entertainment System (NES / Famicom) emulator written in Rust.

The MOS 6502 CPU, the 2C02 PPU, the cartridge bus, and the standard controller are implemented from scratch. Rendering and input go through SDL2.

## Status

- 6502 core: full official instruction set + the common unofficial opcodes (`*LAX`, `*SAX`, `*DCP`, `*ISB/ISC`, `*SLO`, `*SRE`, `*RLA`, `*RRA`, `*ANC`, `*ALR`, `*ARR`, `*AXS`, `*SBC`, all `*NOP` forms). Indirect JMP page-cross bug honored.
- PPU: background + sprite rendering, palette mirroring, nametable mirroring (horizontal/vertical), VRAM 0x2000–0x3EFF range, OAM DMA, VBlank NMI, basic sprite-zero hit.
- Bus: 2 KB internal RAM with mirrors, PPU register window with mirrors, OAM DMA at 0x4014, joypad 1 at 0x4016, NROM (mapper 0) PRG ROM mapping.
- Input: standard NES controller (player 1).
- Output: SDL2 window at 3× scale.

Not yet implemented: APU (audio), mappers other than NROM, second controller, save states.

## Building

Requires a Rust toolchain (1.70+) and `cmake` (used to build SDL2 from source via the bundled feature, so no system SDL2 dev package is required).

```bash
cargo build --release
```

The first build pulls and compiles SDL2 statically; subsequent builds are fast.

## Running

```bash
cargo run --release -- path/to/rom.nes
```

If no ROM path is given, the emulator defaults to `src/samples/Balloon Fight (USA).nes`.

A few sample ROMs are included under `src/samples/` for quick testing.

## Controls

| Key      | NES button |
| -------- | ---------- |
| Arrows   | D-pad      |
| `A`      | A          |
| `S`      | B          |
| `Return` | Start      |
| `Space`  | Select     |
| `Esc`    | Quit       |

## Screenshots

<img width="777" height="803" alt="image" src="https://github.com/user-attachments/assets/118b94e3-a23a-4b2a-8b79-9aa1326cf6ca" />

## Project Layout

```
src/
  cpu/        6502 core (instructions, opcode table, dispatch)
  ppu/        2C02 PPU (registers, address latch, scroll, mask, status)
  bus/        Memory map and tick coordination
  rom/        iNES 1.0 loader
  render/     Frame buffer and tile/sprite rasterizer
  joypad.rs   Standard controller
  main.rs     SDL2 host (window, texture, input, gameloop)
tests/
  cpu_tests.rs
  ppu_tests.rs
```

## Tests

```bash
cargo test
```

Covers CPU edge cases (ZeroPageY, ASL accumulator flags, SBC borrow/overflow, indirect JMP page bug, JSR/RTS roundtrip) and PPU VRAM read/write/mirroring/OAM behavior.

## License

MIT.

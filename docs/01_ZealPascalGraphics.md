# 📘 PRD: Zeal Pascal Graphics & Video Subsystem Integration

## Version 1.0 — Complete Graphics Support for the Zeal Video Board (ZVB)

This PRD defines how ZealPascal will support **all graphics features** of the Zeal Video Board (ZVB), covering:

* Pascal unit definitions for graphics, sprites, tilemaps, palettes, DMA, SPI, audio.
* Translation of ZVB C SDK headers into Pascal.
* Memory‑mapped register access and MMU paging model.
* Example Pascal programs demonstrating usage.
* Compiler/runtime intrinsics for performance‑critical operations.

The goal is to make ZealPascal the **first‑class graphics language for the Zeal 8‑bit Computer**, offering API clarity similar to Turbo Pascal’s Graph unit, but powered by a modern FPGA-based GPU.

---

# 1. Architecture Overview

The Zeal Video Board (ZVB) is an FPGA-based subsystem providing:

* Graphics modes (text, tilemap, bitmap)
* Sprite engine
* Palettes
* Tile/character VRAM
* Bitmap planes
* Audio generator
* SPI/CRC peripherals

All video memory and registers are **external** and accessed via memory-mapped I/O or Zeal OS wrappers.

ZealPascal must:

1. Provide **high-level Pascal APIs** for all ZVB functionality.
2. Provide **low-level register access** via intrinsics.
3. Handle **MMU mappings** transparently when needed.
4. Support **bare-metal and OS-based execution**.

---

# 2. Pascal Unit Definitions

ZealPascal introduces the following units:

## 2.1 `ZVB` (Core Video Board Unit)

Provides initialization and mode control.

```
unit ZVB;
interface
  procedure Init;
  procedure SetMode(mode: Byte);
  procedure EnableDisplay(flag: Boolean);
  procedure WaitVBlank;
implementation
end.
```

## 2.2 `ZVB_Graphics`

Tilemaps, tilesets, bitmap drawing, screen control.

```
unit ZVB_Graphics;
interface
  procedure LoadTileset(data: Pointer; count: Word);
  procedure DrawTile(x, y: Word; id: Word);
  procedure ClearScreen(color: Byte);
  procedure SetTilemap(page: Byte; addr: Word);
  procedure BlitBitmap(src: Pointer; width, height: Word; x, y: Word);
implementation
end.
```

## 2.3 `ZVB_Sprites`

Sprite definitions and updates.

```
unit ZVB_Sprites;
interface
  procedure SpriteSet(id: Byte; x, y: Word; tile: Byte; pal: Byte);
  procedure SpriteShow(id: Byte);
  procedure SpriteHide(id: Byte);
  procedure LoadSpriteSheet(data: Pointer; count: Word);
implementation
end.
```

## 2.4 `ZVB_Palette`

Palette RAM control.

```
unit ZVB_Palette;
interface
  procedure SetColor(index: Byte; rgb: Word);
  procedure FadeTo(target: Pointer);
implementation
end.
```

## 2.5 `ZVB_DMA`

Expose DMA helpers.

```
unit ZVB_DMA;
interface
  procedure DMACopy(src, dest: Word; size: Word);
  procedure DMAFill(dest: Word; size: Word; value: Byte);
implementation
end.
```

## 2.6 `ZVB_SPI`

Access to SPI peripheral for external devices.

```
unit ZVB_SPI;
interface
  procedure SPIWrite(b: Byte);
  function SPIRead: Byte;
implementation
end.
```

## 2.7 `ZVB_Audio`

Sound effects and music.

```
unit ZVB_Audio;
interface
  procedure SetTone(channel: Byte; freq: Word; vol: Byte);
  procedure PlaySample(data: Pointer; size: Word);
implementation
end.
```

---

# 3. Translation of SDK C Headers to Pascal

The Zeal VideoBoard SDK includes headers such as:

* `zvb_gfx.h`
* `zvb_sprite.h`
* `zvb_sound.h`
* `zvb_spi.h`
* `zvb_crc.h`

These define constants and register maps.

### Example C → Pascal Translation

**C header:**

```
#define ZVB_MODE_TEXT   0
#define ZVB_MODE_TILE   1
#define ZVB_MODE_BITMAP 2
```

**Pascal translation:**

```
const
  ZVB_MODE_TEXT   = 0;
  ZVB_MODE_TILE   = 1;
  ZVB_MODE_BITMAP = 2;
```

### Register Mapping

C:

```
#define ZVB_REG_MODE   0x8000
#define ZVB_REG_CTRL   0x8001
```

Pascal:

```
const
  ZVB_REG_MODE = $8000;
  ZVB_REG_CTRL = $8001;
```

Access helper:

```
procedure WriteReg(addr: Word; value: Byte); intrinsic;
```

---

# 4. MMU Mapping Model for Pascal

ZVB memory is external; ZealPascal must support:

* MMU page switching
* Video RAM banking
* Access to tilemap/sprite VRAM

Low-level intrinsic:

```
procedure MapPage(slot: Byte; page: Byte); intrinsic;
```

High-level helper:

```
procedure ZVB_MapTileRAM(page: Byte);
procedure ZVB_MapSpriteRAM(page: Byte);
```

The compiler may optimize repeated `MapPage` sequences.

MMU interaction is always safe because ZealPascal:

* Tracks active pages via runtime state
* Emits warnings for illegal writes

---

# 5. Example Pascal Programs

## 5.1 Hello World (Graphics Mode)

```
program HelloGFX;
uses ZVB, ZVB_Graphics, ZVB_Palette;

begin
  ZVB.Init;
  ZVB.SetMode(ZVB_MODE_TILE);
  ZVB_Palette.SetColor(1, $1F<<10);
  ZVB_Graphics.DrawTile(10, 10, 5);
  while True do ZVB.WaitVBlank;
end.
```

## 5.2 Sprite Demo

```
program SpriteTest;
uses ZVB, ZVB_Sprites;

begin
  ZVB.Init;
  ZVB.SetMode(ZVB_MODE_TILE);

  SpriteSet(0, 50, 50, 3, 0);
  SpriteShow(0);

  while True do begin
    Inc(x); SpriteSet(0, x, 50, 3, 0);
    ZVB.WaitVBlank;
  end;
end.
```

## 5.3 Bitmap Blit Demo

```
program BitmapTest;
uses ZVB, ZVB_Graphics;

begin
  ZVB.Init;
  ZVB.SetMode(ZVB_MODE_BITMAP);
  ZVB_Graphics.BlitBitmap(@Logo, 64, 64, 100, 80);
  while True do ZVB.WaitVBlank;
end.
```

---

# 6. Compiler & Runtime Intrinsics

The following intrinsics must be directly supported:

### 6.1 Register Access

```
procedure WriteReg(addr: Word; value: Byte); intrinsic;
function ReadReg(addr: Word): Byte; intrinsic;
```

### 6.2 DMA Intrinsics

```
procedure DMA_Copy(src, dest: Word; size: Word); intrinsic;
```

### 6.3 MMU Page Mapping

```
procedure MapPage(slot, page: Byte); intrinsic;
```

### 6.4 Video Synchronization

```
procedure WaitVBlank; intrinsic;
```

Compiler recognizes these and emits optimal machine code.

---

# 7. Ensuring Full Graphics Coverage

ZealPascal must support **all ZVB features**:

### Tile Engine

* Multiple tile layers
* Tilemap scrolling
* Tileset loading
* Palette swapping

### Sprite Engine

* Up to full hardware sprite count
* Sprite priorities and palettes
* Sprite RAM allocation helpers

### Bitmap Engine

* Direct framebuffer modes
* Blitting
* Line/rect primitives

### Palettes

* Full palette RAM control
* Fades and palette animation

### Audio

* Tone channels
* Sample playback
* Music engine (optional)

### Peripherals (SPI / CRC)

Fully exposed via units.

---

# 8. ZVB Performance Guidelines

These guidelines help developers write performant graphics code on ZealPascal that respects the Z80’s limits while leveraging the ZVB’s strengths.

## 8.1 General Principles

* **Move control, not pixels**: let ZVB do the heavy lifting (tile/sprite changes) instead of redrawing bitmaps every frame.
* **Exploit hardware scrolling**: use tilemap scrolling and sprite movement rather than per-pixel redraw.
* **Batch VRAM updates**: perform grouped updates during VBlank or in well-defined update windows.
* **Avoid redundant MMU switches**: group VRAM operations by page to minimize mapping overhead.

## 8.2 Tile & Sprite Usage

* Prefer **tile mode** for backgrounds and static scenery.
* Use sprites for:

    * Dynamic entities (player, enemies, bullets)
    * UI elements that move or animate
* Avoid excessively large sprites when small tiles + composition suffice.
* Preload tilesets/sprites into VRAM during loading screens or state transitions.

## 8.3 Cycle Cost Considerations

* Register writes to ZVB are cheap compared to full bitmap blits.
* DMA is preferred for large copies/fills (offloads repetitive work).
* Avoid tight loops writing to VRAM in bitmap mode where possible; use DMA or bulk copy routines.

ZealPascal docs will ship with concrete benchmarks and recommended limits (e.g., max sprite changes per frame, safe blit sizes).

---

# 9. Game Engine Layer Specification

On top of ZVB units, ZealPascal will provide an optional **Game Engine Layer** for higher-level game development.

## 9.1 Core Concepts

* **Scene**: a collection of entities and systems.
* **Entity**: a logical object with position, sprite/tiles, and behavior.
* **Component** (lightweight): position, velocity, sprite, collider.
* **System**: processes specific components (render, physics, input).

## 9.2 Scene Graph

A simple hierarchical model:

* Root scene
* Child nodes for layers (background, midground, foreground, UI)
* Entities attached to layers with z-order/palette info

APIs:

```pascal
procedure SceneAddEntity(e: TEntity);
procedure SceneRemoveEntity(e: TEntity);
procedure SceneUpdate(dt: Integer);
procedure SceneRender;
```

## 9.3 Sprite Batching

The engine collects all visible sprites for the frame, sorts by:

* Tileset
* Palette
* Layer

Then issues compact batches of `SpriteSet`/`SpriteShow` calls to minimize:

* State changes
* MMU switches
* Redundant register writes

## 9.4 Collision System

A simple AABB (axis-aligned bounding box) collision layer:

* Each `TEntity` has optional collider
* Spatial partitioning (grid-based) to avoid O(n²)
* Callback-based collision handling

APIs:

```pascal
procedure AddCollider(e: TEntity; rect: TRect);
procedure CheckCollisions;
```

---

# 10. Graphics Optimizer (Compiler-Level)

The ZealPascal compiler will include peephole optimizations specifically for:

* VRAM register access
* MMU mapping
* DMA usage

## 10.1 Register Write Folding

Multiple consecutive writes to the same ZVB register are folded:

* Remove redundant writes when values are unchanged
* Combine adjacent writes into small loops where possible

## 10.2 MMU Mapping Optimization

The compiler/runtime tracks current mapped pages and:

* Avoids generating `MapPage` calls when the desired page is already active
* Reorders non-dependent VRAM operations to minimize remapping

## 10.3 DMA Promotion

Patterns like:

```pascal
for i := 0 to N do
  VideoBuf[Base + i] := Src[i];
```

may be recognized and replaced with a `DMA_Copy` intrinsic when safe.

---

# 11. Asynchronous I/O and Double Buffering

To reduce tearing and provide smooth animation, ZealPascal will support double buffering strategies.

## 11.1 Double Buffering Models

### 11.1.1 Tilemap Double Buffering

* Maintain two tilemaps in VRAM.
* Render updates to the inactive tilemap.
* Swap tilemap base address or bank at VBlank.

### 11.1.2 Bitmap Double Buffering

* Allocate two bitmap buffers.
* Draw to back buffer while front is displayed.
* Swap buffer pointers at VBlank via a single register write.

## 11.2 Asynchronous Operations

While Zeal is single-CPU, we can simulate async patterns:

* Issue DMA transfers early in frame
* Use `WaitVBlank` as sync point
* Design API to express “submit then present” semantics

APIs:

```pascal
procedure BeginFrame;
procedure EndFrame;  // swaps buffers / completes DMA
```

---

# 12. Pascal DSL for Tilemap Creation

To streamline content authoring, ZealPascal will include a small **embedded DSL** for tilemap definitions.

### 12.1 DSL Goals

* Allow tilemaps to be defined in code with clear structure.
* Enable compile-time generation of tile data.
* Support macros for repeated patterns.

### 12.2 Example DSL Syntax

```pascal
const
  Level1: TTileMap = (
    Width: 16; Height: 12;
    Data: (
      ROW('################'),
      ROW('#..............#'),
      ROW('#..####........#'),
      ...
    )
  );
```

Where `#` and `.` are mapped to tile IDs via a symbol table:

```pascal
TileLegend: array['#'..'.'] of Byte = (
  ['#'] := TILE_WALL,
  ['.'] := TILE_FLOOR
);
```

The compiler expands this into static arrays of tile indices.

---

# 13. Integration with ZealIDE Asset Pipeline

ZealIDE (Zepto) will be extended to manage graphics assets and feed them into ZealPascal projects.

## 13.1 Asset Types

* Tilesets (PNG/BMP → tile data)
* Spritesheets
* Tilemaps (from external editors like Tiled)
* Palettes
* Audio samples/music

## 13.2 Build Pipeline

ZealIDE will:

1. Convert source art (e.g., PNG) into tile and sprite data with ZVB-friendly formats.
2. Generate Pascal include files:

   ```pascal
   const
     PlayerTiles: array[0..NNN] of Byte = (...);
   ```
3. Place binary assets in ROM/FS and expose offsets/sizes.
4. Regenerate assets when source art changes.

## 13.3 Project Configuration

Within ZealIDE:

* Define asset directories.
* Configure tile size, color depth, palette format.
* Configure tilemap import (e.g., Tiled JSON → Pascal DSL or binary map).

## 13.4 Editor Integration

ZealIDE can show:

* Tileset preview
* Sprite preview and animation
* Tilemap preview
* Auto-gen Pascal constants for tile IDs and map names

---

# 14. Final Summary

With performance guidelines, a higher-level game engine layer, graphics-specific compiler optimizations, double buffering and pseudo-async I/O, a Pascal-based tilemap DSL, and ZealIDE asset pipeline integration, ZealPascal Graphics moves beyond a thin wrapper over ZVB.

It becomes a **full, opinionated, and productive graphics stack**:

* Low-level control when you need it.
* High-level ergonomics when you don’t.
* Fully aligned with Zeal’s architecture and Z80’s constraints.

This positions ZealPascal not only as a language but as a **complete game and graphics development environment** for the Zeal 8-bit Computer.

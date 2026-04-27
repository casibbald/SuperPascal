# 📘 PRD: ZealPascal Game Engine

## Version 1.0 — Specification for High‑Level Game Framework on the Zeal 8‑bit Computer

This document defines the architecture, API, performance budget, and demo/test suite for the **ZealPascal Game Engine**, a high‑level framework built atop the ZealPascal graphics stack and the Zeal Video Board (ZVB).

The purpose of the ZealPascal Game Engine is to provide:

* A clean, structured, and idiomatic Pascal API for game development.
* Predictable and optimized update + render loops.
* Scene graphs, entities, components, events.
* Built‑in collision and movement systems.
* Asset and animation management.
* Benchmarked performance guidelines.
* A standard demo suite validating all core features.

The engine is designed to support:

* Tile‑based games (platformers, RPGs, puzzle)
* Sprite‑heavy games (shooters, shmups)
* Hybrid tile+sprite games

---

# 1. Architectural Overview

The ZealPascal Game Engine consists of:

* **TGame** — main game controller and loop
* **TScene** — collection of entities + rendering layers
* **TEntity** — base class for game objects
* **Components** — Position, Sprite, Collider, Script logic
* **Rendering Pipeline** — sprite batching + tilemap renderer
* **Physics/Collision Engine** — grid‑based or AABB
* **Event System** — input, collisions, custom game events
* **Resource Manager** — handles tilesets, sprites, animations, audio

The engine is designed to run at:

* 60 FPS (preferred)
* 30 FPS (fallback for heavy scenes)

---

# 2. Concrete API Definition

Below is the formal Pascal API for the engine.

## 2.1 TGame

Controls initialization, scene management, and main loop.

```pascal
type
  TGame = class
  private
    CurrentScene: TScene;
  public
    procedure Init;
    procedure Run;
    procedure SwitchScene(NewScene: TScene);
  end;
```

### Main Loop

```pascal
procedure TGame.Run;
begin
  while True do
  begin
    CurrentScene.Update;
    CurrentScene.Render;
    ZVB.WaitVBlank;
  end;
end;
```

---

## 2.2 TScene

A scene contains:

* Tilemap layer(s)
* Entities
* Collision grid
* Render order/layers

```pascal
type
  TScene = class
  private
    Entities: array of TEntity;
    Tilemap: PTilemap;
  public
    procedure Init;
    procedure Update;
    procedure Render;
    procedure AddEntity(e: TEntity);
    procedure RemoveEntity(e: TEntity);
  end;
```

---

## 2.3 TEntity

Every game object inherits from TEntity.

```pascal
type
  TEntity = class
  public
    X, Y: Integer;
    VX, VY: Integer;  // velocity
    SpriteID: Byte;
    Visible: Boolean;

    procedure Start; virtual;
    procedure Update; virtual;
    procedure Render; virtual;
  end;
```

### Default behaviors

* `Update` adjusts position by velocity.
* `Render` issues `SpriteSet` + `SpriteShow`.

---

## 2.4 Components (Optional Add‑On)

### Position Component

```pascal
record
  TPositionComponent = record
    X, Y: Integer;
  end;
```

### Sprite Component

```pascal
record
  TSpriteComponent = record
    SpriteID: Byte;
    Frame: Byte;
    AnimationSpeed: Byte;
  end;
```

### Collider Component

```pascal
record
  TCollider = record
    X, Y, W, H: Integer;
  end;
```

---

# 3. Mini‑Engine Specification: Platformer & Shmup Frameworks

This section defines two reference frameworks built atop the engine API.

---

## 3.1 Platformer Framework

### Features

* Tilemap collision (solid tiles)
* Gravity + jumping physics
* Entity‑to‑tile collision resolution
* Animated sprites
* Camera following player

### Core API

```pascal
procedure ApplyGravity(e: TEntity);
procedure HandleTileCollisions(e: TEntity; map: PTilemap);
procedure CameraFollow(target: TEntity);
```

### Player Entity Example

```pascal
type
  TPlayer = class(TEntity)
  public
    procedure Update; override;
  end;
```

### State Machine

* Idle
* Running
* Jumping
* Falling

### Physics

* Fixed timestep for predictable jump arc

---

## 3.2 Shmup Framework (Shoot‑'em‑up)

### Features

* Hundreds of bullets using sprite batching
* Enemy waves and spawn scripting
* Collision via spatial grid
* Bullet patterns using parametric functions

### Bullet Entity

```pascal
type
  TBullet = class(TEntity)
  public
    Damage: Byte;
    procedure Update; override;
  end;
```

### Bullet Manager

* Recycles bullets via object pool

---

# 4. Performance Targets

These values guide optimization and determine engine constraints.

## 4.1 Sprite Budget

* Target: **64 sprites** stable
* Stretch goal: **96 sprites** with optimized code

Sprite operations per frame:

* ≤ 64 `SpriteSet`
* ≤ 64 `SpriteShow`
* ≤ 16 sprite animation updates

## 4.2 Tilemap Updates

* ≤ 128 tile changes per frame in 60 FPS
* Unlimited tile changes during loading screens

## 4.3 Bitmap Blits

* Small blits (≤ 64×64): unlimited per frame
* Large blits (≥ 128×128): use DMA only

## 4.4 CPU Budget

* Update loop: < 60% of frame time
* Render loop: < 30% of frame time
* VBlank sync: remaining margin

---

# 5. Test & Demo Suite

To validate the ZealPascal Game Engine, a canonical demo suite shall be produced.

## 5.1 Demo 1: Sprite Stress Test

* 100+ moving sprites
* Tests sprite batching & performance budget
* Displays FPS counter

## 5.2 Demo 2: Tilemap Scroller

* Smooth horizontal scrolling
* Parallax layer (optional)
* Controlled tile updates

## 5.3 Demo 3: Platformer Demo

* Player movement, jumping, collisions
* Animated sprites
* Camera follow

## 5.4 Demo 4: Shmup Demo

* Bullet patterns
* Enemy paths
* Particle-like effects via sprite pool

## 5.5 Demo 5: Palette & Effects Demo

* Palette cycling
* Fades
* Lightning/flashes via palette manipulation

---

# 6. Engine Runtime Specification

This section defines the low-level runtime architecture for scenes, entities, pools, update loops, and the event system.

---

## 6.1 Overall Memory Architecture

The ZealPascal Game Engine uses page-oriented memory structures that align with Zeal OS and the ZVB’s VRAM mapping.

### 6.1.1 Entity Memory Pools

Entities are stored in contiguous pools:

* Pool resides inside one or more 16 KB pages.
* Allocation uses bump-pointer or free-list model.
* Pools support recycling for high-turnover entities (bullets, particles).

```
[TEntity #0][TEntity #1][TEntity #2]...[FreeList]
```

Each entity contains:

* Position + velocity
* Component bitmask
* Pointers to sprite/animation
* Collider (optional)

### 6.1.2 Scene Memory

Scenes contain:

* Entity pool pointer
* Tilemap pointer
* Collision grid pointer
* Optional background layers

Scenes may allocate additional pages when adding many entities or using large tilemaps.

### 6.1.3 Update Lists

There are 3 main lists:

* `UpdateList` → entities that need per-frame updates
* `RenderList` → entities with visual components
* `PhysicsList` → entities with colliders or motion

These lists are maintained automatically when entities are added/removed.

---

## 6.2 Update Scheduling

The update loop is structured for predictable performance.

```
procedure TScene.Update;
begin
  // 1. Input system
  ProcessInput;

  // 2. Physics update
  for e in PhysicsList do
    e.UpdatePhysics;

  // 3. Logic update
  for e in UpdateList do
    e.Update;

  // 4. Collision pass
  CheckCollisions;

  // 5. Late update
  for e in UpdateList do
    e.LateUpdate;
end;
```

LateUpdate is used for post-collision adjustments or camera sync.

---

## 6.3 Event Bus Design

Events connect decoupled systems.

### 6.3.1 Event Structure

```
EventType: Byte;
Sender: TEntity;
Payload: Pointer;
```

### 6.3.2 Event Queue

Fixed-size ring buffer:

```
PushEvent(e)
PollEvent → returns next event or nil
```

### 6.3.3 Event Types

* `EV_COLLISION`
* `EV_HIT`
* `EV_ANIM_COMPLETE`
* `EV_SPAWN`
* Custom events

### 6.3.4 Event Dispatch

```
while EventAvailable do
  e := PollEvent;
  e.Sender.HandleEvent(e);
```

Entities may override `HandleEvent`.

---

# 7. Animation System PRD

The animation subsystem provides sprite animation and state-driven behavior.

---

## 7.1 Keyframe Sequences

### 7.1.1 Data Structure

```
TKeyframe = record
  TileID: Byte;
  Duration: Byte; // frames
end;

TAnimation = record
  Frames: ^TKeyframe;
  Count: Byte;
  Loop: Boolean;
end;
```

### 7.1.2 Playback

Each entity stores:

* Current frame index
* Time accumulator
* Pointer to TAnimation

### 7.1.3 Update Rule

```
Time += 1;
if Time >= Frames[Cur].Duration then
  Cur := (Cur+1) mod Count;
  Time := 0;
```

If not `Loop`, animation freezes at last frame.

---

## 7.2 Sprite Sheets

Sprite sheets are contiguous blocks of tiles.

### 7.2.1 Sprite Sheet Descriptor

```
TSpriteSheet = record
  BaseTileID: Word;
  Count: Word;
  TileWidth, TileHeight: Byte;
end;
```

### 7.2.2 Loading API

```
procedure LoadSpriteSheet(data: Pointer; count: Word): TSpriteSheet;
```

---

## 7.3 State-Driven Animation

Entities may define animation states:

```
Idle -> Run -> Jump -> Fall -> Land
```

### 7.3.1 State Table

```
TAnimState = (asIdle, asRun, asJump, asFall);

TStateAnim = record
  State: TAnimState;
  Anim: TAnimation;
end;
```

### 7.3.2 Integration with Update

```
if VX = 0 then SetState(asIdle)
else SetState(asRun);
```

Each call to SetState updates the `CurrentAnimation` pointer.

---

# 8. Asset Pipeline Extensions

ZealIDE must integrate fully with the game engine.

---

## 8.1 Auto-Generating Pascal Animation Structs

ZealIDE parses spritesheet metadata (e.g., JSON/XML) and produces:

```
const
  PlayerRun: TAnimation = (
    Frames: @PlayerRunFrames;
    Count: 4;
    Loop: True;
  );
```

Frame durations may be included.

---

## 8.2 Spritesheet Packing Rules

Rules for optimal packing of sprites:

* Pack sprites by animation state to reduce tile bank switching.
* Keep tile widths multiples of 8.
* Sprite sheets should not cross ZVB VRAM bank boundaries.
* Prefer grouping by palette.

ZealIDE enforces or warns about violations.

---

## 8.3 Tile Optimizations

Tile deduplication:

* Remove duplicate tiles automatically.
* Build a tile atlas.

Palette optimizations:

* Group artwork by palette to minimize runtime palette swaps.

---

# 9. Compiler Hooks for Engine Optimization

The compiler can optimize engine usage patterns.

---

## 9.1 Inlining Heuristics for Update

The compiler may automatically inline:

* Small `Update` methods
* Sprite update routines
* Collision functions

Heuristics:

* Function < 16 instructions → inline
* Hot call paths are prioritized

---

## 9.2 VBlank-Aware Rendering Hints

The compiler inserts scheduling hints:

* Group VRAM writes early in the frame
* Defer large blits until safe windows
* Suggest double-buffering when BM blits exceed thresholds

Optional pragma:

```
{$VBLANK_AWARE ON}
```

---

## 9.3 Loop Unrolling for Sprite Batching

Patterns like:

```
for i := 0 to 15 do
  SpriteSet(i, ...);
```

may be unrolled into:

```
SpriteSet(0, ...);
SpriteSet(1, ...);
...
SpriteSet(15, ...);
```

The compiler distinguishes between:

* Constant-bound loops → unroll
* Variable-bound loops → partial unroll or DMA assist

---

# 10. Physics Engine PRD

The ZealPascal Physics Engine is designed for fast, deterministic simulation optimized for the Zeal hardware. It leverages:

* The ZVB tile engine (solid tiles, slopes, one-way platforms)
* Sprite collision regions
* The high-speed Z80 CPU freed from video workloads

---

## 10.1 Broad-Phase Collision Detection

The broad-phase system uses a **uniform spatial grid**, where the world is divided into 16×16 or 32×32 buckets.

### 10.1.1 Spatial Hashing

Each entity with a collider is inserted into one or more grid buckets:

```
BucketX := Entity.X div CellSize;
BucketY := Entity.Y div CellSize;
Grid[BucketX][BucketY].Add(Entity);
```

### 10.1.2 Query

Collision checks only occur between entities in the same or adjacent buckets.

Benefits:

* Reduces collision workload from O(N²) to ~O(N)
* Extremely efficient on Z80

---

## 10.2 Multi-Shape Colliders

ZealPascal supports light-weight collider types:

### 10.2.1 AABB (Axis-Aligned Bounding Box)

Fastest option:

```
TColliderAABB = record
  X, Y, W, H: Integer;
end;
```

### 10.2.2 Circle Collider

Good for projectiles:

```
TColliderCircle = record
  X, Y, Radius: Integer;
end;
```

### 10.2.3 Capsule (Optional, Phase 2)

Useful for smooth platformer physics.

Collider type encoded in entity flags.

---

## 10.3 Tilemap Overlap Resolution

Tilemap collisions use the ZVB tilemap as a **solid-region mask**.

### 10.3.1 Tile Queries

```
TileX := (Entity.X + OffsetX) div TileWidth;
TileY := (Entity.Y + OffsetY) div TileHeight;
Tile := Tilemap[TileY][TileX];
```

### 10.3.2 Solid Tile Rules

A tile may specify its collision type:

* Solid
* One-way platform
* Slope (future)
* Ladder (future)

### 10.3.3 Overlap Correction

For AABB:

```
if OverlapX < OverlapY then
  shift on X axis
else
  shift on Y axis
```

---

## 10.4 Gravity and Forces Model

### 10.4.1 Gravity

```
VY := VY + Gravity;
Y := Y + VY;
```

Gravity constant tuned for 60 FPS.

### 10.4.2 Movement

Velocity-based movement:

```
X := X + VX;
Y := Y + VY;
```

### 10.4.3 Friction

Applied when on ground:

```
VX := VX * Friction;
```

### 10.4.4 Jumping Model

```
if JumpPressed and OnGround then
  VY := -JumpStrength;
```

Supports coyote time (optional).

---

# 11. Audio Engine PRD

ZVB includes an audio subsystem configurable via memory-mapped registers.
ZealPascal provides a structured API for music, sound effects, and mixing.

---

## 11.1 Channel Model

The ZVB audio chip provides:

* Multiple tone channels
* Noise generation
* PCM sample playback (depending on mode)

Each channel is controlled by:

```
Frequency
Volume
Waveform
```

---

## 11.2 Sound Effects

APIs:

```pascal
procedure PlaySFX(id: Byte);
procedure StopSFX(id: Byte);
```

Sound effects are stored as:

```
TSoundEffect = record
  Data: Pointer;
  Length: Word;
end;
```

---

## 11.3 Music Playback

Music can be:

* Pattern-based (tracker-like)
* Streamed PCM (limited by CPU)
* Sequenced notes

API:

```pascal
procedure PlayMusic(song: PMusic);
procedure StopMusic;
procedure MusicUpdate; // called each frame
```

Music driver must:

* Update channel frequencies
* Step through patterns
* Apply envelopes
* Mix (if needed)

---

## 11.4 Channel Mixing

Mixing rules:

* Priority system (SFX > Music if channels limited)
* Ducking (optional fade when SFX plays)

The engine attempts to maintain stable audio even under CPU load.

---

## 11.5 ZealPascal Audio API Summary

```
unit ZVB_Audio;
procedure SetTone(channel: Byte; freq: Word; vol: Byte);
procedure PlaySample(data: Pointer; size: Word);
procedure PlayMusic(song: PMusic);
procedure PlaySFX(id: Byte);
```

---

# 12. Input System PRD

The input system handles keyboard, controller (future), and internal event mapping.

---

## 12.1 Key Events

```
EV_KEYDOWN
EV_KEYUP
```

Keyboard state stored in:

```
KeyDown[0..255]: Boolean;
```

Events generated every frame where state changes occur.

---

## 12.2 Input Mapping

Allows remapping keys:

```pascal
MapAction('Jump', KEY_Z);
MapAction('Shoot', KEY_X);
MapAction('Left', KEY_LEFT);
MapAction('Right', KEY_RIGHT);
```

At runtime:

```
if Input.Down('Jump') then...
```

---

## 12.3 Axis Abstraction

Smooth movement uses virtual axes:

```
X := Input.Axis('MoveX'); // -1, 0, or +1
Y := Input.Axis('MoveY');
```

Computed from multiple keys.

---

# 13. Scripting Layer PRD

The scripting layer allows simple cutscenes, triggers, and event-driven sequences.

---

## 13.1 Language Overview

A tiny interpreted DSL:

```
MOVE entity x y
WAIT frames
PLAY sfx_id
ANIM entity state
IF flag THEN ...
```

Lightweight parser and interpreter managed by ZealPascal runtime.

---

## 13.2 Event Triggers

Examples:

* OnEnterArea
* OnCollision
* OnButtonPress
* OnMusicEnd

Triggers bind to scripts:

```
Trigger('Area1Enter', 'Cutscene1.txt');
```

---

## 13.3 Execution Model

Each script has:

* Instruction pointer
* Local registers
* Timeout counter

Scripts run cooperatively, advancing a few instructions per frame.

---

# 14. Scene Editor & Tooling PRD

The ZealPascal Game Engine integrates tightly with **ZealIDE**, enabling a full visual development workflow for games.

---

## 14.1 Visual Scene Editor (in ZealIDE)

The scene editor provides a real-time, WYSIWYG environment for building levels, placing entities, and configuring scenes.

### 14.1.1 Features

* Tilemap view with zoom/pan
* Grid-based painting
* Entity placement and manipulation
* Layer visibility controls (background, midground, foreground, UI)
* Inspector panel for selected entity

### 14.1.2 Scene Export

The editor generates:

* Pascal constants for tilemaps
* Entity placement arrays
* Scene initialization code
* Asset references

Scene data can also be exported as JSON for external processing.

---

## 14.2 Drag-and-Drop Entity Placement

### Functionality

* Drag entities from a palette onto the scene.
* Modify properties via inspector:

    * Initial position
    * Sprite/animation
    * Collider shape
    * Script triggers

### Generated Code Example

```pascal
Scene.AddEntity(CreateEnemy(120, 80));
Scene.AddEntity(CreatePlatform(32, 128, 64));
```

---

## 14.3 Tilemap Paint Tools

The editor supports multiple tile editing modes:

* Pencil
* Line
* Rectangle/Fill
* Flood fill

### Tile Inspector

Shows:

* Tile ID
* Palette
* Collision type (solid, slope, ladder)

### Undo/Redo

Unlimited history stored efficiently using tile-diff chunks.

---

## 14.4 Animation Previewer

ZealIDE includes an animation viewer supporting:

* Frame-by-frame preview
* Playback speed controls
* Palette preview
* Automatic generation of Pascal animation structs

The previewer simulates ZVB timing to match in-game animation speed.

---

# 15. Networking / Multiplayer PRD

Multiplayer communication is enabled through:

* Link cable (UART or GPIO-based)
* SPI peripheral on the ZVB

This PRD defines the minimal networking stack for cooperative 2-player games.

---

## 15.1 Packet Format

Packets consist of:

```
Byte 0: PacketType
Byte 1: PayloadLength
Byte 2..N: Payload
Byte N+1: CRC (calculated via ZVB CRC unit)
```

### Packet Types

* `PKT_INPUT` — player input state
* `PKT_STATE` — world/actor state sync
* `PKT_CHAT` — optional text
* `PKT_PING`/`PKT_PONG` — latency tests

---

## 15.2 Sync Model

Two models are supported:

### 15.2.1 Lockstep Sync (Deterministic)

* Each client sends input only
* Both machines simulate the full game
* Requires deterministic physics & RNG
* Perfect for shmups, fighting games

### 15.2.2 State Sync (Snapshot)

* One machine is host
* Host sends entity positions/states
* Client interpolates frames
* Good for platformers, co-op titles

---

## 15.3 Cooperative 2-Player Games

APIs:

```pascal
procedure NetInit(mode: TNetMode);
procedure NetSend(data: Pointer; size: Byte);
procedure NetPoll;
```

Example use:

```pascal
if Net.Connected and Input.Down('Shoot') then
  NetSend(@ShootPacket, SizeOf(ShootPacket));
```

---

# 16. FileSystem + SaveGame PRD

The ZealPascal engine supports persistent data through Zeal OS filesystem and optional bare-metal storage.

---

## 16.1 Save Slots

Save files appear as:

```
/save/slot1.dat
/save/slot2.dat
/save/slot3.dat
```

Each file includes:

* Magic header
* Version
* Player stats
* Level state
* Inventory
* Script flags

---

## 16.2 Compression

Save games optionally compressed using:

* Simple RLE
* LZ4-like dictionary compression (optimized for Z80)

Compression reduces storage footprint and load times.

---

## 16.3 Checksum / CRC

ZVB CRC peripheral used for integrity checks:

```
Checksum := CRC32(Data, Size);
```

Save load refuses corrupted files unless developer overrides.

---

# 17. Asset Packaging Specification

ZealPascal introduces a **WAD-like asset bundle format** (`.ZPK` — Zeal Package) containing:

* Tilesets
* Spritesheets
* Audio samples
* Tilemaps
* Scripts
* Scene metadata

---

## 17.1 .ZPK Container Format

### Header

```
'ZPK' + Version
FileCount
IndexOffset
```

### File Index

Each entry:

```
FileName
Offset
Size
Type
CompressionFlag
```

### Chunk Types

* TILESET
* SPRITESHEET
* TILEMAP
* AUDIO
* SCRIPT
* SCENE

---

## 17.2 Runtime Decompression

At runtime:

```
LoadChunk(name): Pointer;
```

* If compressed → decompress into heap pages
* If banked → map directly via MMU

---

## 17.3 Banked Loading

Large assets can be loaded in banks:

* Tilemap row-by-row
* Spritesheet animation banks
* Audio streamed per page

API example:

```pascal
Chunk := ZPK.LoadBank('Level1Tiles', BankIndex);
```

---

# 18. Gameplay Systems PRD

This section defines high-level gameplay systems for ZealPascal games, enabling RPGs, adventure titles, platformers, and complex story-driven games.

---

## 18.1 Dialogue System

The dialogue system supports:

* Text boxes
* Portraits (optional sprite layer)
* Typewriter effects
* Choice-based branching
* Script integration

### 18.1.1 Dialogue Script Format

```
DIALOGUE "Intro1"
  SHOWTEXT "Welcome to Zeal!"
  WAITKEY
  CHOICE "Where am I?" -> Node2
  CHOICE "Goodbye" -> End
END
```

### 18.1.2 Runtime API

```pascal
procedure ShowDialogue(id: String);
procedure ShowText(s: String);
procedure ShowChoices(options: array of String);
```

### 18.1.3 Rendering

* Rendered via tilemap or bitmap text layer
* Text speed adjustable
* Multiple fonts supported

---

## 18.2 UI Widgets

Built-in UI components:

* Label
* Button
* Panel
* Progress bar
* Inventory grid

### Widget Structure

```
TWidget = class
  X, Y: Integer;
  W, H: Integer;
  procedure Render; virtual;
  procedure Update; virtual;
end;
```

### Widget Manager

Scenes can include widget trees:

```
UIRoot.AddChild(Button1);
```

---

## 18.3 Inventory System

The inventory system supports:

* Item slots
* Stackable items
* Equipment slots
* Item metadata

### Item Definition

```
TItem = record
  ID: Byte;
  Name: String;
  IconTile: Byte;
  Stackable: Boolean;
end;
```

### Inventory Structure

```
TInventory = class
  Slots: array[0..31] of TItemStack;
  procedure AddItem(id: Byte);
  procedure RemoveItem(id: Byte);
end;
```

---

## 18.4 Quest Flags

Quest system is based on boolean or integer flags.

### API

```pascal
procedure SetFlag(name: String; value: Integer);
function GetFlag(name: String): Integer;
```

Quest flags can trigger:

* Scripts
* Dialogue options
* Enemy spawns
* Scene transitions

---

## 18.5 Scene Transitions

Supported transition types:

* Fade in/out (palette fades)
* Slide transitions
* Crossfade (tilemap swap)
* Instant load

### API

```pascal
procedure TransitionTo(SceneID: Byte; Effect: TTransition);
```

Transitions integrate with scripting and scene lifecycle.

---

# 19. Audio Tooling PRD

ZealPascal will integrate audio tooling into ZealIDE to simplify creation of music and sound effects.

---

## 19.1 Tracker Integration

The audio engine supports import of tracker modules from:

* ProTracker-like formats
* Custom lightweight `.ZMOD` format

### Features

* Pattern editor inside ZealIDE
* Channel view (frequency/volume envelopes)
* Export to runtime-friendly bytecode

---

## 19.2 Sample Editor

ZealIDE includes tools to:

* Trim audio samples
* Normalize volume
* Convert PCM to chip-compatible format
* Preview playback on real hardware if connected

Generated samples saved as:

```
TSample = record
  Data: Pointer;
  Length: Word;
end;
```

---

## 19.3 Music Compiler

Music compiler converts tracker patterns into:

* Frequency tables
* Envelope sequences
* Playback bytecode

Output inserted into `.ZPK` asset bundles.

APIs:

```pascal
procedure PlayMusic(id: Byte);
procedure StopMusic;
```

---

# 20. Benchmark & Profiling Toolkit

ZealPascal includes tools for profiling the engine and games.

---

## 20.1 Frame Analyzer

Displays:

* Update time
* Render time
* VBlank usage
* DMA load

UI example:

```
Update: 34% | Render: 18% | VBlank: OK
```

---

## 20.2 VRAM Usage Meters

Real-time metrics:

* Tileset occupancy
* Sprite RAM usage
* Palette entries
* Tilemap memory

Debugger integration:

```
VRAM: 68% full | Sprites: 40 used
```

---

## 20.3 Audio Channel Meter

Shows:

* Active channels
* Frequencies
* Volume levels
* SFX priority and mixing

Useful for tuning complex soundscapes.

---

# 21. Runtime Testing Framework

Includes utilities for verifying engine correctness.

---

## 21.1 Automated Unit Tests

Unit tests run on Zeal hardware or emulator.

Testable components:

* Math
* Physics (movement, collisions)
* Tilemap queries
* Animation stepping
* Scripting opcodes

Framework example:

```pascal
AssertEqual(Physics.Step(0, 1), 1);
```

---

## 21.2 Visual Regression Tests

Tests capture frame buffers and compare them with reference images.

Detected differences:

* Graphic corruption
* Tilemap errors
* Palette mismatches

---

## 21.3 Performance Regression Tests

Benchmarks run as part of the build pipeline:

* Sprite stress
* Tile redraw test
* Music playback stability
* Scripting throughput

If results exceed thresholds, the build flags a regression.

---

# 22. ECS Architecture Extension

This section extends the ZealPascal Game Engine with a streamlined, Z80‑optimized ECS (Entity Component System) inspired by Bevy.

The goal: **maximize performance, minimize memory overhead, and provide a predictable, data‑oriented structure** suitable for the constraints of the Z80 while leveraging the full power of the ZVB GPU.

---

# 22.1 ECS Overview

While modern ECS engines (e.g., Bevy) use dynamic archetypes and type‑driven queries, ZealPascal uses a **static archetype model** with **Struct‑of‑Arrays (SoA)** component storage.

### Why ECS on Z80?

* Z80 benefits heavily from contiguous memory iteration.
* SoA layout drastically reduces pointer chasing.
* Systems become simple tight loops → extremely fast.
* Archetype partitioning avoids unnecessary branching.
* Change‑detection bits prevent redundant work.

The ECS integrates seamlessly with existing engine systems: physics, input, rendering, AI, scripting.

---

# 22.2 Entity Representation

Entities are represented as **integer IDs**:

```
type
  TEntityID = Word;
```

Entities store **no data internally**.
All components live in dedicated arrays.

---

# 22.3 Component Storage (SoA Layout)

Each component type has its own array, allocated in a dedicated page or contiguous pages.

### Example:

```
PositionX: array[0..MAX_ENTITIES] of Integer;
PositionY: array[0..MAX_ENTITIES] of Integer;
VelocityX: array[0..MAX_ENTITIES] of Integer;
VelocityY: array[0..MAX_ENTITIES] of Integer;
SpriteID:  array[0..MAX_ENTITIES] of Byte;
Collider:  array[0..MAX_ENTITIES] of TColliderAABB;
```

Benefits:

* Cache-friendly linear iteration
* Systems can process thousands of operations cheaply
* No object pointer traversal

---

# 22.4 Static Archetypes

To avoid runtime archetype mutation (too expensive on Z80), we define **compile-time archetypes**.

### 22.4.1 Archetype Definitions

```
Archetype.Dynamic = [Position, Velocity, Sprite, Collider];
Archetype.Static  = [Position, Collider];
Archetype.Visual  = [Position, Sprite];
Archetype.Logic   = [Position, Script];
Archetype.Particle = [Position, Velocity, Lifetime];
```

Each archetype maintains its own entity list:

```
DynamicEntities: array[...] of TEntityID;
VisualEntities: array[...] of TEntityID;
```

### Benefits

* Eliminates runtime component lookup cost
* Systems iterate only relevant entities
* Predictable execution model

---

# 22.5 Systems as Pure Iterators

Systems operate over **arrays of components**, not entities:

### Example System: Movement

```pascal
procedure MovementSystem;
var i, e: Word;
begin
  for i := 0 to DynCount-1 do
  begin
    e := DynamicEntities[i];
    PositionX[e] += VelocityX[e];
    PositionY[e] += VelocityY[e];
  end;
end;
```

Advantages:

* Extremely fast
* Easy for compiler to optimize (unroll, inline)
* Predictable frame cost

---

# 22.6 System Schedule (Frame Pipeline)

Inspired by Bevy’s schedules, ZealPascal defines an ordered update pipeline:

```
InputSystem
ScriptingSystem
PhysicsSystem
CollisionSystem
AnimationSystem
AISystem
RenderingPrepSystem
RenderingSystem
```

Each system processes only required components.

---

# 22.7 Change Detection

Each entity has an 8‑bit change mask:

```
CHANGED_POS     = %00000001;
CHANGED_SPRITE  = %00000010;
CHANGED_STATE   = %00000100;
...
```

Systems can skip entities with unchanged data:

```
if (ChangeMask[e] and CHANGED_POS) = 0 then Continue;
```

### Benefits

* Renderer skips static sprites → big FPS win
* Physics avoids reprocessing resting bodies

Change flags are automatically cleared at end of frame.

---

# 22.8 Static Queries

Instead of dynamic queries like Bevy’s `Query<&Position, &Velocity>` (too heavy), ZealPascal uses **compile-time queries**, e.g.:

```
for e in DynamicEntities do
  ... // uses Position, Velocity
```

The compiler knows:

* Which components are required
* Which archetype lists contain eligible entities

---

# 22.9 Integration With Existing Engine Systems

### Rendering

* Renderer iterates only Visual & Dynamic archetypes.
* Can batch sprite updates based on contiguous entity IDs.

### Physics

* Physics system runs only on Dynamic archetype.

### AI

* AI states stored in SoA arrays → extremely fast decision loops.

### Scripting

* Scripts drive only entities in Logic archetype.

---

# 22.10 ECS Debug & Tools

ZealIDE gains ECS debugging tools:

* Component inspector (shows Position, Velocity, Sprite for each entity)
* Archetype health metrics (counts, memory usage)
* Change-mask visualizer
* System timing panel for each system in the schedule

---

# 23. Final Summary

With ECS architecture added, the ZealPascal Game Engine now includes:

* Struct-of-Arrays component model
* Compile-time archetypes
* High-performance system iterators
* Deterministic system execution schedule
* Change detection
* ECS-aware debugging and profiling

These upgrades dramatically increase performance and organization, making ZealPascal capable of managing **hundreds of on-screen entities** even on a Z80 CPU, while offering a modern conceptual structure similar to Bevy ECS but scaled to 8‑bit constraints.

This completes the ECS extension for the ZealPascal Game Engine PRD.

The ZealPascal Game Engine PRD has now been expanded to include:

* Gameplay systems: dialogue, UI, inventory, quest flags, transitions
* Audio tooling: tracker, sample editor, music compiler
* Benchmarking and profiling toolkit
* Complete runtime testing framework

ZealPascal now defines not only a programming language and runtime, but a fully integrated **development, debugging, and content creation ecosystem** for the Zeal 8-bit computer.

This completes a modern retro engine design on top of cutting-edge FPGA-backed 8-bit hardware.

This update extends the ZealPascal Game Engine PRD with:

* Full visual scene editor and tooling design
* Networking and multiplayer stack
* File system + save game specification
* Asset packaging system with WAD-like bundles

The ZealPascal Game Engine is now positioned as a:

* **Complete development ecosystem**
* **Modernized 8-bit game framework**
* **Toolchain-integrated runtime & editor suite**

This lays the groundwork for production-quality game development on the Zeal 8-bit computer.

This update integrates:

* Full physics engine design
* Audio engine with music/SFX/mixing
* Input system with mapping & axes
* Lightweight scripting language

Together, these systems complete the ZealPascal Game Engine foundation, enabling:

* Platformers
* RPGs
* Shmups
* Puzzle games
* Cutscenes, story events, sound & control layers

ZealPascal is now specified as a full retro-modern game engine stack for the Zeal 8-bit computer.

This extension formalizes the internal runtime behavior, animation subsystem, engine-aware compiler optimizations, and advanced asset pipeline logic for ZealPascal Game Engine.

The engine is now a fully specified high-level development framework, capable of:

* Efficient sprite/tile animation
* Complex physics and entity interactions
* Asset-driven content pipelines
* Engine-informed compiler optimizations

Together, these systems make ZealPascal one of the most capable 8-bit game development stacks ever designed.

The ZealPascal Game Engine provides a full, structured, and high-performance foundation for game development on the Zeal 8‑bit Computer.

It defines:

* Concrete API for TGame, TScene, TEntity, components
* Standard frameworks for platformers and shmups
* Strict performance targets
* A comprehensive suite of test demos

With this PRD, ZealPascal becomes a complete retro‑modern game development ecosystem, uniquely empowered by the Zeal Video Board’s FPGA acceleration and the Z80’s freed CPU workload.

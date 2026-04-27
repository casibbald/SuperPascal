# 📘 PRD: ZealPascal Audio System

## Version 1.0 — Comprehensive Specification for Music, SFX, Mixing, Tools & Runtime Integration

This document defines the **entire audio subsystem** for the ZealPascal Game Engine, expanding the high‑level engine PRD with a complete, production-grade audio architecture.

The design leverages:

* The Zeal Video Board’s hardware audio features
* ZealPascal’s high-level abstractions
* A mixed-model synthesis + sample playback pipeline
* Toolchain integration (tracker, editor, compiler)

This PRD describes:

1. **Audio hardware model**
2. **Audio runtime system**
3. **Sound effects system**
4. **Music system & compiler**
5. **Channel mixer & priority logic**
6. **Audio scripting interface**
7. **ZealIDE tooling integration**
8. **Profiling & debugging tools**

---

# 1. Audio Hardware Model

The ZVB audio subsystem provides:

* Tone channels
* Noise generator
* PCM/sample playback engine
* Memory-mapped registers for frequency, volume, and waveform
* Optional DMA for streaming sample data

ZealPascal abstracts this through a unified, user-friendly API while enabling low-level control for advanced users.

---

# 2. Audio Runtime System

## 2.1 Audio Manager

The runtime maintains global audio state:

```
type
  TAudioManager = class
    Channels: array[0..7] of TAudioChannel;
    MusicPlaying: Boolean;
    CurrentMusic: PMusicTrack;
    procedure Init;
    procedure Update;
    procedure PlaySFX(id: Byte);
    procedure PlayMusic(track: PMusicTrack);
    procedure StopMusic;
  end;
```

`Update` is called once per frame.

---

## 2.2 Audio Channel Structure

```
type
  TAudioChannel = record
    Frequency: Word;
    Volume: Byte;
    Waveform: Byte;
    Active: Boolean;
    Priority: Byte; // higher = overrides lower
  end;
```

Channels map to ZVB hardware registers.

---

## 2.3 Register Access Intrinsics

Low-level hardware access:

```
procedure AudioRegWrite(addr: Word; value: Byte); intrinsic;
procedure AudioRegWriteWord(addr: Word; value: Word); intrinsic;
```

Used by SFX, Music, and Mixer subsystems.

---

# 3. Sound Effects System (SFX)

## 3.1 SFX Data Format

Each SFX is described as:

```
type
  TSoundEffect = record
    Data: Pointer; // PCM or sequence commands
    Length: Word;
    Priority: Byte;
  end;
```

## 3.2 SFX Playback API

```
procedure PlaySFX(id: Byte);
procedure StopSFX(id: Byte);
```

### Priority Rules

* Higher-priority SFX may override or duck music.
* Low-priority SFX ignored if channels are saturated.

---

# 4. Music System

Music is pattern-based (tracker-like) for CPU efficiency.

## 4.1 Music Track Structure

```
type
  TMusicTrack = record
    Patterns: ^TPattern;
    PatternCount: Byte;
    Order: ^Byte;
    Tempo: Byte;
    Speed: Byte;
  end;
```

## 4.2 Pattern Structure

```
type
  TPattern = record
    Rows: array[0..63] of TMusicRow;
  end;

type
  TMusicRow = record
    Note: Word;    // frequency lookup index
    Volume: Byte;
    Effect: Byte;  // vibrato, slide, etc.
  end;
```

## 4.3 Music Engine Update

Pseudocode:

```
Step := Step + 1;
if Step >= Speed then
  Row := (Row + 1) mod PatternLength;
  ApplyRowToChannels(Row);
  Step := 0;
```

Supports:

* Frequency slides
* Volume envelopes
* Basic chip modulation

---

# 5. Mixer & Priority Logic

## 5.1 Mixer Rules

* SFX may preempt music channels based on priority.
* Music always tries to reclaim channels when SFX ends.

## 5.2 Ducking

When high-priority SFX plays:

```
MusicVolume := MusicVolume * DuckFactor;
```

Volume restored after SFX completes.

## 5.3 Channel Ownership Table

```
ChannelOwner[0..7] = (SFX or MUSIC or IDLE)
```

---

# 6. Audio Scripting Interface

Scripts can trigger or modify audio:

```
PLAY_MUSIC "Theme1"
PLAY_SFX "Explosion"
SET_VOLUME Music 80
FADE_MUSIC 30
```

Engine provides small helper commands:

```
procedure FadeMusic(frames: Byte);
```

Script system may also listen to events:

* OnMusicEnd
* OnBeat

---

# 7. ZealIDE Audio Tooling

## 7.1 Tracker Editor

Features:

* Pattern editing
* Waveform visualization
* Note preview
* Volume & effect columns
* Channel inspector

Exports `.ZMOD` (Zeal Music Module) files.

## 7.2 Sample Editor

Tools for:

* Trimming
* Normalizing
* Downsampling
* Loop-point editing

Exports PCM blocks optimized for ZVB.

## 7.3 Music Compiler

Converts tracker data into runtime-ready tables:

* Frequency tables
* Effect envelopes
* Patterns and orders

Outputs embedded Pascal or `.ZPK` chunks.

---

# 8. Profiling & Debugging Tools

## 8.1 Audio Channel Meter

Real-time display:

* Active channels
* Frequency
* Volume
* Owners (SFX/Music)

## 8.2 Mixer Timeline Debugger

Shows mixing decisions per frame.

## 8.3 Music Pattern Profiler

Reports:

* Longest rows
* Per-frame audio CPU cost
* Effect timing

---

# 9. Out-of-Band Music Streaming PRD

Long PCM tracks or voice samples cannot fit entirely in RAM or must be streamed progressively. This section defines the **streaming audio subsystem** built atop ZVB hardware and ZealPascal runtime.

---

## 9.1 Streaming Architecture Overview

Streaming uses:

* **ZVB DMA** engine for bulk transfers
* **Paged buffers** managed by ZealPascal
* **Double-buffering** for seamless playback
* **Background streaming tasks** integrated into the ECS schedule

### Goals:

* Support long PCM audio (e.g., background ambience, cutscene voiceovers)
* Avoid audio gaps or stutters
* Minimize CPU load on the Z80

---

## 9.2 PCM Stream Format

Large PCM tracks are stored in `.ZPK` bundles as chunked data:

```
StreamHeader
Chunks[0..N] : Each chunk <= 16 KB
```

Chunk metadata:

```
Offset
Size
Compression (optional)
CRC32
```

---

## 9.3 Streaming Pipeline

### Components:

* **StreamReader** – loads/decompresses chunks
* **StreamBuffer** – two alternating PCM buffers
* **StreamPlayer** – feeds ZVB audio channel

### Flow:

```
[Disk] → [StreamReader] → [BufferA] → [ZVB]
                        ↘ [BufferB] ↗
```

While ZVB consumes `BufferA`, the engine fills `BufferB`.
When `BufferA` empties, they swap.

### API:

```pascal
procedure StreamMusicStart(id: Byte);
procedure StreamMusicStop;
procedure StreamMusicUpdate; // called every frame
```

---

## 9.4 Cutscene Voiceovers & Cinematic Audio

Streaming enables:

* Voice samples synchronized with cutscenes
* Long-form narration
* Ambient background loops

Sync hooks:

```pascal
OnStreamChunkBoundary;
OnStreamEnd;
```

Cutscene scripts can react to these events.

---

# 10. Full Game Loop Integration PRD

This section defines how audio, ECS, cutscenes, transitions, and rendering coordinate.

---

## 10.1 Frame Structure With Audio

Updated schedule:

```
1. InputSystem
2. ScriptSystem
3. AudioSystem.Update   ← NEW
4. PhysicsSystem
5. CollisionSystem
6. AnimationSystem
7. AISystem
8. SceneTransitionSystem
9. RenderPrep
10. Rendering
11. AudioStreaming.Update ← NEW
```

Audio updates early to reduce latency.
Streaming updates late to ensure buffers are filled.

---

## 10.2 Scene Transitions & Audio

Transitions may:

* Fade music volume
* Switch tracks via crossfade
* Pause/resume streaming

APIs:

```pascal
procedure MusicFadeOut(frames: Byte);
procedure MusicFadeIn(frames: Byte);
procedure Crossfade(newTrack: PMusicTrack; frames: Byte);
```

Transitions may specify audio behavior:

```
TransitionTo('Cave', EffectFade, MusicChange='CaveAmbience');
```

---

## 10.3 Cutscenes & Audio Synchronization

Cutscene scripts synchronize audio via:

```
WAIT_MUSIC_BEAT
WAIT_MUSIC_ROW
WAIT_STREAM_CHUNK
```

Script can trigger audio actions:

```
PLAY_SFX "Explosion"
STREAM_MUSIC "IntroNarration"
MUSIC_SET_TEMPO 6
```

---

## 10.4 ECS Interaction

ECS systems may rely on audio signals:

* AI reacts to musical beats
* Entities trigger SFX on state change
* Environmental audio zones change music dynamically

Audio registers change-detection flags to minimize overhead.

---

# 11. Audio Regression Testing

Ensures that audio output remains stable across engine versions.

---

## 11.1 SFX Regression Tests

Automated test cases verify:

* Envelope correctness
* Priority behavior
* Channel takeover rules

Tests compare:

* Expected channel sequence
* Expected register writes

---

## 11.2 Music Regression Tests

Tests run a music track offline and capture:

* Per-row frequency tables
* Volume patterns
* Effects (vibrato, slide)

Outputs compared to reference signatures:

```
PatternHash := CRC32(PatternPlaybackDump);
```

Failures indicate unintended musical changes.

---

## 11.3 Streaming Regression Tests

Validate:

* Seamless buffer swap
* No PCM underruns
* Correct synchronization with scripts

Simulation includes:

* Chunk load delays
* Varying CPU load

Expected output:

```
No gaps > 1 PCM frame
No unexpected resets
```

---

## 11.4 Golden Audio Snapshots

Capture audio snapshots for known scenes:

* PCM samples dumped at key points
* Compared using fuzzy matching (tolerate minor variance)

This catches:

* Mixing errors
* Incorrect frequencies
* Wrong envelopes
* Regressions after refactoring

---

# 12. Final Summary

This expansion adds a **full audio streaming subsystem**, **complete integration into the main game loop**, and a **rigorous audio regression testing suite**.

ZealPascal Audio now supports:

* Long-form streamed audio (PCM/voice)
* Music and SFX coordination
* Cutscene synchronization
* Robust cross-version stability guarantees

With this, ZealPascal’s audio framework reaches professional-level completeness, fully aligned with the capabilities of the Zeal hardware.

The **ZealPascal Audio System** is now a complete specification including:

* Hardware audio model
* Runtime audio engine
* SFX subsystem with priorities
* Tracker-style music system
* Mixer architecture with ducking & channel ownership
* Audio scripting hooks
* Full ZealIDE audio editing toolchain
* Comprehensive debugging & profiling tools

This PRD defines a modern, powerful audio stack designed specifically for the capabilities and constraints of the Zeal 8-bit Computer, making ZealPascal capable of rich, expressive audio for games and applications.

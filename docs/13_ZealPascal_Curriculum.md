# 📘 ZealPascal Curriculum v1.0 — Three-Semester Educational Program

## A Complete 36‑Week Teaching Plan for Schools Using the Zeal 8‑bit Computer

This document defines the **official ZealPascal teaching curriculum**, designed for:

* Middle schools
* High schools
* Homeschool programs
* After‑school technology clubs
* Offline computer labs using the **Zeal 8-bit Computer** + **ZealIDE**

The curriculum spans **3 × 12‑week semesters**:

1. **Semester 1: Foundations of Programming & Pascal**
2. **Semester 2: Game Programming, Graphics, ECS & Math**
3. **Semester 3: Capstone Project (Full Game or Mathematical Simulation)**

Each semester includes:

* Weekly objectives
* Lectures
* Exercises
* Assessments
* Projects

The curriculum is designed to take a student from complete beginner → game developer → independent creator.

---

# 🎓 0. Educational Philosophy

The ZealPascal program is built around:

### ✔ Accessibility

Simple concepts first (records, loops, procedures), no internet required.

### ✔ Incremental mastery

Students gain confidence by building visible, working programs every week.

### ✔ Integration with math & physics

The Zeal 8-bit computer becomes a **teaching machine** for:

* trigonometry
* geometry
* linear algebra (2D)
* digital audio

### ✔ Project-based learning

Every semester ends with a tangible project designed to impress and build a portfolio.

---

# 🧠 Semester 1 — Foundations of Programming (12 Weeks)

## *Goal:* Students master structured programming, data types, control flow, and basic graphics.

### **Week 1 — Introduction to the Zeal Computer & ZealPascal**

* What is a computer? CPU, RAM, video, input.
* First ZealPascal program: `Hello, Zeal!`
* How to write, run, debug in ZealIDE.
* Variables: `integer`, `byte`, `boolean`.

### **Week 2 — Expressions & Operators**

* Arithmetic operators
* Assignment
* Operator precedence
* Using the REPL mode (optional)

### **Week 3 — Procedures & Functions**

* Modular thinking
* Parameter passing (value only)
* Return values
* Exercises: temperature converter, simple calculator

### **Week 4 — Conditionals**

* `if`, `if/else`, nested conditions
* Boolean logic
* Mini-project: Number guessing game

### **Week 5 — Loops (Part 1)**

* `while` and `repeat` loops
* Counters & accumulators

### **Week 6 — Loops (Part 2)**

* `for` loops
* Loop variables and iteration patterns
* Mini-project: multiplication tables generator

### **Week 7 — Arrays & Records**

* 1D arrays
* Records as data containers
* Simple simulations using arrays

### **Week 8 — Strings & Text Processing**

* `string` operations
* Concatenation, substring, lexicographic comparison
* Mini-project: text banner animator

### **Week 9 — Introduction to Graphics**

* ZVB basics
* Setting screen mode
* Drawing tiles and pixels
* Project: draw a smiley face using tiles

### **Week 10 — Simple Animation**

* Moving an object each frame
* VBlank
* Input handling
* Project: move a character with arrow keys

### **Week 11 — Modular Programming with Units**

* Creating and using units
* Splitting code across files

### **Week 12 — Final Project (Beginner)**

Students choose one:

* Brick-breaker lite
* Calculator with UI
* Animated clock
* Maze explorer

Assessment: Completed project + oral explanation.

---

# 🎮 Semester 2 — Game Development, ECS & Applied Math (12 Weeks)

## *Goal:* Students learn ECS architecture, physics basics, trig, sprites, audio, scripting, and build full games.

### **Week 1 — ECS Concepts & Records as Components**

* Review: records
* Struct-of-arrays concept
* Components: Position, Velocity, Sprite
* Systems: Update, Render

### **Week 2 — Sprite Engine**

* Show/hide sprites
* Animate sprite frames
* Project: bouncing sprite demo

### **Week 3 — Tilemaps & Level Design**

* Tilemap DSL
* Drawing tilemaps
* Tile collisions

### **Week 4 — Physics Basics**

* Gravity
* Velocity integration
* Axis-aligned bounding boxes (AABB)

### **Week 5 — Applied Math: Fixed-Point (`Q8.8`)**

* Why floating-point is expensive
* Fractional movement
* Smoothing camera motion

### **Week 6 — Trigonometry for Games**

* Sin, cos, tan
* Angle-based movement
* Rotation demo: rotating sprite or pointer

### **Week 7 — Audio: SFX & Music**

* PlaySFX
* PlayMusic
* Volume control
* Sync animations to music

### **Week 8 — Scripting for Events & Behavior**

* Cutscene scripts
* WAIT, MOVE, ANIM commands

### **Week 9 — UI & Menus**

* Buttons
* Panels
* Simple menu system

### **Week 10 — Putting It All Together**

* Build a simple platformer or shmup
* ECS + trig + physics + sprites

### **Week 11 — Optimization & Debugging**

* Profiling tools
* Sprite budget
* Tile update limits
* Reducing memory usage

### **Week 12 — Final Project (Intermediate)**

Students build one:

* Full platformer (with multiple levels)
* 2D shmup
* Puzzle game with UI

Assessment: Working game + written documentation.

---

# 🚀 Semester 3 — Capstone Project (12 Weeks)

## *Goal:* Students design, build, test, and publish a complete game or simulation.

### **Week 1 — Project Proposal**

* Students pitch 3 ideas
* Teacher approves one based on scope

### **Week 2 — Architecture & Planning**

* ECS design
* Sprite and tileset planning
* Level map planning
* Audio requirements

### **Week 3–5 — Core System Implementation**

Students build:

* Entity logic
* Collision systems
* Physics
* UI flows

Weekly teacher check-ins.

### **Week 6–7 — Content Creation**

* Drawing sprites and tiles
* Creating music and SFX in ZealIDE
* Writing dialogues and scripts

### **Week 8–9 — Polish Phase**

* Performance tuning
* Bug fixing
* Adding QoL features

### **Week 10 — Testing & Refinement**

* Playtesting
* Teacher and peer feedback
* Compliance tests: does it run at 60 FPS?

### **Week 11 — Packaging**

* Build `.ZPK` bundle
* Write README
* Prepare presentation

### **Week 12 — Capstone Presentation Day**

Each student presents:

* Their game/simulation
* Their design decisions
* What they learned in ZealPascal

Capstone examples:

* Top-down RPG
* Physics sandbox
* Story-based adventure
* Math visualizer (graphs, trig demos, fractals)

---

# 🧪 Assessment Strategy

* **Weekly exercises** reinforce incremental learning.
* **Mini-projects** track creativity and debugging skills.
* **Semester projects** evaluate integration of all skills.
* **Capstone project** measures mastery and autonomy.

Rubrics emphasize:

* Correctness
* Creativity
* Code clarity
* Ability to explain thought process

---

# 📘 Final Notes

This curriculum is:

* **Complete** enough for a teacher to run a year-long course
* **Flexible** for different skill levels
* **Scalable**: beginner → intermediate → advanced
* **Designed around ZealPascal + ZealIDE**, leveraging 8-bit hardware for modern educational impact

Once compiler + IDE prototypes exist, we can generate:

* Student workbook
* Teacher guide
* Step-by-step tutorial projects
* Printable worksheets

---

# ✔️ ZealPascal Curriculum v1.0 — Draft Complete

Ready for refinement, visual formatting, and packaging into educational materials.

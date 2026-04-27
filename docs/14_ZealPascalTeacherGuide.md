# 📘 **ZealPascal Teacher Guide v1.0**

## A Complete Instructor Handbook for Teaching ZealPascal Over Three 12-Week Semesters

This Teacher Guide supports the **ZealPascal Curriculum v1.0** and provides:

* Weekly teaching strategies
* Expected student difficulties and remediation paths
* Assessment rubrics
* Classroom workflows using ZealIDE
* Hardware management guidelines for the Zeal 8-bit Computer
* Pedagogical rationale behind each lesson
* Project templates and grading strategies

The guide assumes teachers **do not need advanced programming experience**. It is designed to be accessible to general computer science instructors, STEM teachers, and even motivated non-specialists.

---

# 🎓 0. Teaching Philosophy

ZealPascal is designed to:

* Introduce programming gently
* Reinforce mathematical thinking
* Empower creativity through game development
* Avoid complexity found in modern languages
* Provide an offline, distraction-free educational platform

The philosophy: **learn by building**.

Students immediately visualize results through graphics, audio, and real games.

You teach three things at once:

1. **Programming**
2. **Mathematical reasoning**
3. **System thinking** (game loops, ECS, events)

---

# 🧰 1. Classroom Setup

## 1.1 Hardware

Each student or pair requires:

* Zeal 8-bit computer
* SD card with ZealPascal toolkit installed
* Keyboard + display

Teachers require:

* Instructor Zeal unit
* Projector / HDMI capture (optional)
* Preloaded example projects

## 1.2 Software

* **ZealIDE**: code editor, builder, and debugger
* **ZealPascal compiler** (`zc`)
* Standard library & engine templates
* Pre-installed demos

## 1.3 Folder Structure for Students

```
/Projects
  /Semester1
  /Semester2
  /Capstone
```

---

# 🗂 2. Weekly Teaching Strategies (Semester by Semester)

Below is a condensed set of weekly instructor notes. Full lesson plans follow in the appendix.

---

# 📘 Semester 1 – Foundations

Focus: **structured programming**, **logic**, **intro graphics**.

### Week 1 – Introductions

**Teacher goals:**

* Demystify how computers work
* Build confidence early
* Show students a real result on screen

**Common struggles:**

* Typing errors → emphasize the compiler will guide them
* Forgetting semicolons

**Remediation:**

* Pair programming for slower students

---

### Week 2 – Expressions & Variables

**Teacher goals:**

* Reinforce mental model: variables store values
* Show variable changes visually (e.g., counter on screen)

**Common struggles:**

* = vs :=

**Fix:** explain that := means “becomes”.

---

### Week 3 – Procedures & Functions

Emphasize **reuse** and **building blocks**.

**Teaching tip:** Have students write a function they use twice.

---

### Week 4 – Conditionals

Introduce debugging through **print statements**.

**Try:** make a branching story (“Choose Your Adventure”).

---

### Week 5–6 – Loops

Focus on real examples:

* Drawing rows of tiles
* Animated counters
* Timers

**Common mistakes:** infinite loops

**Fix:** explain loop variable movement.

---

### Week 7 – Arrays & Records

Trick: show them arrays with a visualization (10 boxes labeled 0–9).

Records = “custom grouped data”.

---

### Week 8 – Strings

Use text animations and simple “UI” banners.

---

### Week 9 – Graphics Basics

Show tilemap vs bitmap.

Assign micro-project: draw a character.

---

### Week 10 – Input & Animation

Teach frame-based thinking: **Update → Draw → WaitVBlank**.

---

### Week 11 – Units

Explain modularity using folders on a computer.

---

### Week 12 – Final Project Evaluation

Rubric:

* correctness
* clarity
* creativity
* presentation

---

# 🎮 Semester 2 – Game Development, ECS, Math

Focus: **game loops**, **sprites**, **tilemaps**, **trig**, **physics**, **audio**, **ECS**.

### Week 1 – ECS

Use analogy: “Entities are IDs. Components are Lego bricks. Systems act on many bricks at once.”

### Week 2 – Sprites

Show simple animation.
Let them build a bouncing ball.

### Week 3 – Tilemaps

Encourage students to create their own levels.
Introduce tile collision using simple math.

### Week 4 – Physics

Teach gravity using arrows and motion charts.
Show difference between velocity and position.

### Week 5 – Fixed-Point Math

Explain why floats are slow.
Use Q8.8 for smooth motion.

### Week 6 – Trigonometry

Tie into math curriculum:

* unit circle
* waves
* angles in games

Hands-on: build a rotating arrow.

### Week 7 – Audio

Teach layering of SFX + music.
Show how sound enhances games.

### Week 8 – Scripting

Interactive cutscenes.
Students love this.

### Week 9 – UI

Make a working menu.
Buttons change scenes.

### Week 10 – Integration Week

Build mini-game with:

* movement
* enemies
* collisions
* level transitions

### Week 11 – Optimization

Use the profiler.
Show sprite budgets.
Performance matters.

### Week 12 – Final Game Project Evaluation

Rubric includes:

* fun gameplay
* good structure
* readable code
* working ECS

---

# 🏆 Semester 3 – Capstone Project

Focus: **independent work**, **software engineering discipline**, **polish**.

### Week 1 – Proposals

Help students narrow ideas to achievable scope.

### Week 2 – Architecture

Review:

* entity/component lists
* scene graph diagram
* flow charts

### Week 3–5 – Core Systems

Students implement:

* core gameplay
* collision
* animation
* audio
* scripting

Teacher role:

* unblock
* guide problem solving

### Week 6–7 – Content Creation

Show how to:

* draw sprites
* define tilemaps
* create sound effects

### Week 8–9 – Polish

Students improve:

* responsiveness
* UI polish
* menu design

### Week 10 – Testing

Use compliance suite lite:

* does game run at 60 FPS?
* any sprite glitches?

### Week 11 – Packaging

Create `.ZPK` bundle.
Prepare README.

### Week 12 – Capstone Presentation

Students present:

* game demo
* development journey
* lessons learned

Award certificates or badges.

---

# 📘 Pedagogical Strategies

## 1. Encourage pair programming

Pairs help reduce frustration.

## 2. Visual teaching is key

Show graphics + code side-by-side.

## 3. Use metaphors

* ECS is Lego
* Records are boxes
* Classes are robots that control boxes

## 4. Error messages are opportunities

Teach debugging early and positively.

## 5. Celebrate creativity

Games allow self-expression.

---

# 🧪 Assessment Rubrics

### Beginner Project

* 40% correctness
* 30% creativity
* 20% clarity
* 10% presentation

### Intermediate Game

* 30% gameplay
* 25% ECS correctness
* 20% graphics/audio polish
* 15% code structure
* 10% presentation

### Capstone

* 25% design
* 25% implementation
* 20% polish
* 20% documentation
* 10% presentation

---

# 📂 Appendices

## Appendix A — Weekly Lesson Plans

(Teacher-facing worksheets to be added in future drafts.)

## Appendix B — Tutorials for ZealIDE

* building
* debugging
* profiling
* asset import

## Appendix C — Troubleshooting Checklist

* syntax errors
* flicker in animation
* incorrect collision detection
* missing assets

---

# ✔️ ZealPascal Teacher Guide v1.0 — Draft Complete

This guide equips teachers with:

* clear weekly instructions
* motivations behind each lesson
* grading and assessment tools
* strategies for managing diverse classrooms

It is now ready for refinement and expansion into teacher training materials, printable PDFs, and interactive tutorials.

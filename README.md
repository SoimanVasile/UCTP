# ⚡ UCTP: Rust Timetable Scheduler

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?style=flat-square)
![Algorithm](https://img.shields.io/badge/Algorithm-Simulated_Annealing-blue?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)

> **A high-performance, automated university scheduling engine built in Rust.**

## 💡 About The Project

University class scheduling is an **NP-Hard optimization problem**. Manually assigning thousands of lectures to limited rooms without conflicts is nearly impossible.

I built **UCTP** to solve this using **Simulated Annealing**. It generates mathematically perfect schedules by treating constraints as an "energy minimization" problem, "cooling" a chaotic schedule into a conflict-free solution in seconds.

**Why Rust?**
I chose Rust for its **memory safety** and **zero-cost abstractions**, allowing the solver to run millions of iterations per second without garbage collection pauses.

## ✨ Key Features

* **⚡ Blazing Fast:** Solves complex schedules (20+ groups, hundreds of slots) in <1 second.
* **🛡️ Conflict-Free:** Guarantees 0 collisions for Rooms, Teachers, and Student Groups.
* **🧠 Smart Constraints:**
    * **Hard Constraints:**
        * **Room Capacity:** Students must fit in the room.
        * **Laboratory Equipment:** Labs must be in rooms with computers/equipment.
        * **Teacher Availability:** Professors cannot be in two places at once.
        * **Teleportation:** Prevents back-to-back classes in different buildings.
    * **Soft Constraints:**
        * **Gap Minimization:** Reduces awkward empty hours between classes.
        * **Compact Schedule:** Penalizes long 12-hour days, preferring compact blocks (e.g., 8am-2pm).
* **📊 Visual Output:** Renders a clean, readable ASCII timetable for every student group.

## 🚀 Quick Start

### 1. Clone & Setup
```bash
git clone [https://github.com/your-username/uctp.git](https://github.com/your-username/uctp.git)
cd uctp
```

### 2. Run the Solver
Use the `--release` flag to enable compiler optimizations (crucial for the algorithm's speed).

```bash
cargo run --release
```

### 3. See the Result
If a perfect schedule is found, the tool prints a formatted timetable for each group:

```text
╔═════════════════════════════════════════════════════════════════════════════╗
║ GROUP: CS Year 1 - G1 (ID: 11)                                              ║
╚═════════════════════════════════════════════════════════════════════════════╝
      Time      |          Mon           |          Tue           | ...
-------------------------------------------------------------------------------
  08:00-10:00   |     Math (Lecture)     |          ---           |
                |    (Amphitheater A)    |                        |
-------------------------------------------------------------------------------
  10:00-12:00   |          ---           |   Programming (Lab)    |
                |                        |        (Lab 101)       |
-------------------------------------------------------------------------------
```

## 🛠️ How It Works

The engine uses a **Simulated Annealing** algorithm to explore the search space:

1.  **State Representation:** A flat vector of assignments (Course ID → Time/Room).
2.  **Cost Function:** Calculates a "Penalty Score" based on broken constraints (e.g., +10,000 for a collision).
3.  **Mutation:** Randomly moves a class to a new slot/room.
4.  **Acceptance Probability:**
    * Better solution? **Always Accept.**
    * Worse solution? **Probabilistic Accept** ($P = e^{-\Delta E / T}$) to escape local optima.
5.  **Cooling Schedule:** Temperature decays geometrically ($T_{k+1} = T_k \times 0.9995$).

## 📂 Project Structure

This project follows idiomatic Rust architecture:

```
src/
├── domain/           # Core Logic (The "Business Rules")
│   └── schedule.rs   # The heavy-lifting constraint algorithms
├── solver/           # The AI Engine
│   └── simulated_annealing.rs
├── io/               # Data Handling
│   ├── normalize.rs  # Optimizes O(1) lookups by mapping IDs to indices
│   └── output.rs     # CLI Visualization
└── main.rs           # Entry point
```

## 🔮 Roadmap

* [x] Core Solver (Simulated Annealing)
* [x] Room & Group Collision Detection
* [x] Laboratory & Capacity Constraints
* [x] Gap Minimization (Soft Constraint)
* [x] Teacher Conflict Constraints (Double Booking & Teleportation)
* [ ] Parallel Execution (Multithreading)

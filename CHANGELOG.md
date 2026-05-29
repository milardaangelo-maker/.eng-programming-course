# 📜 English-Lang Official Update Log

This log tracks the evolution of English-Lang, from a simple script runner to a professional-grade logic engine.

---

## [v2.5] - The Stable Release (Current)
**"Machine-Rigid, Human-Friendly"**
- **Unified Grammar**: Standardized all complex commands to use the `do:` ... `end` block structure.
- **Real Lists**: Introduced the `create list` command for true data structure handling (no more fake list strings).
- **Depth-Aware Parser**: Rewrote the engine to count nesting levels, allowing infinite `if` inside `loop` inside `function`.
- **Improved Performance**: Optimized the Rust core for faster execution of loops and math.

## [v2.0] - The Logic Upgrade
**"Giving the Language a Brain"**
- **Nested Logic**: Added the ability to put commands inside other commands.
- **Loops**: Introduced the `repeat [N] times` command.
- **Math Engine**: Added the `calculate` verb using a real mathematical expression evaluator.
- **User Input**: Added `ask user` to allow scripts to be interactive.
- **Functions**: First implementation of custom skills using the `to` keyword.

## [v1.5] - IDE & System Integration
**"Professional Environment"**
- **VS Code Extension**: Created local syntax highlighting for `.eng` files.
- **File Association**: Registered `.eng` in the Windows Registry for double-click execution.
- **System Bridge**: Standardized `run system command` to allow access to CMD and PowerShell.

## [v1.0] - The Birth
**"The Simplest Language in the World"**
- **Core Engine**: Initial Rust-based interpreter using Regex pattern matching.
- **Basic Verbs**: Launched with `print`, `store`, `wait`, and `send to discord webhook`.
- **Portability**: Compiled into a single, no-install `.exe` for Windows.
- **Python Version**: Created a parallel Python interpreter for open-source readability.

---
*English-Lang: Coding at the speed of thought.*

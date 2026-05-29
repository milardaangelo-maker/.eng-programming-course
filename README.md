# 🎓 English-Lang v2.5 Stable Masterclass

Welcome to the **Stable Release** of the Easiest Language in the World. v2.5 standardizes everything so your code is "Machine-Rigid" but "Human-Friendly."

---

## 🛠️ The Core Rules
1. **Verbs First**: Every line starts with a clear action (Say, Ask, Store, Calculate).
2. **Blocks**: Every complex command starts with `do:` and MUST end with `end`.
3. **Variables**: Use `{name}` to use a stored value inside a sentence.

---

## 🏁 Module 1: Communication
```text
say "Hello World"
wait 2
ask "What is your name?" as username
say "Nice to meet you, {username}!"
```

---

## 🧠 Module 2: Memory & Lists
v2.5 introduces **Real Lists** for processing groups of items.
```text
# Simple storage
store "Angelo" as creator

# Real Lists
create list "Apple, Banana, Cherry" as fruits

for each f in fruits do:
    say "I am eating a {f}"
end
```

---

## ➗ Module 3: Math Engine
```text
calculate 10 + 5 * 2 as result
say "The answer is {result}"
```

---

## ⚖️ Module 4: Decision Logic
Use `is-equal-to`, `is-greater-than`, `is-less-than`, or `contains`.
```text
if {result} is-greater-than 15 do:
    say "That is a big number!"
end
```

---

## 🏗️ Module 5: Custom Skills (Functions)
Teach the language new tricks.
```text
to cleanup do:
    say "Cleaning up temporary files..."
    run system command del *.tmp
end

# Use it by just typing the name:
cleanup
```

---

## 📂 Module 6: System Bridge
```text
run system command start chrome
run system command dir
```

---

## 🚀 Graduation Project: The Pro Assistant
Save this as `pro.eng` to see everything working perfectly:

```text
print --- English-Lang v2.5 Stable Assistant ---

# 1. Ask and calculate
ask "Enter a number:" as num
calculate {num} * {num} as squared
say "The square of {num} is {squared}."

# 2. Logic Check
if {squared} is-greater-than 100 do:
    say "Wow, that is a huge result!"
end

# 3. List Processing
create list "Logs, Backups, Reports" as folders
for each folder in folders do:
    say "Creating {folder}..."
    run system command mkdir {folder}
end

say "All tasks complete."
```

---

## 🛠️ How to Run
1. Save your file with a **`.eng`** extension.
2. Double-click it or run: `.\English-Lang.exe myscript.eng`

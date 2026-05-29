# 🎓 English-Lang v2.0: The Easiest Language in the World

Welcome to the **New & Improved** English-Lang. We have upgraded the engine to support Functions, Lists, Logic, and Math while keeping it 100% human-readable.

---

## 🏁 Module 1: The Basics
Every program starts with communication and timing.

### 1.1 Printing
```text
print Hello, world!
```

### 1.2 Waiting
```text
wait 2
```

---

## 🧠 Module 2: Memory & Interaction
Variables allow your script to "remember" information.

### 2.1 Storing Data
```text
store "Angelo" as creator
print This language was made by {creator}.
```

### 2.2 Asking the User
```text
ask user "What is your favorite color?" as fav_color
print Oh, {fav_color} is a nice color!
```

---

## ➗ Module 3: Math Engine
English-Lang can now handle complex calculations.
```text
calculate 10 + 5 * 2 as result
print The answer is {result}
```

---

## 🔄 Module 4: Loops (Scanning Lists)
You can now process lists of information easily.
```text
store "Apple, Banana, Pear" as fruits
for each f in {fruits} do:
    print I like {f}!
end
```

---

## ⚖️ Module 5: Comparison Logic
Make your program "think" and make decisions.
```text
if age is-greater-than 18 then: print You are an adult.
if name is-equal-to "Angelo" then: print Welcome back, Boss.
```

---

## 🛠️ Module 6: Functions (New Skills)
Group commands together to create a reusable skill.
```text
to say hi do:
    print Hello!
    print How are you today?
end

# To use it, just type the name:
say hi
```

---

## 📂 Module 7: System Mastery
Run **any** Windows command using the System Bridge.
```text
run system command dir
run system command start chrome https://google.com
```

---

## 🚀 Graduation Project: The v2 Assistant
Copy this into a `.eng` file to see the full power:

```text
# Define a skill
to scan files do:
    print Scanning directory...
    run system command dir
end

print --- English-Lang v2 Assistant ---

ask user "What is your name?" as name
print Welcome, {name}!

# Use a loop
store "CPU, Memory, Disk" as checks
for each item in {checks} do:
    print Checking {item}...
end

# Use a function
scan files

print Done. Have a great day {name}!
```

---

## 🛠️ How to Run
1. Save as `.eng`.
2. Double-click or run: `.\English-Lang.exe myscript.eng`

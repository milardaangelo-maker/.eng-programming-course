# 🎓 English-Lang Masterclass: Coding in Plain English

Welcome to the official course for **English-Lang**. This language was designed to be powerful, portable, and—most importantly—written exactly like you speak.

---

## 🏁 Module 1: The Basics (Output & Time)
Every program starts with communication and timing.

### 1.1 Printing
To show text on the screen, use the `print` command followed by your message.
```text
print Hello, world!
```

### 1.2 Waiting
To make your script pause (useful for automation), use the `wait` command followed by the number of seconds.
```text
print I am going to sleep...
wait 5
print I am awake!
```

---

## 🧠 Module 2: The Brain (Variables)
Variables allow your script to "remember" information.

### 2.1 Storing Data
Use the `store [VALUE] as [NAME]` syntax.
```text
store "Angelo" as creator
```

### 2.2 Using Data
To use a stored variable, wrap its name in curly braces `{}`.
```text
print This language was made by {creator}.
```

---

## 📂 Module 3: System Mastery (Infinite Power)
This is where English-Lang becomes "General Purpose."

### 3.1 Creating Files
You can generate text files instantly.
```text
create file notes.txt with content This is a note for {creator}.
```

### 3.2 The "System Bridge"
The `run system command` is your secret weapon. It allows you to run **any** command that works in your Command Prompt (CMD).
*   **List files:** `run system command dir`
*   **Open a website:** `run system command start https://google.com`
*   **Shutdown (Be careful!):** `run system command shutdown /s`

---

## 🌐 Module 4: The Web (Discord)
Connecting your script to the internet.

### 4.1 Discord Webhooks
Send notifications to your Discord server in one line.
```text
send to discord webhook:YOUR_LINK_HERE Hello from English-Lang!
```

---

## 🚀 Graduation Project: The Automated Reporter
Let's combine everything into a single automation script.

**Goal:** Create a file, read it, and send a notification.

```text
# Step 1: Set up data
store "Report_v1" as filename
store "https://discord.com/api/webhooks/..." as my_webhook

# Step 2: Action
print Generating {filename}...
create file {filename}.txt with content Script finished successfully at 12:00.

# Step 3: Verify & Notify
print Sending update to Discord...
send to discord webhook:{my_webhook} The {filename} is ready!
run system command type {filename}.txt

print Mission Accomplished.
```

---

## 🛠️ How to Run Your Scripts
1. Save your code in a file ending in `.eng` (e.g., `myscript.eng`).
2. Open CMD or PowerShell.
3. Type: `.\English-Lang.exe myscript.eng`

**You are now a Master of English-Lang!**

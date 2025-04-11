# ETF (Eat The Frog)

A fast, extensible, and common-sense approach to task management. With foundations of plain-text and command-line accessibility, this CLI tool helps to record, organize, and facilitate completion of your tasks.

This tool was born out of the need to wake up and "Eat the frog" - meaning that you should get your hardest task done first-thing even if it's uncomfortable.

TODO Insert screenshot/gif here?

## Features

- [ ] Global and local contexts
- [ ] Easy configuration with sane defaults
- [ ] Quickly add tasks with minimal friction
- [ ] Filter tasks with easy syntax
- [ ]

## Task/File Format

ETF only stores tasks in plaintext, so readability is key. Besides the task name, a field starts with a unique symbol and ends with a `;` (semicolon). The following fields are currently supported:

- Task Name
- `-` Task Notes
- `@` Due Date
- `>` Start Date
- `!` Priority
- `#` Tags

TODO Is this really what I want to do? Or should I just have tasks stored as YAML? That could leverage existing libraries and save me from writing a parser of files (just inputs)

## Brain dump

- Rather than implement "recurring" tasks, leave that up to users to schedule cron jobs or whatever they need.
- Build some kind of "hook" on task completion for extensibility

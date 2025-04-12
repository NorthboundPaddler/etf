# ETF (Eat The Frog)

A fast, extensible, and common-sense approach to task management. With foundations of plain-text and command-line accessibility, this CLI tool helps to record, organize, and facilitate completion of your tasks.

This tool was born out of the need to wake up and "Eat the frog" - meaning that you should get your hardest task done first-thing even if it's uncomfortable.

TODO Insert screenshot/gif here?

## Features

- [ ] Global and local contexts
- [ ] Easy configuration with sane defaults
- [ ] Quickly add tasks with minimal friction
- [ ] Filter tasks with easy syntax

## Task/File Format

ETF only stores tasks in plaintext (YAML), so readability is key. The symbols used to enter each field should be user configurable.

The following 'fields' with the listed symbol as default will be used:

- Task Name
- `-` Task Notes
- `@` Due Date
- `>` Start Date
- `!` Priority
- `#` Tags

A key part of task management is the quick entry of tasks into the system, irrespective of how 'formal' they're entered. For example, Todoist has an excellent natural language processing tool to handle filling in metadata on a new task using a single text entry.

## Brain dump

- Rather than implement "recurring" tasks, leave that up to users to schedule cron jobs or whatever they need.
- Build some kind of "hook" on task completion for extensibility
- For extension of this CLI tool into a service, a server could just field inputs and pass them to this binary.
- Tasks should be entered either with discrete CLI flags for each field or with a parsed "natural language" input.

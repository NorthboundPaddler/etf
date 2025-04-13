# ETF (Eat The Frog)

A fast, extensible, and common-sense approach to task management. With foundations of plain-text and command-line accessibility, this CLI tool helps to record, organize, and facilitate completion of your tasks.

This tool was born out of the need to wake up and "Eat the frog" - meaning that you should get your hardest task done first-thing even if it's uncomfortable.

TODO Insert screenshot/gif here?

## Features

- [ ] Global and local contexts
- [ ] Easy configuration with sane defaults
- [ ] Quickly add tasks with minimal friction
- [ ] Filter tasks with easy syntax
- [ ] Extensible and easy to debug

## Task/File Format

ETF only stores tasks in plaintext (YAML), so readability is key. The symbols used to enter each field should be user configurable.

The following 'fields' with the listed symbol as default will be used:

- Task ID (auto-generated)
- Task Name (no symbol needed)
- `&` Task Notes
- `@` Due Date
- `>` Start Date
- `!` Priority
- `#` Tags
- Entry Date (auto-generated)
- Completed Date (auto-generated)

A key part of task management is the quick entry of tasks into the system, irrespective of how 'formal' they're entered. For example, Todoist has an excellent natural language processing tool to handle filling in metadata on a new task using a single text entry.

An example entry to add a task would look like so:
`etf add Pick up groceries @tomorrow at 5pm !1 #errand #personal`

Rather than a "Project" or "Group" field, you can use as many tags per task as you wish. At a high level, you could use tags to facilitate a structure like so:

```
- personal
    - errand
    - chores
        * cleaning
    - fitness
    - reading 
- work 
    * application XYZ
        + feature Q 
    * maintenance tasks 
```

While this tool doesn't build a default way to add the tag `#Cleaning` and auto-add the other tags that you've mentally organized it under, you can always write an extension that handles this for you. It may require a definition of such heirarchy, but that's within the realm of how this tool can be used.

## Configuration

The configuration files are also YAML, so they are easy to read and write. The default settings locate a config in `$XDG_CONFIG_HOME` or you can create one in any directory for a local configuration in that context.

The global configuration file should be at the following path: `$XDG_CONFIG_HOME/etf/etf-config.yaml`.

If the `-l` or `--local` argument is used, then a local configuration will be searched for. If the current working directory is inside of a git repository, then a `.etf-config.yaml` file is expected at the root of the repository (i.e. one directory above the `.git` folder). Otherwise, if the current working directory is not inside of a git repo, then `.etf-config.yaml` is expected in the current working directory.

### Syntax

The configuration is written in simple YAML, and currently there's no settings to implement.

## Brain dump

- Rather than implement "recurring" tasks, leave that up to users to schedule cron jobs or whatever they need.
- Build some kind of "hook" on task completion for extensibility
- For extension of this CLI tool into a service, a server could just field inputs and pass them to this binary.
- Tasks should be entered either with discrete CLI flags for each field or with a parsed "natural language" input.

## Roadmap

- [o] Set up and read configuration/data files
  - [X] Read a global config file (YAML), create defaults
  - [X] Read a local config file (YAML), rolling up in working dir if it's a git repo
  - [ ] Write an `init` command to create config, task, and completed YAML files
  - [ ] Generate a local config file from template if missing when -l is called
- [ ] Build CLI for `add` command
  - [ ] Serialize input to YAML
  - [ ] Write to global context "storage" file (i.e. file at known dir)
- [ ] Build CLI for `query` command
  - [ ] Deserialize YAML from global context "storage" file
  - [ ] Output files as either "pretty" or "raw" (JSON?)
- [ ] Build CLI for `complete` command
  - [ ] Move YAML data from global context "storage" file to "done" file
  - [ ] Add a timestamp to the "completed date" field.

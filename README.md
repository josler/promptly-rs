### What?

I often end up repeating sets of console commands over and over again, and often repeating information. One approach to reducing typing would be to script these, but I often want _some_ form of control over what is being executed, with the ability to stop a chain of commands at any point and modify something and continue manually.

Promptly does exactly that, it's a fancy set of scripts that prompts for user input but with sensible defaults.

This used to be in Ruby, but startup costs from Ruby for CLI tools added small but noticable latency, so I rewrote it in Rust.

### Install

Download a release and put it in your path.

Or from source: (Rust 1.30+)

```
git clone git@github.com:josler/promptly-rs.git
cd promptly-rs
cargo install
```

### Dependencies

GitHub's [hub](https://github.com/github/hub), for some functionality. Hopefully in the future this will be removed as a dependency.

### Usage

#### Auto PR

Promptly can automatically follow several steps to generate a PR in the style I'm accustomed:

1. Ask for description
1. Generate branch name from description (can override), check out branch
1. Generate commit from description (can override), commit
1. Ask to push to remote
1. Ask to generate pull request using `hub`
1. Generate PR title from description (can override)

```
$ promptly pr
```

#### Auto CI

Promptly wraps a hub command to get the CI status check for a branch. An alias.

```
$ promptly ci
```

### Configuration

If you want to add a prefix to your branch names (like your initials, for example), then edit the config file at `~/.config/promptly/config.toml` to add a `prefix='jo/'`, or whatever.

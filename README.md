# Symbolic Link Generator

This Rust program generates symbolic links based on a configuration provided in a JSON file.

## Usage

1. Clone or download this repository.
2. Install Rust and its dependencies if you haven't already.
3. Create a configuration file named `config.json` in the project directory. The configuration format should follow the example below:

```json
{
  "folders": [
    {
      "path": "path/to/source/folder1",
      "exclude": ["excluded_folder1", "excluded_folder2"]
    },
    {
      "path": "path/to/source/folder2",
      "exclude": ["excluded_folder3"]
    }
  ],
  "symlink_root": "path/to/target/folder"
}
```

# Regular Expressions (Regex) Basics

Regular expressions (regex or regexp) are powerful tools for pattern matching and text manipulation. They are widely used in various programming languages and tools to find, match, and manipulate strings based on certain patterns. In the context of the Symbolic Link Generator, regex is used to define exclusion patterns for files and folders.

## Basic Syntax

- `.`: Matches any character except a newline.
- `*`: Matches zero or more occurrences of the previous character or group.
- `+`: Matches one or more occurrences of the previous character or group.
- `?`: Matches zero or one occurrence of the previous character or group.
- `\`: Escapes a special character.
- `[]`: Defines a character class.
- `()`: Groups expressions together.

## Examples

- `abc`: Matches the string "abc".
- `a.c`: Matches "a", followed by any character, then "c" (e.g., "abc", "axc").
- `a*`: Matches "a" zero or more times (e.g., "a", "aa", "aaa").
- `a+`: Matches "a" one or more times (e.g., "a", "aa", "aaa", but not "").
- `a?`: Matches "a" zero or one time (e.g., "a", or "").
- `a\.`: Matches "a" followed by a dot (e.g., "a.").
- `[abc]`: Matches any of the characters "a", "b", or "c".
- `(abc)`: Groups the pattern "abc" together.

## Using Regex in Symbolic Link Generator

In the Symbolic Link Generator, regex is used to specify exclusion patterns for files and folders. For example:
- `.*\.exe`: Excludes all files with a ".exe" extension.
- `folder\d{2}`: Excludes folders with names like "folder01", "folder02", etc.
- `.*\.log|.*\.txt`: Excludes both ".log" and ".txt" files.

Please note that regex patterns can become complex for advanced matching. Test and verify your patterns before using them to ensure the desired behavior.

For more details on regex syntax and usage, you can refer to [regex documentation](https://docs.rs/regex/1.4.3/regex/index.html).

---
Remember that this explanation provides only a basic overview of regex. It's recommended to learn more about regex to effectively use it for various tasks.

# ai-ripgrep
Combining AI with ripgrep. So that you'll not have to remember the regex for searching particular term or doing the action you want. Planning same to integrate with groq to have a speedy AI serach and buiding on the top of what ripgrep already has. 

### Todo
- [ ] Adding groq based search. 
- [ ] Adding Openai support. 
- [ ] Natural language support 

## Some features ripgrep already have 
- Text Searching: Quickly searches for text patterns within files.
- Recursive Directory Search: Recursively searches directories for matching text.
- Regular Expressions: Supports searching with regular expressions.
- Binary File Exclusion: Automatically skips binary files when searching.
- Ignore Files Support: Honors .gitignore, .ignore, and .rgignore files to skip files or directories during searches.
- Context Display: Can display lines of context before or after matches (similar to grep -C).
- Multithreading: Utilizes multiple CPU cores to speed up search operations.
- Cross-Platform: Works on Linux, macOS, and Windows.
- File Type Filtering: Allows filtering by file types (e.g., rg --type rust to search only Rust files).
- Colorized Output: Provides colorized output for easier readability of search results.
- File Inclusion/Exclusion: Includes or excludes specific files or directories using command-line options.
- Case Insensitivity: Offers case-insensitive search options.
- Search with File Patterns: Supports searching only within files matching specific patterns.
- Line Number Display: Displays line numbers where matches occur within files.
- Performance Optimization: Optimized for performance, making it faster than traditional grep in many scenarios.

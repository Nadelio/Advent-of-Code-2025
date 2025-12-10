Regex:
- Fail match if start with 0 -> `^[1-9]`
- Fail match if doesn't match a number -> `\d*`
- Fail match if doesn't match two copies of first match -> `\1`
- Fail match if contains more than the above patterns -> `$`
Full pattern: `^([1-9]\d*)\1$`

## Counter
A simple counter contract in sway

### Storage
```
# 64bit integer to store value for count
counter
```

### Abi
```
# fn (read, write), updates the counter on call by +1
fn increment();

# fn (read, write), updates the counter on call by -1
fn decrement();

# fn (read), reads the counter value
fn count() -> u64;
```


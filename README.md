-- --
# Hey! What's this?

This is my foreign word learning app written in rust. It displays a window with words every N minutes and allows you to repeat words from time to time.

## Install
```bash
git clone https://github.com/acm-wq/shark.git
cd shark
# Before that you can change the constant TIMER_INTERVAL_SECS to change the time interval of the word window output
cargo run
```

## Todo

- [ ] Outputting sentences or groups of words instead of just one
- [ ] Changing the time at the moment of application launch
- [ ] Continuation of the program operation when closing the window of adding words

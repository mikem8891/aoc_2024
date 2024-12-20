use std::time::{Duration, Instant};

pub fn stopwatch<T, F: FnOnce() -> T>(func: F) -> (T, Duration){
  let time = Instant::now();
  let result = func();
  let elapsed = time.elapsed();
  (result, elapsed)
}

macro_rules! stopwatch {
    ($func: expr) => {{
      let time = std::time::Instant::now();
      let result = $func;
      let elapsed = time.elapsed();
      (result, elapsed)
  }};
}
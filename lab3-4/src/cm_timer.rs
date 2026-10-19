use std::time::Instant;

pub struct Timer {
    started: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("time: {:.6}", self.started.elapsed().as_secs_f64());
    }
}

pub fn time_it<F, T>(action: F) -> T
where
    F: FnOnce() -> T,
{
    let started = Instant::now();
    let result = action();
    println!("time: {:.6}", started.elapsed().as_secs_f64());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_runs_once_and_can_move_its_result() {
        let mut calls = 0;
        let text = String::from("готово");
        let result = time_it(|| {
            calls += 1;
            text
        });
        assert_eq!(calls, 1);
        assert_eq!(result, "готово");
    }
}

use std::{
    cmp::{max, Ordering},
    collections::BinaryHeap,
    num::NonZeroUsize,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Worker {
    free_at: i32,
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Worker) -> Option<Ordering> {
        other.free_at.partial_cmp(&self.free_at)
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Worker) -> Ordering {
        other.free_at.cmp(&self.free_at)
    }
}

impl Worker {
    fn free_at(&self) -> i32 {
        self.free_at
    }

    fn schedule_work(&mut self, start_at: i32, work_duration: i32) {
        self.free_at = max(self.free_at, start_at) + work_duration;
    }
}

#[derive(Debug)]
pub(crate) struct Pool {
    workers: BinaryHeap<Worker>,
}

impl Pool {
    pub fn new(n_workers: NonZeroUsize) -> Self {
        let workers = (0..n_workers.get()).map(|_| Worker::default()).collect();

        Self {
            workers
        }
    }

    pub fn take_work(&mut self, start_at: i32, work_duration: i32) -> i32 {
        let mut worker = self.workers.pop().unwrap();

        worker.schedule_work(start_at, work_duration);
        let will_free = worker.free_at();

        self.workers.push(worker);

        will_free
    }

    pub fn free_at(&self) -> i32 {
        self.workers
            .iter()
            .map(|worker| worker.free_at())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker() {
        let mut worker: Worker = Default::default();

        assert_eq!(worker.free_at, 0);
        assert_eq!(worker.free_at, worker.free_at());

        worker.schedule_work(0, 0);

        assert_eq!(worker.free_at, 0);
        assert_eq!(worker.free_at, worker.free_at());

        worker.schedule_work(0, 15);

        assert_eq!(worker.free_at, 15);
        assert_eq!(worker.free_at, worker.free_at());

        worker.schedule_work(0, 325);

        assert_eq!(worker.free_at, 340);
        assert_eq!(worker.free_at, worker.free_at());

        worker.schedule_work(1000, 5);

        assert_eq!(worker.free_at, 1005);
        assert_eq!(worker.free_at, worker.free_at());
    }

    #[test]
    fn test_worker_priority() {
        let mut worker1: Worker = Default::default();
        let mut worker2: Worker = Default::default();

        worker1.schedule_work(0, 10);
        worker2.schedule_work(0, 20);

        // worker2 will be free later, so he has lesser priority
        assert!(worker1 > worker2);
    }

    #[test]
    fn test_pool() {
        let mut pool = Pool::new(NonZeroUsize::new(3).unwrap());

        // three workers with times:
        // 0, 0, 0
        assert_eq!(pool.free_at(), 0);

        pool.take_work(0, 10);
        // three workers with times:
        // 10, 0, 0
        assert_eq!(pool.free_at(), 10);

        pool.take_work(0, 10);
        // three workers with times:
        // 10, 10, 0
        assert_eq!(pool.free_at(), 10);

        pool.take_work(0, 10);
        // three workers with times:
        // 10, 10, 10
        assert_eq!(pool.free_at(), 10);

        pool.take_work(0, 15);
        // three workers with times:
        // 25, 10, 10
        assert_eq!(pool.free_at(), 25);

        pool.take_work(0, 5);
        // three workers with times:
        // 25, 15, 10
        assert_eq!(pool.free_at(), 25);

        pool.take_work(100, 5);
        // three workers with times:
        // 25, 15, 105
        assert_eq!(pool.free_at(), 105);
    }
}

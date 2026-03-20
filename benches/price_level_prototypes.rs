use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::{HashMap, VecDeque};
use std::hint::black_box;

#[derive(Default)]
struct VecDequeLevel {
    orders: VecDeque<u64>,
}

impl VecDequeLevel {
    fn enqueue(&mut self, order_id: u64) {
        self.orders.push_back(order_id);
    }

    fn pop_front(&mut self) -> Option<u64> {
        self.orders.pop_front()
    }

    fn remove(&mut self, order_id: u64) -> Option<u64> {
        let position = self.orders.iter().position(|id| *id == order_id)?;
        self.orders.remove(position)
    }
}

#[derive(Clone, Copy)]
struct SlabNode {
    order_id: u64,
    prev: Option<usize>,
    next: Option<usize>,
    occupied: bool,
}

#[derive(Default)]
struct SlabLevel {
    nodes: Vec<SlabNode>,
    free: Vec<usize>,
    index: HashMap<u64, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl SlabLevel {
    fn enqueue(&mut self, order_id: u64) {
        let slot = self.free.pop().unwrap_or_else(|| {
            self.nodes.push(SlabNode {
                order_id: 0,
                prev: None,
                next: None,
                occupied: false,
            });
            self.nodes.len() - 1
        });

        self.nodes[slot] = SlabNode {
            order_id,
            prev: self.tail,
            next: None,
            occupied: true,
        };

        match self.tail {
            Some(tail) => self.nodes[tail].next = Some(slot),
            None => self.head = Some(slot),
        }
        self.tail = Some(slot);
        self.index.insert(order_id, slot);
    }

    fn pop_front(&mut self) -> Option<u64> {
        let head = self.head?;
        let order_id = self.nodes[head].order_id;
        self.remove(order_id)
    }

    fn remove(&mut self, order_id: u64) -> Option<u64> {
        let slot = self.index.remove(&order_id)?;
        let node = self.nodes[slot];

        match node.prev {
            Some(prev) => self.nodes[prev].next = node.next,
            None => self.head = node.next,
        }
        match node.next {
            Some(next) => self.nodes[next].prev = node.prev,
            None => self.tail = node.prev,
        }

        self.nodes[slot].occupied = false;
        self.free.push(slot);
        Some(node.order_id)
    }
}

#[derive(Clone, Copy)]
struct DenseNode {
    prev: Option<usize>,
    next: Option<usize>,
    occupied: bool,
}

struct DenseIndexedLevel {
    nodes: Vec<DenseNode>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl DenseIndexedLevel {
    fn with_capacity(count: usize) -> Self {
        Self {
            nodes: vec![
                DenseNode {
                    prev: None,
                    next: None,
                    occupied: false,
                };
                count
            ],
            head: None,
            tail: None,
        }
    }

    fn enqueue(&mut self, order_id: u64) {
        let slot = order_id as usize;
        self.nodes[slot] = DenseNode {
            prev: self.tail,
            next: None,
            occupied: true,
        };

        match self.tail {
            Some(tail) => self.nodes[tail].next = Some(slot),
            None => self.head = Some(slot),
        }
        self.tail = Some(slot);
    }

    fn pop_front(&mut self) -> Option<u64> {
        let head = self.head?;
        self.remove(head as u64)
    }

    fn remove(&mut self, order_id: u64) -> Option<u64> {
        let slot = order_id as usize;
        let node = self.nodes.get(slot).copied()?;
        if !node.occupied {
            return None;
        }

        match node.prev {
            Some(prev) => self.nodes[prev].next = node.next,
            None => self.head = node.next,
        }
        match node.next {
            Some(next) => self.nodes[next].prev = node.prev,
            None => self.tail = node.prev,
        }

        self.nodes[slot].occupied = false;
        self.nodes[slot].prev = None;
        self.nodes[slot].next = None;
        Some(order_id)
    }
}

fn seed_vecdeque(count: u64) -> VecDequeLevel {
    let mut level = VecDequeLevel::default();
    for order_id in 0..count {
        level.enqueue(order_id);
    }
    level
}

fn seed_slab(count: u64) -> SlabLevel {
    let mut level = SlabLevel::default();
    for order_id in 0..count {
        level.enqueue(order_id);
    }
    level
}

fn seed_dense(count: u64) -> DenseIndexedLevel {
    let mut level = DenseIndexedLevel::with_capacity(count as usize);
    for order_id in 0..count {
        level.enqueue(order_id);
    }
    level
}

fn enqueue_vecdeque(c: &mut Criterion) {
    c.bench_function("prototype_enqueue_vecdeque_1000", |b| {
        b.iter(|| {
            let mut level = VecDequeLevel::default();
            for order_id in 0..1_000 {
                level.enqueue(order_id);
            }
            black_box(level);
        });
    });
}

fn enqueue_slab(c: &mut Criterion) {
    c.bench_function("prototype_enqueue_slab_1000", |b| {
        b.iter(|| {
            let mut level = SlabLevel::default();
            for order_id in 0..1_000 {
                level.enqueue(order_id);
            }
            black_box(level);
        });
    });
}

fn enqueue_dense(c: &mut Criterion) {
    c.bench_function("prototype_enqueue_dense_1000", |b| {
        b.iter(|| {
            let mut level = DenseIndexedLevel::with_capacity(1_000);
            for order_id in 0..1_000 {
                level.enqueue(order_id);
            }
            black_box(level);
        });
    });
}

fn fifo_sweep_vecdeque(c: &mut Criterion) {
    c.bench_function("prototype_fifo_sweep_vecdeque_1000", |b| {
        b.iter(|| {
            let mut level = seed_vecdeque(1_000);
            while level.pop_front().is_some() {}
            black_box(level);
        });
    });
}

fn fifo_sweep_slab(c: &mut Criterion) {
    c.bench_function("prototype_fifo_sweep_slab_1000", |b| {
        b.iter(|| {
            let mut level = seed_slab(1_000);
            while level.pop_front().is_some() {}
            black_box(level);
        });
    });
}

fn fifo_sweep_dense(c: &mut Criterion) {
    c.bench_function("prototype_fifo_sweep_dense_1000", |b| {
        b.iter(|| {
            let mut level = seed_dense(1_000);
            while level.pop_front().is_some() {}
            black_box(level);
        });
    });
}

fn deep_cancel_vecdeque(c: &mut Criterion) {
    c.bench_function("prototype_deep_cancel_vecdeque_1000", |b| {
        b.iter(|| {
            let mut level = seed_vecdeque(1_000);
            for order_id in (0..1_000).rev() {
                black_box(level.remove(order_id));
            }
            black_box(level);
        });
    });
}

fn deep_cancel_slab(c: &mut Criterion) {
    c.bench_function("prototype_deep_cancel_slab_1000", |b| {
        b.iter(|| {
            let mut level = seed_slab(1_000);
            for order_id in (0..1_000).rev() {
                black_box(level.remove(order_id));
            }
            black_box(level);
        });
    });
}

fn deep_cancel_dense(c: &mut Criterion) {
    c.bench_function("prototype_deep_cancel_dense_1000", |b| {
        b.iter(|| {
            let mut level = seed_dense(1_000);
            for order_id in (0..1_000).rev() {
                black_box(level.remove(order_id));
            }
            black_box(level);
        });
    });
}

criterion_group!(
    benches,
    enqueue_vecdeque,
    enqueue_slab,
    enqueue_dense,
    fifo_sweep_vecdeque,
    fifo_sweep_slab,
    fifo_sweep_dense,
    deep_cancel_vecdeque,
    deep_cancel_slab,
    deep_cancel_dense
);
criterion_main!(benches);

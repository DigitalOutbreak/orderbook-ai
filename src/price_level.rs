use crate::events::BookLevelView;
use crate::order::Order;
use crate::types::{OrderId, Price, Quantity};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq)]
struct LevelNode {
    order: Option<Order>,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriceLevel {
    price: Price,
    nodes: Vec<LevelNode>,
    free: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
    order_count: usize,
    total_quantity: Quantity,
}

impl PriceLevel {
    pub fn new(price: Price) -> Self {
        Self {
            price,
            nodes: Vec::new(),
            free: Vec::new(),
            head: None,
            tail: None,
            order_count: 0,
            total_quantity: Quantity::zero(),
        }
    }

    pub fn price(&self) -> Price {
        self.price
    }

    pub fn is_empty(&self) -> bool {
        self.order_count == 0
    }

    #[cfg(test)]
    pub fn order_count(&self) -> usize {
        self.order_count
    }

    #[cfg(test)]
    pub fn total_quantity(&self) -> Quantity {
        self.total_quantity
    }

    pub fn enqueue(&mut self, order: Order) -> usize {
        let slot = self.allocate_slot(order);
        match self.tail {
            Some(tail) => {
                self.nodes[tail].next = Some(slot);
                self.nodes[slot].prev = Some(tail);
            }
            None => {
                self.head = Some(slot);
            }
        }
        self.tail = Some(slot);
        slot
    }

    pub fn pop_front(&mut self) -> Option<(Order, usize)> {
        let slot = self.head?;
        self.remove_at(slot)
    }

    pub fn push_front(&mut self, order: Order) -> usize {
        let slot = self.allocate_slot(order);
        match self.head {
            Some(head) => {
                self.nodes[head].prev = Some(slot);
                self.nodes[slot].next = Some(head);
            }
            None => {
                self.tail = Some(slot);
            }
        }
        self.head = Some(slot);
        slot
    }

    pub fn remove(&mut self, order_id: OrderId, slot: usize) -> Option<Order> {
        let node = self.nodes.get(slot)?;
        let order = node.order.as_ref()?;
        if order.order_id != order_id {
            return None;
        }
        self.remove_at(slot).map(|(order, _)| order)
    }

    pub fn view(&self) -> BookLevelView {
        BookLevelView {
            price: self.price,
            total_quantity: self.total_quantity,
            order_count: self.order_count,
        }
    }

    pub fn order_at(&self, slot: usize) -> Option<&Order> {
        self.nodes.get(slot)?.order.as_ref()
    }

    pub fn iter_entries(&self) -> PriceLevelEntryIter<'_> {
        PriceLevelEntryIter {
            level: self,
            current: self.head,
        }
    }

    fn allocate_slot(&mut self, order: Order) -> usize {
        let slot = self.free.pop().unwrap_or_else(|| {
            self.nodes.push(LevelNode {
                order: None,
                prev: None,
                next: None,
            });
            self.nodes.len() - 1
        });
        self.total_quantity =
            Quantity::new(self.total_quantity.value() + order.remaining_qty.value());
        self.nodes[slot] = LevelNode {
            order: Some(order),
            prev: None,
            next: None,
        };
        self.order_count += 1;
        slot
    }

    fn remove_at(&mut self, slot: usize) -> Option<(Order, usize)> {
        let prev = self.nodes.get(slot)?.prev;
        let next = self.nodes.get(slot)?.next;

        match prev {
            Some(prev_slot) => self.nodes[prev_slot].next = next,
            None => self.head = next,
        }
        match next {
            Some(next_slot) => self.nodes[next_slot].prev = prev,
            None => self.tail = prev,
        }

        let mut node = LevelNode {
            order: None,
            prev: None,
            next: None,
        };
        std::mem::swap(&mut node, &mut self.nodes[slot]);

        let order = node.order?;
        self.total_quantity =
            Quantity::new(self.total_quantity.value() - order.remaining_qty.value());
        self.order_count -= 1;
        self.free.push(slot);

        if self.order_count == 0 {
            self.head = None;
            self.tail = None;
        }

        Some((order, slot))
    }
}

// This level keeps FIFO and arbitrary removal in the same structure:
//
// - enqueue at the tail: O(1)
// - pop the head during matching: O(1)
// - remove by known slot during cancel: O(1)
//
// The cost is a bit more bookkeeping than `VecDeque<Order>`, but the engine no
// longer has to linearly scan deep price levels during cancellation.

pub struct PriceLevelEntryIter<'a> {
    level: &'a PriceLevel,
    current: Option<usize>,
}

impl<'a> Iterator for PriceLevelEntryIter<'a> {
    type Item = (usize, &'a Order);

    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.current?;
        let node = self.level.nodes.get(slot)?;
        let order = node.order.as_ref()?;
        self.current = node.next;
        Some((slot, order))
    }
}

impl Default for PriceLevel {
    fn default() -> Self {
        Self::new(Price::new(Decimal::ZERO))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::{NewOrderRequest, Order, OrderType, Side};
    use crate::types::{MarketId, SequenceNumber};
    use rust_decimal_macros::dec;

    fn make_order(order_id: u64, sequence: u64, qty: Decimal) -> Order {
        Order::from_request(
            OrderId::new(order_id),
            SequenceNumber::new(sequence),
            NewOrderRequest {
                market_id: MarketId::from("SOL/USDC"),
                side: Side::Buy,
                order_type: OrderType::Limit,
                quantity: Quantity::new(qty),
                price: Some(Price::new(dec!(100.00))),
            },
        )
    }

    #[test]
    fn preserves_fifo_order() {
        let mut level = PriceLevel::new(Price::new(dec!(100.00)));
        level.enqueue(make_order(1, 1, dec!(1.000)));
        level.enqueue(make_order(2, 2, dec!(2.000)));

        let (first, _) = level.pop_front().unwrap();
        let (second, _) = level.pop_front().unwrap();

        assert_eq!(first.order_id, OrderId::new(1));
        assert_eq!(second.order_id, OrderId::new(2));
    }

    #[test]
    fn removes_specific_order_and_updates_total_quantity() {
        let mut level = PriceLevel::new(Price::new(dec!(100.00)));
        let slot_one = level.enqueue(make_order(1, 1, dec!(1.500)));
        level.enqueue(make_order(2, 2, dec!(2.000)));

        let removed = level.remove(OrderId::new(1), slot_one).unwrap();

        assert_eq!(removed.order_id, OrderId::new(1));
        assert_eq!(level.total_quantity(), Quantity::new(dec!(2.000)));
        assert_eq!(level.order_count(), 1);
    }
}

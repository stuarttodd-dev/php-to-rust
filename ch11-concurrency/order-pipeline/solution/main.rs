use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: u64,
    pub product_id: u64,
    pub qty: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderResult {
    Reserved { id: u64 },
    OutOfStock { id: u64 },
}
pub fn run(
    orders: Vec<Order>,
    stock: HashMap<u64, u32>,
    workers: usize,
) -> (Vec<OrderResult>, HashMap<u64, u32>) {
    let (tx, rx) = mpsc::channel::<Order>();
    let rx = Arc::new(Mutex::new(rx));
    let stock = Arc::new(Mutex::new(stock));
    let (result_tx, result_rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..workers {
        let rx = Arc::clone(&rx);
        let stock = Arc::clone(&stock);
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let message = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };
                let Ok(order) = message else {
                    break;
                };
                let result = {
                    let mut guard = stock.lock().unwrap();
                    let available = guard.get(&order.product_id).copied().unwrap_or(0);
                    if available < order.qty {
                        OrderResult::OutOfStock { id: order.id }
                    } else {
                        guard.insert(order.product_id, available - order.qty);
                        OrderResult::Reserved { id: order.id }
                    }
                };
                result_tx.send(result).unwrap();
            }
        }));
    }
    drop(result_tx);
    for order in orders {
        tx.send(order).unwrap();
    }
    drop(tx);
    for handle in handles {
        handle.join().unwrap();
    }
    let mut results: Vec<OrderResult> = result_rx.into_iter().collect();
    results.sort_by_key(|result| match result {
        OrderResult::Reserved { id } | OrderResult::OutOfStock { id } => *id,
    });
    let stock = Arc::try_unwrap(stock).unwrap().into_inner().unwrap();
    (results, stock)
}

fn main() {
    let orders = vec![Order {
        id: 1,
        product_id: 1,
        qty: 1,
    }];
    let stock = HashMap::from([(1u64, 5u32)]);
    let _ = run(orders, stock, 1);
}
#[cfg(test)]
mod tests {
    use super::{Order, OrderResult, run};
    use std::collections::HashMap;
    fn order(id: u64, product_id: u64, qty: u32) -> Order {
        Order {
            id,
            product_id,
            qty,
        }
    }
    #[test]
    fn all_reserved_when_stock_allows() {
        let orders = vec![order(2, 1, 1), order(1, 2, 1)];
        let stock = HashMap::from([(1u64, 5u32), (2, 3)]);
        let (results, _) = run(orders, stock, 2);
        assert_eq!(
            results,
            vec![
                OrderResult::Reserved { id: 1 },
                OrderResult::Reserved { id: 2 },
            ]
        );
    }
    #[test]
    fn out_of_stock_leaves_stock_unchanged() {
        let orders = vec![order(2, 1, 1), order(1, 3, 2)];
        let stock = HashMap::from([(1u64, 5u32), (3, 1)]);
        let (results, stock) = run(orders, stock, 4);
        assert_eq!(
            results,
            vec![
                OrderResult::OutOfStock { id: 1 },
                OrderResult::Reserved { id: 2 },
            ]
        );
        assert_eq!(stock.get(&3), Some(&1));
    }
    #[test]
    fn results_are_sorted_by_order_id() {
        let orders = vec![order(3, 1, 1), order(1, 2, 1), order(2, 3, 1)];
        let stock = HashMap::from([(1u64, 5u32), (2, 5), (3, 5)]);
        let (results, _) = run(orders, stock, 4);
        let ids: Vec<u64> = results
            .iter()
            .map(|result| match result {
                OrderResult::Reserved { id } | OrderResult::OutOfStock { id } => *id,
            })
            .collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }
    #[test]
    fn one_worker_and_four_workers() {
        let orders = vec![order(2, 1, 2), order(1, 3, 1)];
        let stock = HashMap::from([(1u64, 5u32), (3, 4)]);
        let (one, _) = run(orders.clone(), stock.clone(), 1);
        let (four, _) = run(orders, stock, 4);
        assert_eq!(one, four);
        assert_eq!(
            one,
            vec![
                OrderResult::Reserved { id: 1 },
                OrderResult::Reserved { id: 2 },
            ]
        );
    }
}

use std::time::Duration;

pub async fn fetch_price(id: u64) -> u32 {
    tokio::time::sleep(Duration::from_millis(5)).await;
    match id {
        1 => 1000,
        2 => 2500,
        3 => 800,
        _ => 800,
    }
}

pub async fn sum_blocking() -> u64 {
    tokio::task::spawn_blocking(|| (1_u64..=100).sum())
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn join_two_prices() {
        let (a, b) = tokio::join!(fetch_price(1), fetch_price(2));
        assert_eq!(a, 1000);
        assert_eq!(b, 2500);
    }

    #[tokio::test]
    async fn timeout_can_fail() {
        let result = tokio::time::timeout(Duration::from_millis(1), fetch_price(1)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn bounded_channel() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        tx.send(42).await.unwrap();
        assert_eq!(rx.recv().await, Some(42));
    }

    #[tokio::test]
    async fn blocking_sum() {
        assert_eq!(sum_blocking().await, 5050);
    }
}

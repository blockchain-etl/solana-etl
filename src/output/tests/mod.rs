#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_publisher_connection() {
        let publisher = crate::output::publish::StreamPublisher::new().await;
        //publisher.disconnect().await;
    }
}

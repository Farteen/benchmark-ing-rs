mod test {
    use std::task::Context;
    use tokio::macros::support::{Pin, Poll};
    use tokio_stream::{Stream, StreamExt};

    #[tokio::test]
    async fn test_tokio_stream() {
        let mut stream = tokio_stream::iter(&[1, 2, 3]);
        while let Some(v) = stream.next().await {
            println!("Got = {:?}", v);
        }
    }

    struct Interval {
        rem: usize,
        delay: std::time::Duration,
    }

    impl Stream for Interval {
        type Item = ();

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.rem == 0 {
                return Poll::Ready(None);
            }
            match Pin::new(&mut self.delay).poll(cx) {}
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            unimplemented!()
        }
    }
}

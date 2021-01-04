mod test {
    use tokio::sync::oneshot;
    use tokio::sync::mpsc;
    use tokio::net::{TcpStream, TcpListener};
    use tokio::*;
    use std::future::Future;
    use std::task::Context;
    use tokio::macros::support::{Pin, Poll};
    use std::net::SocketAddr;

    async fn some_operaion() -> String {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        "a".to_string()
    }

    #[tokio::test]
    async fn test_tokio_oneshot() {
        let (mut tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        tokio::spawn(async {
            tokio::select! {
                val = some_operaion() => {
                    let _ = tx1.send(val);
                }
                _ = tx1.closed() => {
                    println!("tx1 closed");
                }
            }
        });
        tokio::spawn(async {
            let _ = tx2.send("two");
        });

        tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
        }
    }

    struct MySelect {
        rx1: oneshot::Receiver<&'static str>,
        rx2: oneshot::Receiver<&'static str>,
    }

    impl Future for MySelect {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
                println!("rx1 completed first with {:p} {:?}", &val, val);
                return Poll::Ready(());
            }

            if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
                println!("rx2 completed second with {:p} {:?}", &val, val);
                return Poll::Ready(());
            }

            Poll::Pending
        }
    }

    #[tokio::test]
    async fn test_better_understand_select() {
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(1));
            let a = "2";
            println!("{:p}", &a);
            tx1.send(a);
        });
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(2));
            tx2.send("1");
        });
        MySelect {
            rx1,
            rx2,
        }.await;
    }

    #[tokio::test]
    async fn test_async_syntax() {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            tx.send("done").unwrap();
        });

        tokio::select! {
        socket = TcpStream::connect("localhost:3456") => {
        println!("socket connected {:?}", socket);
        }
        msg = rx => {
        println!("received message first {:?}", msg);
        }
        }
    }

    #[tokio::test]
    async fn test_accepting_socket() -> io::Result<()> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            tx.send(()).unwrap();
        });

        let mut listener = TcpListener::bind("localhost:3456").await?;
        tokio::select! {
            _ = async {
            loop {
                let (socket, _) = listener.accept().await?;
                // tokio::spawn(async move {process(socket)});
            }
            Ok::<_, io::Error>(())
            } => {}
            _ = rx => {
            println!("terminating accept loop");
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_pattern_matching() {
        let (mut tx1, mut rx1) = mpsc::channel(128);
        let (mut tx2, mut rx2) = mpsc::channel(128);

        tokio::spawn(async move {

        });

        tokio::select! {
            Some(v) = rx1.recv() => {
                println!("Got {:?} from rx1", v);
            }
            Some(v) = rx2.recv() => {
                println!("Got {:?} from rx2", v);
            }
            else {
                println!("Both channels closed");
            }
        }
    }

    //  select! guarantees that only single handler runs
    async fn race(
        data: &[u8],
        addr1: SocketAddr,
        addr2: SocketAddr,
    ) -> io::Result<()> {
        tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await:/
            Ok::<_, io::Error>(())
        } => {}
        else => {}
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_borrowing() {

    }
}

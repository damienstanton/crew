use crate::workgroup::{task, Crew};
use anyhow::Result;
use futures::channel::mpsc::{channel, TryRecvError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrewError {
    #[error("Worker reports a failure: {0}")]
    WorkerFailure(&'static str),
}

pub async fn assemble_crew<T: 'static>(
    of_size: usize,
    executing: fn() -> Result<T, CrewError>,
) -> Result<Option<T>, TryRecvError>
where
    T: Send + Copy,
{
    let crew = Crew::new();
    let (tx, mut rx) = channel::<T>(of_size);
    for _ in 0..of_size {
        let mut tx_ref = tx.clone();
        let m = crew.member();
        task::spawn(async move {
            match executing() {
                Ok(t) => match tx_ref.try_send(t) {
                    Ok(_) => (),
                    Err(_) => (),
                },
                Err(e) => {
                    eprintln!("Crew task interrupted by worker {}. Cause: {}", m.id, e);
                }
            };
            drop(m);
        });
    }
    crew.block().await;
    rx.try_next()
}

#[cfg(test)]
mod test {
    use super::{assemble_crew, CrewError};
    #[async_std::test]
    async fn single_crew() {
        fn f() -> Result<i32, CrewError> {
            Ok(42)
        }
        let val = assemble_crew::<i32>(1, f).await;
        assert_eq!(val.unwrap(), Some(42));
    }
    #[async_std::test]
    async fn single_crew_failure() {
        fn f() -> Result<i32, CrewError> {
            Err(CrewError::WorkerFailure("I just can't"))
        }
        let val = assemble_crew::<i32>(1, f).await;
        assert_eq!(val.is_err(), true);
    }
}

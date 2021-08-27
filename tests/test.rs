use revolt_rs;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("revolt.rs error")]
    RevoltRs {
        #[from]
        source: revolt_rs::RevoltRsError,
    },
    #[error("std::env::VarError")]
    Var {
        #[from]
        source: std::env::VarError,
    },
}

#[test]
fn create_client() -> Result<(), MyError> {
    let token = std::env::var("REVOLT_TOKEN");
    let client = revolt_rs::RevoltRs::new(&token?)?;
    assert_eq!(
        client,
        revolt_rs::RevoltRs {
            client: client.client,
        }
    );
    Ok(())
}

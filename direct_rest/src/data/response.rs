use iface::transfer::Transfer;

pub enum TransferResponse<T>
    where
        T: Transfer,
{
    Success(T),
    Failure(String),
}

use iface::transfer::Transfer;

pub enum Response<T>
where
    T: Transfer,
{
    Success(T),
    Failure(String),
}

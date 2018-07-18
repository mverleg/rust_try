use iface::transfer::Transfer;

pub struct TransferRequest<T>
    where
        T: Transfer,
{
    val: T,
}

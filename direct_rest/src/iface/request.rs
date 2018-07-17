use iface::transfer::Transfer;

pub struct Request<T>
where
    T: Transfer,
{
    val: T,
}

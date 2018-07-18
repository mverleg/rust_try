pub trait Endpoint {
    fn api_query(req: TransferRequest<Example>) -> TransferResponse<Example> {
        unimplemented!();
    }
}

use std::{future::Future, io, pin::Pin, sync::Arc};
use viadkim::verifier::LookupTxt;


type LookupOutput = Vec<io::Result<Vec<u8>>>;
type LookupFuture<'a> = Pin<Box<dyn Future<Output = io::Result<LookupOutput>> + Send + 'a>>;
#[derive(Clone)]
pub struct Resolver(Arc<dyn Fn(&str) -> LookupFuture<'_> + Send + Sync>);

impl Resolver {
    pub fn new(f: impl Fn(&str) -> LookupFuture<'_> + Send + Sync + 'static) -> Self {
        Self(Arc::new(f))
    }
}

impl LookupTxt for Resolver {
    type Answer = LookupOutput;
    type Query<'a> = Pin<Box<dyn Future<Output = io::Result<Self::Answer>> + Send + 'a>>;

    fn lookup_txt(&self, domain: &str) -> Self::Query<'_> {
        let domain = domain.to_owned();
       
        Box::pin(async move { self.0(&domain).await })
    }
}

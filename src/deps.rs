use hyper::{Body};
use juniper::{GraphQLType, RootNode, ScalarRefValue, ScalarValue};

use std::sync::Arc;
use std::marker::PhantomData;

use hyper::rt::Future;
use futures::future;
use futures::Async;

pub fn graphql<CtxT, QueryT, MutationT, S>(
    root_node: Arc<RootNode<'static, QueryT, MutationT, S>>,
) -> impl Future<Item = Body, Error = ()>
where
    S: ScalarValue + Send + Sync + 'static,
    for<'b> &'b S: ScalarRefValue<'b>,
    CtxT: Send + Sync + 'static,
    QueryT: GraphQLType<S, Context = CtxT> + Send + Sync + 'static,
    MutationT: GraphQLType<S, Context = CtxT> + Send + Sync + 'static,
    QueryT::TypeInfo: Send + Sync,
    MutationT::TypeInfo: Send + Sync,
{
    let request = create_request();
    request.execute(root_node)
}

fn create_request<S: ScalarValue>() -> GraphQLRequest<S> {
    unimplemented!()
}

struct GraphQLRequest<S: ScalarValue>(
    PhantomData<S>
);

impl<S> GraphQLRequest<S>
where
    S: ScalarValue,
    for<'b> &'b S: ScalarRefValue<'b>,
{
    fn execute<'a, CtxT: 'a, QueryT, MutationT>(
        self,
        root_node: Arc<RootNode<'a, QueryT, MutationT, S>>,
    ) -> impl Future<Item = hyper::Body, Error = ()> + 'a
    where
        S: 'a,
        QueryT: GraphQLType<S, Context = CtxT> + 'a,
        MutationT: GraphQLType<S, Context = CtxT> + 'a,
    {
        let requests: Vec<JuniperGraphQLRequest<S>> = vec![];
        future::join_all(requests.into_iter().map(move |request| {
            future::poll_fn(move || {
                let _res = request.execute(&root_node);
                Ok(Async::Ready(()))
            })
        }))
        .map(|_| {
            unimplemented!()
        })
    }
}

struct JuniperGraphQLRequest<S: ScalarValue>(
    PhantomData<S>
);

impl<S> JuniperGraphQLRequest<S>
where
    S: ScalarValue,
{
    fn execute<'a, CtxT, QueryT, MutationT>(
            &'a self,
            _root_node: &'a RootNode<QueryT, MutationT, S>,
        )
        where
            S: ScalarValue,
            QueryT: GraphQLType<S, Context = CtxT>,
            MutationT: GraphQLType<S, Context = CtxT>,
            for<'b> &'b S: ScalarRefValue<'b>,
        {
            unimplemented!()
        }
}



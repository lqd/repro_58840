use hyper::rt::Future;
use hyper::service::service_fn;

use juniper::FieldResult;
use std::sync::Arc;

mod deps;

struct Query;

juniper::graphql_object!(Query: () |&self| {
    field favoriteEpisode(&executor) -> FieldResult<()> {
        Ok(())
    }
});

fn f() -> Arc<juniper::RootNode<'static, Query, juniper::EmptyMutation<()>>> {
    unimplemented!()
}

fn main() {
    let root_node = f();
    move || {
        let root_node = root_node.clone();
        service_fn(move |_: hyper::Request<hyper::Body>| -> Box<Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            Box::new(deps::graphql(root_node))
        })
    };
}

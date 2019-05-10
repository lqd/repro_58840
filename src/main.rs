extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate hyper;

extern crate juniper;
extern crate juniper_hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate uuid;

use dotenv::dotenv;
use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Response, Server, StatusCode};
use std::sync::Arc;

use juniper::{FieldResult, EmptyMutation};

#[derive(juniper::GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

struct Query;

juniper::graphql_object!(Query: Ctx |&self| {
    field favoriteEpisode(&executor) -> FieldResult<Episode> {
        // Use the special &executor argument to fetch our fav episode.
        Ok(executor.context().0)
    }
});

// Arbitrary context data.
struct Ctx(Episode);

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>>;

fn main() {
    // Load configuration
    dotenv().ok();

    // Prepare logger
    pretty_env_logger::init();

    info!("Starting Cardioid API");

    // Prepare HOST configuration
    let host = std::env::var("HOST").expect("Missing HOST definition in env variables");
    let address = host.parse().expect("Invalid HOST configuration");

    // Prepare context and schema
    let context = Arc::new(Ctx(Episode::NewHope));
    let root_node = Arc::new(Schema::new(Query {}, EmptyMutation::new()));

    // Define network service
    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = context.clone();
        service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            match (req.method(), req.uri().path()) {
                // GraphiQL
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),

                // GraphQL
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)), //
                (&Method::POST, "/graphql") => {
                    Box::new(juniper_hyper::graphql(root_node, ctx, req)) //
                }

                // Default response
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };

    // Create server with service
    let server = Server::bind(&address)
        .serve(new_service)
        .map_err(|e| error!("Server error: {}", e));

    info!("Listening on http://{}", address);

    // Run server
    hyper::rt::run(server);
}

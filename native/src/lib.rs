use neon::register_module;
use neon::prelude::*;
use thruster::hyper_server::HyperServer;
use thruster::thruster_context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::thruster_proc::{async_middleware, middleware_fn};
use thruster::{App, ThrusterServer};
use thruster::{MiddlewareNext, MiddlewareReturnValue, MiddlewareResult};

#[middleware_fn]
async fn fib(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let n = context.params.get("n").unwrap().parse::<u128>().unwrap();

    context.body(&format!("{{\"n\":{}}}", fibonacci(n)));
    Ok(context)
}

fn fibonacci(number: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..number {
        let tmp = a;
        a = b;
        b = a + tmp;
    }

    b
}

fn start(mut cx: FunctionContext) -> JsResult<JsNull> {
    let port = cx.argument::<JsNumber>(0)?.value();

    let mut app = App::<HyperRequest, Ctx>::create(generate_context);
    app.get("/v2/fib/:n", async_middleware!(Ctx, [fib]));

    let server = HyperServer::new(app);
    server.start("0.0.0.0", port as u16);

    Ok(cx.null())
}

register_module!(mut cx, {
    let _ = cx.export_function("start", start);

    Ok(())
});

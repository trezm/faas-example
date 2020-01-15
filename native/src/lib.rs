use neon::register_module;
use neon::prelude::*;
use thruster::thruster_context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx,
};
use thruster::{MiddlewareNext, MiddlewareReturnValue, MiddlewareResult};
use thruster::thruster_proc::{middleware_fn};
use thruster_x::{hyper_express as express};

#[express(generate_context)]
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

register_module!(mut cx, {
    let _ = cx.export_function("fib", express_fib);

    Ok(())
});

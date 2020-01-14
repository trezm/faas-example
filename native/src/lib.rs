use neon::register_module;
use neon::prelude::*;


fn fib(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value();

    Ok(cx.number(fibonacci(n)))
}

fn fibonacci(number: u32) -> u32 {
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
    let _ = cx.export_function("fib", fib);

    Ok(())
});

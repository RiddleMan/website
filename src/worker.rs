use worker::*;

#[event(fetch)]
async fn main(_: Request, _: Env, _: Context) -> Result<Response> {
    Response::ok("Hello, dupa!")
}

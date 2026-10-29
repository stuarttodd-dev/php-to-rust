use gloo_net::http::Request;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const API: &str = "http://127.0.0.1:8090";

#[derive(Clone, Deserialize)]
struct Product {
    title: String,
    price_cents: u32,
}

#[derive(Deserialize)]
struct CreatedBasket {
    id: i32,
}

#[derive(Deserialize)]
struct Cents {
    total_cents: u32,
}

#[derive(Serialize)]
struct DiscountBody {
    code: String,
}

async fn load_catalog() -> Result<(Vec<(String, u32)>, i32), String> {
    let products: Vec<Product> = Request::get(&format!("{API}/products"))
        .send()
        .await
        .map_err(|err| err.to_string())?
        .json()
        .await
        .map_err(|err| err.to_string())?;
    let created: CreatedBasket = Request::post(&format!("{API}/baskets"))
        .send()
        .await
        .map_err(|err| err.to_string())?
        .json()
        .await
        .map_err(|err| err.to_string())?;
    Ok((
        products
            .into_iter()
            .map(|item| (item.title, item.price_cents))
            .collect(),
        created.id,
    ))
}

#[component]
fn Shop() -> impl IntoView {
    let catalog = LocalResource::new(load_catalog);
    let basket_id = RwSignal::new(0);
    let qty = RwSignal::new(0u32);
    let added = RwSignal::new(String::new());
    let priced = RwSignal::new(String::new());
    let discounted = RwSignal::new(String::new());
    let paid = RwSignal::new(String::new());
    let page_error = RwSignal::new(String::new());
    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || Suspend::new(async move {
                match catalog.await {
                    Ok((titles, id)) => {
                        basket_id.set(id);
                        view! {
                            <ul>
                                {titles
                                    .into_iter()
                                    .map(|(title, price_cents)| view! { <li>{title}" "{price_cents}</li> })
                                    .collect_view()}
                            </ul>
                            <button on:click=move |_| {
                                let id = basket_id.get();
                                leptos::task::spawn_local(async move {
                                    match shop_front::add_mug(id).await {
                                        Ok(()) => {
                                            qty.update(|count| *count += 1);
                                            let total = shop_front::mug_total("", qty.get())
                                                .map(|cents| cents.to_string())
                                                .unwrap_or_else(|| "unknown".into());
                                            added.set(format!("added {total}"));
                                        }
                                        Err(message) => page_error.set(message),
                                    }
                                });
                            }>"Add mug"</button>
                            <button on:click=move |_| {
                                let id = basket_id.get();
                                let mugs = qty.get();
                                leptos::task::spawn_local(async move {
                                    let wasm_total = shop_front::mug_total("SAVE10", mugs)
                                        .map(|cents| cents.to_string())
                                        .unwrap_or_else(|| "unknown".into());
                                    match Request::post(&format!("{API}/baskets/{id}/discount"))
                                        .json(&DiscountBody {
                                            code: "SAVE10".into(),
                                        })
                                    {
                                        Ok(request) => match request.send().await {
                                            Ok(response) => match response.json::<Cents>().await {
                                                Ok(body) => {
                                                    priced.set(format!("wasm {wasm_total}"));
                                                    discounted.set(format!("discount {}", body.total_cents));
                                                }
                                                Err(err) => page_error.set(err.to_string()),
                                            },
                                            Err(err) => page_error.set(err.to_string()),
                                        },
                                        Err(err) => page_error.set(err.to_string()),
                                    }
                                });
                            }>"SAVE10"</button>
                            <button on:click=move |_| {
                                let id = basket_id.get();
                                leptos::task::spawn_local(async move {
                                    match Request::post(&format!("{API}/baskets/{id}/checkout"))
                                        .send()
                                        .await
                                    {
                                        Ok(response) => match response.json::<Cents>().await {
                                            Ok(body) => paid.set(format!("checkout {}", body.total_cents)),
                                            Err(err) => page_error.set(err.to_string()),
                                        },
                                        Err(err) => page_error.set(err.to_string()),
                                    }
                                });
                            }>"Checkout"</button>
                            <p>{added}</p>
                            <p>{priced}</p>
                            <p>{discounted}</p>
                            <p>{paid}</p>
                            <p>{page_error}</p>
                        }.into_any()
                    }
                    Err(message) => view! { <p>{message}</p> }.into_any(),
                }
            })}
        </Suspense>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <Shop/> })
}

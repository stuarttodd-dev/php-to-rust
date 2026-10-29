use dioxus::prelude::*;
use dioxus_router::{Link, Outlet, Routable, Router};
use gloo_storage::{LocalStorage, Storage};

const SHOP_CSS: Asset = asset!("/assets/shop.css");

/// Base URL of your Axum shop API (no trailing slash).
/// Course examples use port 8090; match your running server.
const API_BASE: &str = "http://127.0.0.1:8090";

#[derive(Clone, Debug, serde::Deserialize)]
struct ProductRow {
    id: i64,
    title: String,
    price_cents: i64,
}

/// Matches `GET /api/baskets/{basket_id}` JSON from the Chapter 15 shop API shape.
#[derive(Clone, Debug, serde::Deserialize)]
struct BasketLineDto {
    product_id: u64,
    title: String,
    quantity: u32,
    line_subtotal_cents: u64,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct BasketDto {
    lines: Vec<BasketLineDto>,
    subtotal_cents: u64,
}

fn format_gbp(cents: i64) -> String {
    let pounds = cents / 100;
    let pence = cents.rem_euclid(100);
    format!("£{pounds}.{pence:02}")
}

async fn fetch_products() -> Result<Vec<ProductRow>, String> {
    let url = format!("{API_BASE}/api/products");
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

async fn fetch_product(id: i64) -> Result<ProductRow, String> {
    let url = format!("{API_BASE}/api/products/{id}");
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 404 {
        return Err("not found".into());
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

async fn fetch_basket(basket_id: &str) -> Result<BasketDto, String> {
    let url = format!("{API_BASE}/api/baskets/{basket_id}");
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 404 {
        return Err("not found".into());
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

const BASKET_KEY: &str = "shop_basket_id";

fn ensure_basket_id() -> String {
    if let Ok(id) = LocalStorage::get::<String>(BASKET_KEY) {
        if !id.trim().is_empty() {
            return id;
        }
    }
    let id = "demo".to_string();
    let _ = LocalStorage::set(BASKET_KEY, &id);
    id
}

#[derive(serde::Serialize)]
struct AddLine {
    product_id: i64,
    qty: u32,
}

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(ShopShell)]
    #[route("/")]
    Home {},
    #[route("/product/:id")]
    ProductDetail { id: i64 },
    #[route("/basket")]
    Basket {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn ShopShell() -> Element {
    rsx! {
        document::Stylesheet { href: SHOP_CSS }
        header { class: "site-header",
            div { class: "site-header__inner",
                Link { to: Route::Home {}, class: "site-logo", "Rust shop" }
                Link { to: Route::Basket {}, class: "site-header__basket", "Basket" }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let filter = use_signal(String::new);
    rsx! {
        CatalogHome { filter }
    }
}

#[component]
fn CatalogHome(mut filter: Signal<String>) -> Element {
    let catalog = use_resource(|| async move { fetch_products().await });

    rsx! {
        main { class: "catalog-main",
            div { class: "site-search",
                label { r#for: "catalog-search", "Search" }
                input {
                    id: "catalog-search",
                    r#type: "search",
                    placeholder: "Filter by title",
                    value: "{filter}",
                    oninput: move |e| *filter.write() = e.value(),
                }
            }
            h1 { "Products" }
            match catalog.read().as_ref() {
                None => rsx! {
                    p { class: "catalog-status", "Loading catalog…" }
                },
                Some(Err(e)) => rsx! {
                    p { class: "catalog-status catalog-status--error",
                        "Could not load products: {e}"
                    }
                },
                Some(Ok(products)) => {
                    let needle = filter.read().to_lowercase();
                    let rows: Vec<_> = products
                        .iter()
                        .filter(|p| {
                            needle.is_empty()
                                || p.title.to_lowercase().contains(&needle)
                        })
                        .collect();

                    if rows.is_empty() {
                        rsx! {
                            p { class: "catalog-status",
                                if needle.is_empty() {
                                    "No products returned."
                                } else {
                                    "No products match your search."
                                }
                            }
                        }
                    } else {
                        rsx! {
                            ul { class: "product-grid",
                                for p in rows {
                                    li {
                                        key: "{p.id}",
                                        class: "product-card",
                                        h2 {
                                            Link {
                                                to: Route::ProductDetail { id: p.id },
                                                class: "product-card__link",
                                                "{p.title}"
                                            }
                                        }
                                        p { class: "product-card__price",
                                            "{format_gbp(p.price_cents)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductDetail(id: i64) -> Element {
    let pid = id;
    let detail = use_resource(move || async move { fetch_product(pid).await });

    rsx! {
        main { class: "catalog-main product-detail",
            Link { to: Route::Home {}, class: "back-link", "← Back to catalog" }
            match detail.read().as_ref() {
                None => rsx! { p { "Loading…" } },
                Some(Err(e)) if e == "not found" => rsx! {
                    p { class: "catalog-status--error", "Product not found." }
                },
                Some(Err(e)) => rsx! {
                    p { class: "catalog-status--error", "Error: {e}" }
                },
                Some(Ok(p)) => rsx! {
                    article { class: "product-detail__card",
                        h1 { "{p.title}" }
                        p { class: "product-card__price", "{format_gbp(p.price_cents)}" }
                        AddToBasket { product_id: p.id }
                    }
                },
            }
        }
    }
}

#[component]
fn AddToBasket(product_id: i64) -> Element {
    let mut message = use_signal(|| None::<String>);
    let pid = product_id;

    let on_add = move |_| async move {
        let basket_id = ensure_basket_id();
        let url = format!("{API_BASE}/api/baskets/{basket_id}/items");
        let body = AddLine { product_id: pid, qty: 1 };
        let json = serde_json::to_string(&body).unwrap_or_default();
        let req = match gloo_net::http::Request::post(&url)
            .header("Content-Type", "application/json")
            .body(json)
        {
            Ok(req) => req,
            Err(e) => {
                message.set(Some(e.to_string()));
                return;
            }
        };
        match req.send().await {
            Ok(resp) if resp.ok() || resp.status() == 204 => {
                message.set(Some("Added to basket.".into()));
            }
            Ok(resp) => {
                message.set(Some(format!("Basket error: HTTP {}", resp.status())));
            }
            Err(e) => message.set(Some(e.to_string())),
        }
    };

    rsx! {
        div { class: "add-to-basket",
            button { r#type: "button", onclick: on_add, "Add to basket" }
            if let Some(m) = message.read().as_ref() {
                p { class: "catalog-status", "{m}" }
            }
        }
    }
}

#[component]
fn Basket() -> Element {
    let basket_id = ensure_basket_id();
    let loaded = use_resource(move || {
        let id = basket_id.clone();
        async move { fetch_basket(&id).await }
    });
    let mut checkout_note = use_signal(|| None::<String>);

    rsx! {
        main { class: "catalog-main basket-page",
            Link { to: Route::Home {}, class: "back-link", "← Back to catalog" }
            h1 { "Basket" }
            match loaded.read().as_ref() {
                None => rsx! {
                    p { class: "catalog-status", "Loading basket…" }
                },
                Some(Err(e)) if e == "not found" => rsx! {
                    p { class: "catalog-status", "Basket not found." }
                },
                Some(Err(e)) => rsx! {
                    p { class: "catalog-status catalog-status--error", "Error: {e}" }
                },
                Some(Ok(b)) if b.lines.is_empty() => rsx! {
                    p { class: "catalog-status", "Your basket is empty." }
                    Link { to: Route::Home {}, class: "back-link", "Continue shopping" }
                },
                Some(Ok(b)) => {
                    let lines = b.lines.clone();
                    let subtotal = b.subtotal_cents as i64;
                    rsx! {
                        section { class: "basket-lines",
                            ul {
                                for line in lines {
                                    li {
                                        key: "{line.product_id}",
                                        class: "basket-line",
                                        span { class: "basket-line__title", "{line.title}" }
                                        " × "
                                        span { "{line.quantity}" }
                                        " — "
                                        span { class: "basket-line__amount",
                                            "{format_gbp(line.line_subtotal_cents as i64)}"
                                        }
                                    }
                                }
                            }
                            p { class: "basket-subtotal",
                                "Subtotal: "
                                "{format_gbp(subtotal)}"
                            }
                            button {
                                r#type: "button",
                                class: "checkout-btn",
                                onclick: move |_| {
                                    checkout_note.set(Some(
                                        "Stub checkout — add a real order route in Axum when you extend the API."
                                            .into(),
                                    ));
                                },
                                "Checkout"
                            }
                            if let Some(note) = checkout_note.read().as_ref() {
                                p { class: "catalog-status", "{note}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

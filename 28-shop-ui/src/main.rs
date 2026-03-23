use dioxus::prelude::*;

const SHOP_CSS: Asset = asset!("/assets/shop.css");

/// Base URL of your Axum shop API (no trailing slash).
/// Course examples use port 8090; the GitHub `27-shop-api` README may use another port (e.g. 8080). Match your running server.
const API_BASE: &str = "http://127.0.0.1:8090";

#[derive(Clone, Debug, serde::Deserialize)]
struct Product {
    id: i64,
    title: String,
    price_cents: i64,
}

fn format_gbp(cents: i64) -> String {
    let pounds = cents / 100;
    let pence = cents.rem_euclid(100);
    format!("£{pounds}.{pence:02}")
}

async fn fetch_products() -> Result<Vec<Product>, String> {
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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let filter = use_signal(String::new);

    rsx! {
        document::Stylesheet { href: SHOP_CSS }
        ShopHeader { filter }
        CatalogHome { filter }
    }
}

#[component]
fn ShopHeader(mut filter: Signal<String>) -> Element {
    rsx! {
        header { class: "site-header",
            div { class: "site-header__inner",
                a { class: "site-logo", href: "/", "Rust shop" }
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
            }
        }
    }
}

#[component]
fn CatalogHome(filter: Signal<String>) -> Element {
    let catalog = use_resource(|| async move { fetch_products().await });

    rsx! {
        main { class: "catalog-main",
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
                                        h2 { "{p.title}" }
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

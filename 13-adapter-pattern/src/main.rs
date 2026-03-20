trait PaymentGateway {
    fn charge_cents(&self, amount_cents: u64, currency: &str);
}

struct ForeignPaySdk;

impl ForeignPaySdk {
    fn submit_charge(&self, amount: &str, currency_code: u16) {
        println!("ForeignPay: {} (code {})", amount, currency_code);
    }
}

fn iso_4217_numeric(currency: &str) -> u16 {
    match currency {
        "USD" => 840,
        "EUR" => 978,
        _ => 0,
    }
}

struct ForeignPayAdapter {
    inner: ForeignPaySdk,
}

impl PaymentGateway for ForeignPayAdapter {
    fn charge_cents(&self, amount_cents: u64, currency: &str) {
        let major = amount_cents as f64 / 100.0;
        let amount = format!("{:.2}", major);
        let code = iso_4217_numeric(currency);
        self.inner.submit_charge(&amount, code);
    }
}

fn checkout(gateway: &dyn PaymentGateway, amount_cents: u64, currency: &str) {
    gateway.charge_cents(amount_cents, currency);
}

fn main() {
    let gw = ForeignPayAdapter {
        inner: ForeignPaySdk,
    };
    checkout(&gw, 1999, "USD");
}

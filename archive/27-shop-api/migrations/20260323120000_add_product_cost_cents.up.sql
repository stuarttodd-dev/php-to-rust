-- Internal cost for margin math; never exposed on public JSON (see ProductResponse).
ALTER TABLE products
    ADD COLUMN cost_cents INTEGER NOT NULL DEFAULT 0 CHECK (cost_cents >= 0);

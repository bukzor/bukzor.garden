pub fn pick_random<T>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }
    let idx = (js_sys::Math::random() * items.len() as f64) as usize;
    items.get(idx.min(items.len() - 1))
}

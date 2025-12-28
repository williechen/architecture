use crate::chapter1::{self, Batch};

pub fn is_valid_sku(sku: &str, batches: &Vec<&mut Batch>) -> bool {
    batches.iter().any(|b| b.sku == sku)
}

pub fn allocate(
    lien: &chapter1::OrderLine,
    batches: Vec<&mut Batch>,
) -> Result<Option<String>, String> {
    if !is_valid_sku(&lien.sku, &batches) {
        return Err(format!("Invalid sku {}", lien.sku));
    }

    let res = chapter1::allocate(lien, batches)?;

    Ok(res)
}

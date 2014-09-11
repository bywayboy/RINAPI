use super::super::prelude::{
	GdiObject , StockLogicalObject , wapi
};

pub fn GetStockObject(stock : StockLogicalObject) -> GdiObject {
    unsafe {
        wapi::DC::GetStockObject(stock)
    }
}
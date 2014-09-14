use super::super::prelude::{
	GdiObject , StockLogicalObject , wapi
};

pub fn GetStockObject(stock : StockLogicalObject) -> GdiObject {
    unsafe {
        wapi::DeviceContext::GetStockObject(stock)
    }
}
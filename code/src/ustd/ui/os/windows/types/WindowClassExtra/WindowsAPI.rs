extern "stdcall" {
	fn RegisterClassEx(
		/*   _In_   */ lpwcx : *const WNDCLASSEX
	) -> ATOM /* WINAPI */;
}
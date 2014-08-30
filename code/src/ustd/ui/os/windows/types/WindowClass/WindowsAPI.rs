extern "stdcall" {
	fn RegisterClass(
		/*   _In_   */ lpWndClass : *const WNDCLASS
	) -> ATOM /* WINAPI */;
}
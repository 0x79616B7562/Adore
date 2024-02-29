macro_rules! ctx {
	( ) => {
		unsafe {
			use crate::gfx::raw::context::CONTEXT;
			CONTEXT.as_mut().expect("No context created.")
		}
	};
}

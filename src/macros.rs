macro_rules! ctx {
	( ) => {
		unsafe {
			use crate::gfx::raw::context::CONTEXT;
			CONTEXT.as_mut().expect("No context created.")
		}
	};
}

macro_rules! include_bytes_from_root {
    ( $x:expr ) => {
        {
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $x))
        }
    };
}

macro_rules! include_str_from_root {
    ( $x:expr ) => {
        {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $x))
        }
    };
}

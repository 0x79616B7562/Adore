include!("macros.rs");

pub use glam;
pub use log;
pub use wrld::Desc;

//

mod adore;
mod api;
mod assets;
mod gfx;
pub mod logger;
mod math;
mod time;
mod traits;
mod types;
mod window;

pub use adore::*;
pub use api::*;
pub use assets::*;
pub use gfx::*;
pub use math::*;
pub use time::*;
pub use traits::*;
pub use types::*;
pub use window::{
    KeyCode,
    MouseButton,
    WindowConfig,
};

//     ,'``.._   ,'``.
//     :,--._:)\,:,._,.:       All Glory to
//     :`--,''   :`...';\      the HYPNO TOAD!
//      `,'       `---'  `.
//      /                 :
//     /                   \
//   ,'                     :\.___,-.
//  `...,---'``````-..._    |:       \
//    (                 )   ;:    )   \  _,-.
//     `.              (   //          `'    \
//      :               `.//  )      )     , ;
//    ,-|`.            _,'/       )    ) ,' ,'
//   (  :`.`-..____..=:.-':     .     _,' ,'
//    `,'\ ``--....-)='    `._,  \  ,') _ '``._
// _.-/ _ `.       (_)      /     )' ; / \ \`-.'
//`--(   `-:`.     `' ___..'  _,-'   |/   `.)
//    `-. `.`.``-----``--,  .'
//      |/`.\`'        ,',');
//          `         (/  (/

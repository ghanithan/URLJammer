use common::ArcStr;
use paste::paste;
use serde::Deserialize;

use crate::{Storage, StorageSetting};

pub trait HandleStorage {
    fn new(storage_setting: StorageSetting) -> Self;
    fn meta(&self) -> String;
    fn create_path(&self, slug: String) -> Result<String, anyhow::Error>;
    fn create_asset(&self, dir_path: String, content: String) -> Result<(), anyhow::Error>;
}

#[macro_export]
macro_rules! create_enum_and_type {

    (
       $(#[$meta:meta])*
       $vis:vis enum $struct_name:ident {
          $(
          $(#[$variant_meta:meta])*
          $variant_vis:vis $variant_name:ident( $variant_type:ty)
          ),*$(,)+
      }
      ) => {
            $(#[$meta])*
            $vis enum $struct_name{
                $(
                $(#[$variant_meta:meta])*
                    $variant_vis $variant_name( $variant_type),
                )*
            }
            paste::paste!{
              $(#[$meta])*
              #[derive(Deserialize, Clone)]
              $vis enum [<$struct_name Type>]{
                  $(
                  $(#[$variant_meta:meta])*
                      $variant_vis $variant_name,
                  )*
              }


            }
      }

}

use fiors::{materials::MaterialCategory, FIOClient};

use quote::{format_ident, quote};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FIOClient::new_from_env()?;
    client.local_cache_dir = Some(".fio_cache".into());

    let all_materials = client.get_all_materials().await?;

    let mut pre = quote! {
        let mut map = HashMap::new();
    };

    let mut all_materials: Vec<_> = all_materials.into_iter().collect();
    all_materials.sort_by(|a, b| a.0.cmp(&b.0));

    for (_ticker, mat) in all_materials.iter() {
        let material_id = &mat.material_id;
        let category_name = &mat.category_name;
        let category_id = &mat.category_id;
        let name = &mat.name;
        let ticker = &mat.ticker;
        let weight = mat.weight;
        let volume = mat.volume;

        let category = format_ident!(
            "{}",
            format!("{:?}", MaterialCategory::from_name(category_name).unwrap())
        );
        pre.extend(quote! {
            map.insert(#ticker, StaticMaterialInfo {
                material_id: #material_id,
                category_name: #category_name,
                category_id: #category_id,
                name: #name,
                ticker: #ticker,
                weight: #weight,
                volume: #volume,
                category: MaterialCategory :: #category,
            });
        });
    }

    let f = quote! {
        use std::collections::HashMap;
        use crate::materials::MaterialCategory;
        use once_cell::sync::OnceCell;


        #[derive(Debug, Copy, Clone)]
        pub struct StaticMaterialInfo {
            pub material_id: &'static str,
            pub category_name: &'static str,
            pub category_id: &'static str,
            pub name: &'static str,
            pub ticker: &'static str,
            pub weight: f32,
            pub volume: f32,
            pub category: MaterialCategory,
        }

        static DB: once_cell::sync::OnceCell<HashMap<&'static str, StaticMaterialInfo>> = OnceCell::new();

        pub fn get_material_db() -> &'static HashMap<&'static str, StaticMaterialInfo> {
            DB.get_or_init(|| {
                construct_material_db()
            })
        }


        pub fn construct_material_db() -> HashMap<&'static str, StaticMaterialInfo> {
            #pre

            map
        }
    };

    match syn::parse_file(&f.to_string()) {
        Ok(file) => {
            let formatted = prettyplease::unparse(&file);
            // println!("{}", formatted);
            std::fs::write("src/material_db.rs", formatted)?;
        }
        Err(e) => {
            println!("Failed to parse: {e}");
            println!("{f}");
        }
    }

    // let file = syn::parse_file(&f.to_string())?;
    // let formatted = prettyplease::unparse(&file);
    // println!("{formatted}");

    Ok(())
}

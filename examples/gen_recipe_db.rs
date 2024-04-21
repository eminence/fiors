use anyhow::Context;
use fiors::FIOClient;

use quote::quote;

#[cfg(feature = "gendb")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FIOClient::new_with_key("9dd5160d-acc8-493d-b222-d5f96273f677".into());
    client.local_cache_dir = Some(".fio_cache".into());

    let mut all_recipes: Vec<_> = client
        .get_all_recipes()
        .await
        .context("Failed to get recipe list from API")?;

    all_recipes.sort_by(|a, b| a.standard_recipe_name.cmp(&b.standard_recipe_name));

    let f = quote! {
        use once_cell::sync::OnceCell;

        #[derive(Debug, Copy, Clone)]
        pub struct StaticRecipeMaterial {
            pub ticker: &'static str,
            pub amount: u32,
        }

        #[derive(Debug, Copy, Clone)]
        pub struct StaticRecipeInfo {
            pub building_ticker: &'static str,
            pub recipe_name: &'static str,
            pub standard_recipe_name: &'static str,
            pub duration: std::time::Duration,
            pub inputs: &'static [StaticRecipeMaterial],
            pub outputs: &'static [StaticRecipeMaterial],
        }


        static DB: once_cell::sync::OnceCell<Vec<StaticRecipeInfo>> = OnceCell::new();

        pub fn get_recipe_db() -> &'static [StaticRecipeInfo] {
            DB.get_or_init(construct_recipe_db)
        }

        pub fn construct_recipe_db() -> Vec<StaticRecipeInfo> {
           vec![
               #(#all_recipes),*
           ]
        }
    };

    match syn::parse_file(&f.to_string()) {
        Ok(file) => {
            let formatted = prettyplease::unparse(&file);
            // println!("{}", formatted);
            std::fs::write("src/recipe_db.rs", formatted)?;
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

#[cfg(not(feature = "gendb"))]
fn main() {
    println!("This example requires the `gendb` feature to be enabled");
}

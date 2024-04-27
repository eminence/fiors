use fiors::FIOClient;

use quote::quote;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FIOClient::new_from_env()?;
    client.local_cache_dir = Some(".fio_cache".into());

    let all_buildings = client.get_all_buildings().await?;

    let mut pre = quote! {
        let mut map = HashMap::new();
    };

    let mut all_buildings: Vec<_> = all_buildings.into_iter().collect();
    all_buildings.sort_by(|a, b| a.0.cmp(&b.0));

    for (_ticker, building) in all_buildings {
        let name = &building.name;
        let ticker = &building.ticker;
        let expertise = if let Some(exp) = building.expertise {
            quote! { Some(#exp) }
        } else {
            quote! { None }
        };
        let pioneers = building.pioneers;
        let settlers = building.settlers;
        let technicians = building.technicians;
        let engineers = building.engineers;
        let scientists = building.scientists;
        let area_cost = building.area_cost;
        let building_cost_ticker = building
            .building_costs
            .iter()
            .map(|bc| bc.commodity_ticker.as_str());
        let building_cost_amount = building.building_costs.iter().map(|bc| bc.amount);

        pre.extend(quote! {
            let x = StaticBuildingInfo {
                name: #name,
                ticker: #ticker,
                expertise: #expertise,
                pioneers: #pioneers,
                settlers: #settlers,
                technicians: #technicians,
                engineers: #engineers,
                scientists: #scientists,
                area_cost: #area_cost,
                building_cost: &[ #( (#building_cost_ticker, #building_cost_amount) ),* ]
            };
            map.insert(#ticker, x);
            map.insert(#name, x);
        });

        // break;
    }

    let f = quote! {
        use std::collections::HashMap;
        use crate::materials::MaterialCategory;
        use once_cell::sync::OnceCell;


        #[derive(Debug, Copy, Clone)]
        pub struct StaticBuildingInfo {
            pub name: &'static str,
            pub ticker: &'static str,
            pub expertise: Option<&'static str>,
            pub pioneers: u32,
            pub settlers: u32,
            pub technicians: u32,
            pub engineers: u32,
            pub scientists: u32,
            pub area_cost: u32,
            pub building_cost: &'static [(&'static str, u32)]
        }

        static DB: once_cell::sync::OnceCell<HashMap<&'static str, StaticBuildingInfo>> = OnceCell::new();

        pub fn get_building_db() -> &'static HashMap<&'static str, StaticBuildingInfo> {
            DB.get_or_init(construct_building_db)
        }


        pub fn construct_building_db() -> HashMap<&'static str, StaticBuildingInfo> {
            #pre

            map
        }
    };

    match syn::parse_file(&f.to_string()) {
        Ok(file) => {
            let formatted = prettyplease::unparse(&file);
            // println!("{}", formatted);
            std::fs::write("src/building_db.rs", formatted)?;
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

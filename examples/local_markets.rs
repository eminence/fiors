use std::collections::HashMap;

use anyhow::Context;
use fiors::{
    get_material_db,
    materials::{MaterialCategory, MaterialWithColor},
    FIOClient,
};

fn read_from_stdin() -> bool {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    buf.trim() == "y"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FIOClient::new_with_key("9dd5160d-acc8-493d-b222-d5f96273f677".into());
    client.local_cache_dir = Some(".fio_cache".into());

    let username = client.is_auth().await?;

    // let all_materials = client.get_all_materials().await?;
    // create a map from material name to category
    // let material_categories: HashMap<String, MaterialCategory> = all_materials
    //     .iter()
    //     .map(|(name, mat)| (name.clone(), MaterialCategory::from_name(&mat.category_name).unwrap_or_else(|| panic!("unknown category {}", mat.category_name))))
    //     .collect();

    let planets = client.get_storage_planets_for_user(&username).await?;

    for planet in planets {
        println!("==> {}", planet.name);
        if !read_from_stdin() {
            continue;
        }

        if !planet.has_local_market {
            continue;
        }
        let mut printed_lm_header = false;

        let lm = client.get_planet_localmarket(&planet.id).await?;

        // keep track of our own selling orders, we'll need this later
        let mut our_selling_orders = Vec::new();

        for ad in lm.selling_ads {
            let price_per_unit = ad.total_price / ad.material_amount as f32;

            // find the current benten station market price
            let cx = client
                .get_exchange_info(&format!("{}.CI1", ad.material_ticker))
                .await?;

            let instant = cx.instant_sell(ad.material_amount);

            // let buy_diff = 100.0 * price_per_unit / cx.ask;
            let cx_ask = cx
                .ask
                .map(|b| format!("{:.2}", 100.0 * price_per_unit / b))
                .unwrap_or("N/A".into());

            if ad.creator_company_code == "EM32" {
                our_selling_orders.push(ad.material_ticker.clone());
                continue;
            }
            if !printed_lm_header {
                println!("  --- Local Market:");
                printed_lm_header = true;
            }

            let colored_ticket =
                MaterialWithColor::new(&ad.material_ticker).with_amount(ad.material_amount as i32);
            println!(
                "  {} is selling {colored_ticket} for {} ({:.1} {} per unit / {cx_ask}% / {:.1}% of CX) ",
                ad.creator_company_name,
                // ad.material_amount,
                // ad.material_ticker,
                ad.total_price,
                price_per_unit,
                ad.currency,
                100.0 * price_per_unit / cx.price
            );

            // if we buy this local market ad, can we instantly sell it on the CX for a profit?
            if let Some(instant) = instant {
                if ad.total_price < instant.total_value {
                    println!(
                        "    !!! This is lower than CX offers prices ({:?} on the CX) -- good deal",
                        instant.total_value
                    );
                }
            }

            // if we need this material, can we buy it cheaper on the CX?
            // if matches!(cx.bid, Some(cx_bid) if price_per_unit < cx_bid) {
            //     println!("    !!! This is cheaper than CX buy prices ({:?} per unit on the CX) -- good deal", cx.bid);
            // }
        }

        for ad in lm.buying_ads {
            if !printed_lm_header {
                println!("  --- Local Market:");
                printed_lm_header = true;
            }

            // check to see if anyone is buying at prices higher than the CX
            let price_per_unit = ad.total_price / ad.material_amount as f32;
            let cx = client
                .get_exchange_info(&format!("{}.CI1", ad.material_ticker))
                .await?;
            // let sell_diff = 100.0 * price_per_unit / cx.bid;
            let cx_bid = cx
                .bid
                .map(|b| format!("{:.2}", 100.0 * price_per_unit / b))
                .unwrap_or("N/A".into());

            let instant = cx.instant_buy(ad.material_amount);
            let colored_ticket =
                MaterialWithColor::new(&ad.material_ticker).with_amount(ad.material_amount as i32);

            println!(
                "  {} is buying {colored_ticket} for {} ({:.1} {} per unit / {cx_bid}% / {:.1}% of CX) ",
                ad.creator_company_name,
                // ad.material_amount,
                // ad.material_ticker,
                ad.total_price,
                price_per_unit,
                ad.currency,
                100.0 * price_per_unit / cx.price
            );

            // can we buy this from the CX instantly and sell it to this local market ad for a profit?
            if let Some(instant) = instant {
                let profit = ad.total_price - instant.total_value;
                if profit > 0.0 {
                    println!(
                        "    !!! Can buy at CX for {} and sell to this ad for {} profit",
                        instant.total_value, profit
                    );
                }
            }
        }

        // number of days we want to be able to run before needing a resupply
        let num_days_inventory = 21.0; // 3 weeks

        //A map of our needs, from ticker name to (is_essential, amount_needed)   (where amount_needed is per 3 weeks)
        let mut total_needs: HashMap<String, (bool, f32)> = HashMap::new();

        // get our base inventory for this planet
        let inv = client
            .get_storage_for_user(&username, &planet.id)
            .await?
            .context("No inventory found")?;

        println!("  --- Production Lines:");

        let production_lines = client.get_planet_production(&username, &planet.id).await?;
        // level load across all production lines (negative values indicate daily need)
        let mut total_daily_production = HashMap::new();
        for prod in production_lines {
            // if prod.building_type != "prefabPlant1" { continue }
            // dbg!(&prod);
            let daily = prod.daily_production();
            for (mat, amt) in daily.outputs {
                *total_daily_production.entry(mat).or_insert(0.0) += amt;
            }
            for (mat, amt) in daily.inputs {
                *total_daily_production.entry(mat).or_insert(0.0) -= amt;
            }
        }

        for (material, amount) in &total_daily_production {
            if *amount > 0.0 {
                let colored_material = MaterialWithColor::new(material);
                println!(
                    "  Producing {} per day",
                    colored_material.with_amount(amount.round() as i32),
                );
            }
        }
        for (material, amount) in &total_daily_production {
            if *amount < 0.0 {
                let inv_amount = inv.items.get(material).map(|i| i.quantity).unwrap_or(0);
                let days = inv_amount as f32 / -amount;

                let colored_material = MaterialWithColor::new(material);
                println!(
                    "  Consuming {} per day (lasting {:.1} days)",
                    colored_material.with_amount((-amount.round()) as i32),
                    days
                );
                total_needs.entry(material.clone()).or_default().1 += -amount * num_days_inventory;
            }
        }

        println!("  --- Needs: ");

        // get our workforce requirements
        let workforce = client
            .get_planet_workforce_for_user(&username, &planet.id)
            .await?;

        for (_workforce_type, details) in workforce.details {
            for need in details.needs {
                let entry = total_needs.entry(need.ticker.clone()).or_default();
                entry.0 = need.essential;
                entry.1 += need.units_per_interval * num_days_inventory;
            }
        }

        for (need, (essential, amount)) in total_needs {
            let colored_need = MaterialWithColor::new(&need);

            if let Some(inv) = inv.items.get(&need) {
                let diff = amount - inv.quantity as f32;
                if amount > 0.0 && diff > 0.0 {
                    println!(
                        "  Need {} (have {} / need total of {:.1})",
                        colored_need.with_amount(diff.ceil() as i32),
                        inv.quantity,
                        amount
                    );
                } else {
                    // if we have an excess, check to see if we are already selling it on the LM.  If not, post an ad
                    let need_category = get_material_db().get(need.as_str()).unwrap().category;
                    if !our_selling_orders.contains(&need)
                        && !["DW", "RAT", "COF"].contains(&need.as_str())
                        && (need_category == MaterialCategory::ConsumablesBasic
                            || need_category == MaterialCategory::ConsumablesLuxury)
                    {
                        let cx = client.get_exchange_info(&format!("{need}.CI1")).await?;
                        let lm_fee = if planet.local_market_fee_factor > 0.0 {
                            50.0 + (30.0 * planet.local_market_fee_factor)
                        } else {
                            0.0
                        };
                        let proposed_price = 10.0 * cx.high * 1.15 + lm_fee;
                        let colored_inv = MaterialWithColor::new(&inv.ticker);
                        println!("  +++ We have {} in inventory, only need {}, post a sell order on the LM (proposed 10 units at {proposed_price})",
                        colored_inv.with_amount(inv.quantity as i32),  amount);
                        // println!("  Have excess of {} (have {} / need {})", need, inv.quantity, amount);
                    }
                }
            } else if amount > 0.0 && essential {
                println!(
                    "  Need {} (have no inventory)",
                    colored_need.with_amount(amount.ceil() as i32)
                );
            }
        }
    }

    Ok(())
}

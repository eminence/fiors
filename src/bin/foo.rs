use fiors::FIOClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FIOClient::new_with_key("9dd5160d-acc8-493d-b222-d5f96273f677".into());
    client.local_cache_dir = Some(".fio_cache".into());

    let planets = client.get_storage_planets_for_user("eminence32").await?;
    for planet in planets {
        if !planet.has_local_market {
            continue;
        }
        println!("==> {}", planet.name);

        let lm = client.get_planet_localmarket(&planet.id).await?;
        for ad in lm.selling_ads {
            let price_per_unit = ad.total_price / ad.material_amount as f32;
           
            // find the current benten station market price
            let cx = client.get_exchange_info(&format!("{}.CI1", ad.material_ticker)).await?;

            let buy_diff = price_per_unit / cx.ask;

            println!(
                "  {} is selling {} units of {} ({} {} per unit / {:.2} of cx)",
                ad.creator_company_name,
                ad.material_amount,
                ad.material_ticker,
                price_per_unit,
                ad.currency,
                buy_diff
            );


            if price_per_unit < cx.ask {
                println!("    !!! This is lower than CX offers prices ({} per unit on the CX) -- good deal", cx.ask);
            } else {
                // println!("  --- This is higher than CX prices");
            }
            if price_per_unit < cx.bid {
                let possible_profit = (cx.bid - price_per_unit) * ad.material_amount as f32;
                println!("    !!! This is lower than CX buy prices ({} per unit on the CX) -- possible arbitrage.  Possible profit: {possible_profit:.2} {}", cx.bid, ad.currency);
            }

        }
    }

    Ok(())
}

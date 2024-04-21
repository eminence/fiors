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
    DB.get_or_init(|| { construct_material_db() })
}
pub fn construct_material_db() -> HashMap<&'static str, StaticMaterialInfo> {
    let mut map = HashMap::new();
    map.insert(
        "AAR",
        StaticMaterialInfo {
            material_id: "d4a247f1ae7a17b6e80057866ecdf90d",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "antennaArray",
            ticker: "AAR",
            weight: 0.78f32,
            volume: 0.5f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "ABH",
        StaticMaterialInfo {
            material_id: "274e35935b4ba9f4032f657b0ab6e0e5",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "advancedBulkhead",
            ticker: "ABH",
            weight: 0.6f32,
            volume: 0.9f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "ACS",
        StaticMaterialInfo {
            material_id: "a7a19eb0c3f253884c901bf1d05f2369",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "automatedCoolingSystem",
            ticker: "ACS",
            weight: 0.3f32,
            volume: 0.2f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "ADE",
        StaticMaterialInfo {
            material_id: "bf1f62e450627f2db02884657c399284",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "advancedDeckElements",
            ticker: "ADE",
            weight: 0.4f32,
            volume: 1.5f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "ADR",
        StaticMaterialInfo {
            material_id: "f99a1c9a730d92fd78b7dc7c2d710060",
            category_name: "medical equipment",
            category_id: "ef05c6b81cf109f142dd415ce6d36e67",
            name: "autoDoc",
            ticker: "ADR",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::MedicalEquipment,
        },
    );
    map.insert(
        "ADS",
        StaticMaterialInfo {
            material_id: "03133ed742ebc355fb01a1b363d48a70",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "audioDistributionSystem",
            ticker: "ADS",
            weight: 0.7f32,
            volume: 2f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "AEF",
        StaticMaterialInfo {
            material_id: "5fa39c2298568262540bf6579feb017e",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "aerostatFoundation",
            ticker: "AEF",
            weight: 2f32,
            volume: 5f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "AEN",
        StaticMaterialInfo {
            material_id: "6f5e5b36f990caafe0b6eae133084930",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "advancedEngine",
            ticker: "AEN",
            weight: 14f32,
            volume: 7f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "AFP",
        StaticMaterialInfo {
            material_id: "f02fe7ddd1a92d3b7968fdbc40f8447e",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "advancedFuelPump",
            ticker: "AFP",
            weight: 1f32,
            volume: 0.25f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "AFR",
        StaticMaterialInfo {
            material_id: "b591180c0f681910b0e29ade88b6be62",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "advancedFuelRod",
            ticker: "AFR",
            weight: 0.4f32,
            volume: 0.1f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "AGS",
        StaticMaterialInfo {
            material_id: "0a37a0d116f1271b285b7304e413733b",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "advancedHighgSeats",
            ticker: "AGS",
            weight: 30f32,
            volume: 5f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "AHP",
        StaticMaterialInfo {
            material_id: "686d319fc72c05510a884292da6cb664",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "advancedHullPlate",
            ticker: "AHP",
            weight: 17f32,
            volume: 10f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "AIR",
        StaticMaterialInfo {
            material_id: "cbfa90b487f0d79023e11da44f052f96",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "airScrubber",
            ticker: "AIR",
            weight: 1.7f32,
            volume: 3f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "AL",
        StaticMaterialInfo {
            material_id: "ed72545a5db1f16430d79a240dc61e6c",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "aluminium",
            ticker: "AL",
            weight: 2.7f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "ALE",
        StaticMaterialInfo {
            material_id: "cd0c4bd7aae87c7e402d359b6e908b05",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "technicianLuxuryDrink",
            ticker: "ALE",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "ALG",
        StaticMaterialInfo {
            material_id: "ec6fe0efa83e52fd59f248a069aee9bb",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "proteinAlgae",
            ticker: "ALG",
            weight: 0.7f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "ALO",
        StaticMaterialInfo {
            material_id: "04f72b0c957b769c6d45c011e9c9b4b5",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "aluminiumOre",
            ticker: "ALO",
            weight: 1.35f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "AMM",
        StaticMaterialInfo {
            material_id: "bcf4a2029e461cf76d29de8c66ec2ff9",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "ammonia",
            ticker: "AMM",
            weight: 0.86f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "ANZ",
        StaticMaterialInfo {
            material_id: "5de316b11d1e8f5dc142665ae979ec19",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "advancedNozzle",
            ticker: "ANZ",
            weight: 6f32,
            volume: 3f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "APT",
        StaticMaterialInfo {
            material_id: "815b7171c7647a71758ff3a13bd303a9",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "advancedHeatShield",
            ticker: "APT",
            weight: 0.03f32,
            volume: 0.3f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "AR",
        StaticMaterialInfo {
            material_id: "a014493f8716f1636704dc836acc66ec",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "argon",
            ticker: "AR",
            weight: 1.784f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "ARP",
        StaticMaterialInfo {
            material_id: "e73f2311d23a1dc4e096d2cf389e5ee2",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "advancedRadiationShielding",
            ticker: "ARP",
            weight: 0.04f32,
            volume: 0.2f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "ASE",
        StaticMaterialInfo {
            material_id: "c83a4fc74233f9ffac3f5b7b7b83dce5",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "advancedStructuralElements",
            ticker: "ASE",
            weight: 0.5f32,
            volume: 0.6f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "AST",
        StaticMaterialInfo {
            material_id: "5a3934eb6d4ffc642fab7a358c3fb1b8",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "aluminiumTitaniumAlloy",
            ticker: "AST",
            weight: 4.98f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "ATA",
        StaticMaterialInfo {
            material_id: "a531fd67223d12e94103dff359c50e0d",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "advancedWindow",
            ticker: "ATA",
            weight: 0.3f32,
            volume: 0.4f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "ATP",
        StaticMaterialInfo {
            material_id: "7bfabb5b93eb7a287206e062ce829e29",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "advancedThermalProtectionMaterial",
            ticker: "ATP",
            weight: 2.9f32,
            volume: 1f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "AU",
        StaticMaterialInfo {
            material_id: "de74e4c1ab68d0889a7204f154153da1",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "gold",
            ticker: "AU",
            weight: 19.32f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "AUO",
        StaticMaterialInfo {
            material_id: "3efc94d2a18702eee04ba2850bdb9bef",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "goldOre",
            ticker: "AUO",
            weight: 3.86f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "AWF",
        StaticMaterialInfo {
            material_id: "e40f3373c1b7c0158d69ee9adfe612b1",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "waterFilter",
            ticker: "AWF",
            weight: 0.8f32,
            volume: 1.2f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "AWH",
        StaticMaterialInfo {
            material_id: "8284845e663b10975f8eec5f4426ec29",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "advancedWhippleShielding",
            ticker: "AWH",
            weight: 0.12f32,
            volume: 1f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "BAC",
        StaticMaterialInfo {
            material_id: "6daa4e4e074e1af8fb74458fe2196124",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "bacteria",
            ticker: "BAC",
            weight: 0.15f32,
            volume: 0.15f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "BAI",
        StaticMaterialInfo {
            material_id: "6224081a169a0951a9cac7553d601ba7",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "basicAiFramework",
            ticker: "BAI",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "BBH",
        StaticMaterialInfo {
            material_id: "63b98e9700f6630cbfd9c824e0882388",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "basicBulkhead",
            ticker: "BBH",
            weight: 0.5f32,
            volume: 0.8f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "BCO",
        StaticMaterialInfo {
            material_id: "7bd03426f1ced7f6aa13a6195f494930",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "copperConnectors",
            ticker: "BCO",
            weight: 0.005f32,
            volume: 0.002f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "BDE",
        StaticMaterialInfo {
            material_id: "420b38d32301dfe56403d1f3967d5b86",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "basicDeckElements",
            ticker: "BDE",
            weight: 0.1f32,
            volume: 1.5f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "BE",
        StaticMaterialInfo {
            material_id: "d3c0b6280da00e2ab4ee1ea6046cb488",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "beryllium",
            ticker: "BE",
            weight: 1.84f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "BEA",
        StaticMaterialInfo {
            material_id: "e97e2535fec019710299709a7462c066",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "proteinBeans",
            ticker: "BEA",
            weight: 0.8f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "BER",
        StaticMaterialInfo {
            material_id: "fc9e1183c47d97315434a8b2c768d35c",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "beryl",
            ticker: "BER",
            weight: 1.92f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "BFP",
        StaticMaterialInfo {
            material_id: "3798c56adc243fe2b25d988139939d8d",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "basicFuelPump",
            ticker: "BFP",
            weight: 0.8f32,
            volume: 0.2f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "BFR",
        StaticMaterialInfo {
            material_id: "206d41a1981a67025be0299c67943607",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "basicFuelRod",
            ticker: "BFR",
            weight: 0.3f32,
            volume: 0.1f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "BGC",
        StaticMaterialInfo {
            material_id: "115e4c08cdd519c175ca3d58af562c1e",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "blueGoldConnectors",
            ticker: "BGC",
            weight: 0.01f32,
            volume: 0.002f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "BGO",
        StaticMaterialInfo {
            material_id: "b2ca8883f60fb72011ad33855c531f3a",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "goldIronAlloy",
            ticker: "BGO",
            weight: 19.32f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "BGS",
        StaticMaterialInfo {
            material_id: "9375c2dbd1de92b8ec107677bfeda193",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "basicHighgSeats",
            ticker: "BGS",
            weight: 20f32,
            volume: 3f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "BHP",
        StaticMaterialInfo {
            material_id: "c03644ad9e74ee585b1148f608010003",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "basicHullPlate",
            ticker: "BHP",
            weight: 10f32,
            volume: 10f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "BID",
        StaticMaterialInfo {
            material_id: "1dc2dcd0be19e4272767510d3392fe36",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "fullBodyInteractionDevice",
            ticker: "BID",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "BL",
        StaticMaterialInfo {
            material_id: "aa4fb3a0d1c1cf17199f436b12c59b69",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "breathableLiquid",
            ticker: "BL",
            weight: 1.12f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "BLE",
        StaticMaterialInfo {
            material_id: "5745aed2f78b303ee96528213f99ec9f",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "bleach",
            ticker: "BLE",
            weight: 0.5f32,
            volume: 0.5f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "BMF",
        StaticMaterialInfo {
            material_id: "47952dfcb98b6d573fd3e9461cfdddcf",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "mainFrameBlank",
            ticker: "BMF",
            weight: 0.8f32,
            volume: 1.2f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "BND",
        StaticMaterialInfo {
            material_id: "26c6a0defb9109b7ac3fab559b9d4f65",
            category_name: "medical equipment",
            category_id: "ef05c6b81cf109f142dd415ce6d36e67",
            name: "bandages",
            ticker: "BND",
            weight: 0.001f32,
            volume: 0.005f32,
            category: MaterialCategory::MedicalEquipment,
        },
    );
    map.insert(
        "BOR",
        StaticMaterialInfo {
            material_id: "fba7294b0f74a4a2527f5a7049c70120",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "boronCrystals",
            ticker: "BOR",
            weight: 1.8f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "BOS",
        StaticMaterialInfo {
            material_id: "08adb8c8b129d07b85919cff2fdc2c0d",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "borosilicate",
            ticker: "BOS",
            weight: 1.5f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "BPT",
        StaticMaterialInfo {
            material_id: "5dee73eea0e8e99a6564a011ddc80880",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "basicHeatShield",
            ticker: "BPT",
            weight: 0.02f32,
            volume: 0.3f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "BR1",
        StaticMaterialInfo {
            material_id: "70504de53ce6b9348f725e442586d75c",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "commandBridge1",
            ticker: "BR1",
            weight: 180f32,
            volume: 300f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "BR2",
        StaticMaterialInfo {
            material_id: "73e30b806837b71692e2857fd9aff667",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "commandBridge2",
            ticker: "BR2",
            weight: 280f32,
            volume: 400f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "BRM",
        StaticMaterialInfo {
            material_id: "e97bde9e60d4c60a16205d2c859eda62",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "bioreactiveMineral",
            ticker: "BRM",
            weight: 2.5f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "BRO",
        StaticMaterialInfo {
            material_id: "069cfa526ddedea57b71adc0d4556c21",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "copperAluminiumAlloy",
            ticker: "BRO",
            weight: 8.73f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "BRP",
        StaticMaterialInfo {
            material_id: "6070c50f7e58a7db432416085531a01b",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "basicRadiationShielding",
            ticker: "BRP",
            weight: 0.03f32,
            volume: 0.2f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "BRS",
        StaticMaterialInfo {
            material_id: "bfef679d7f2ac72b24c2a277fdf072ef",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "commandBridgeShort",
            ticker: "BRS",
            weight: 150f32,
            volume: 200f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "BSC",
        StaticMaterialInfo {
            material_id: "c8a952f1ae3dbd4be8a72741efa29880",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "bodyScanner",
            ticker: "BSC",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "BSE",
        StaticMaterialInfo {
            material_id: "6bbdf7d0503153753c8cb23653a4d22b",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "basicStructuralElements",
            ticker: "BSE",
            weight: 0.3f32,
            volume: 0.5f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "BTA",
        StaticMaterialInfo {
            material_id: "334f3510e3265410da15174160fbf8f5",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "basicWindow",
            ticker: "BTA",
            weight: 0.3f32,
            volume: 0.4f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "BTS",
        StaticMaterialInfo {
            material_id: "72b87b633c828f417737f5e2e473083d",
            category_name: "liquids",
            category_id: "473218d3618453bf25d2cd0b5616a72f",
            name: "tungstenResource",
            ticker: "BTS",
            weight: 1.05f32,
            volume: 1f32,
            category: MaterialCategory::Liquids,
        },
    );
    map.insert(
        "BWH",
        StaticMaterialInfo {
            material_id: "42e71bbfad2d69d9716ba14e653f50d4",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "basicWhippleShielding",
            ticker: "BWH",
            weight: 0.1f32,
            volume: 1f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "BWS",
        StaticMaterialInfo {
            material_id: "dca85bbac276e8688496bd8ae06b1185",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "workstationBlank",
            ticker: "BWS",
            weight: 0.05f32,
            volume: 0.1f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "C",
        StaticMaterialInfo {
            material_id: "4bbcc7dc5dd16e9af90528ed04d9c6ba",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "carbon",
            ticker: "C",
            weight: 2.25f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "CA",
        StaticMaterialInfo {
            material_id: "d4b42f6606b6f9d7ca1aea9295154288",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "calcium",
            ticker: "CA",
            weight: 1.54f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "CAF",
        StaticMaterialInfo {
            material_id: "363f35e6ebb48062872112416e49424c",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "caffeinatedBeans",
            ticker: "CAF",
            weight: 0.86f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "CAP",
        StaticMaterialInfo {
            material_id: "2db0ebcc44d7df90defe67cc63903a76",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "capacitor",
            ticker: "CAP",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "CBL",
        StaticMaterialInfo {
            material_id: "7c21fd74ca3684f0df5470d774e694a3",
            category_name: "energy systems",
            category_id: "0509ed0a396f5a004d24ef08e33786fb",
            name: "largeCapacitorBank",
            ticker: "CBL",
            weight: 5.4f32,
            volume: 2.4f32,
            category: MaterialCategory::EnergySystems,
        },
    );
    map.insert(
        "CBM",
        StaticMaterialInfo {
            material_id: "284020181d5731f5119fd98352061ad5",
            category_name: "energy systems",
            category_id: "0509ed0a396f5a004d24ef08e33786fb",
            name: "mediumCapacitorBank",
            ticker: "CBM",
            weight: 3.6f32,
            volume: 1.6f32,
            category: MaterialCategory::EnergySystems,
        },
    );
    map.insert(
        "CBS",
        StaticMaterialInfo {
            material_id: "46fef6e8b458e46414345cf8248410c3",
            category_name: "energy systems",
            category_id: "0509ed0a396f5a004d24ef08e33786fb",
            name: "smallCapacitorBank",
            ticker: "CBS",
            weight: 1.8f32,
            volume: 0.8f32,
            category: MaterialCategory::EnergySystems,
        },
    );
    map.insert(
        "CC",
        StaticMaterialInfo {
            material_id: "c3851a633c5e8c625bfe332e79af7360",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "climateController",
            ticker: "CC",
            weight: 0.5f32,
            volume: 1f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "CCD",
        StaticMaterialInfo {
            material_id: "e0bc5afdf3ca754592f454fdbd813c74",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "crowdControlDrone",
            ticker: "CCD",
            weight: 0.3f32,
            volume: 0.05f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "CD",
        StaticMaterialInfo {
            material_id: "cb9bcbcc36de690abcafb9d5cc944cd4",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "touchScreen",
            ticker: "CD",
            weight: 0.004f32,
            volume: 0.002f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "CF",
        StaticMaterialInfo {
            material_id: "c8cc72aa9babd2ca382be2f358202587",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "ceramicFabric",
            ticker: "CF",
            weight: 2.82f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "CHA",
        StaticMaterialInfo {
            material_id: "099fe9e22a3c6238629445fe518b58ff",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "combustionChamber",
            ticker: "CHA",
            weight: 1.2f32,
            volume: 0.7f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "CL",
        StaticMaterialInfo {
            material_id: "19be1b1e67efe63de8127d057508fe0c",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "chlorine",
            ticker: "CL",
            weight: 3.2f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "CLI",
        StaticMaterialInfo {
            material_id: "4d2a6c0cf6eb404cda3b35820bcbe453",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "caliche",
            ticker: "CLI",
            weight: 2.42f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "CMK",
        StaticMaterialInfo {
            material_id: "67eab8960fc6262250335978d64c9e80",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "coreModuleKit",
            ticker: "CMK",
            weight: 4.56f32,
            volume: 33.2f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "COF",
        StaticMaterialInfo {
            material_id: "9842ad8d9dfdf8e4c2e80117e7b5ebaf",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "pioneerLuxuryDrink",
            ticker: "COF",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "COM",
        StaticMaterialInfo {
            material_id: "ee0e83e198391b1c9affe29ac07623b0",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "communicationSystem",
            ticker: "COM",
            weight: 0.5f32,
            volume: 1.5f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "COT",
        StaticMaterialInfo {
            material_id: "7aeed35adeb209ebdb4985f451ed3e46",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "cottonProcessed",
            ticker: "COT",
            weight: 0.77f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "CQL",
        StaticMaterialInfo {
            material_id: "e04f3be303b7dabe50ef92a354070891",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "crewQuarters",
            ticker: "CQL",
            weight: 75f32,
            volume: 150f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "CQM",
        StaticMaterialInfo {
            material_id: "128870a0d62fc67b9d562bb40ddf3922",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "crewQuartersMed",
            ticker: "CQM",
            weight: 50f32,
            volume: 100f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "CQS",
        StaticMaterialInfo {
            material_id: "9f4c5f6a328a1bef6a989c7ea6b21fb9",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "crewQuartersSmall",
            ticker: "CQS",
            weight: 25f32,
            volume: 50f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "CQT",
        StaticMaterialInfo {
            material_id: "2a75ecd980c6edd020d71a8b8948aa6a",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "crewQuartersTiny",
            ticker: "CQT",
            weight: 12.5f32,
            volume: 25f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "CRU",
        StaticMaterialInfo {
            material_id: "3e9e003ed20e4a62eb811a8a50223a65",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "cryoUnit",
            ticker: "CRU",
            weight: 2.2f32,
            volume: 2f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "CST",
        StaticMaterialInfo {
            material_id: "5a41a18d80ddc6982b61f5d2df496358",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "cryogenicFluid",
            ticker: "CST",
            weight: 1.14f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "CTF",
        StaticMaterialInfo {
            material_id: "5b959b6c0c20016402811352969960c0",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "ceramicTungstenFabric",
            ticker: "CTF",
            weight: 4.32f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "CU",
        StaticMaterialInfo {
            material_id: "495ffb5105f367b76b1310c1d6736ac9",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "copper",
            ticker: "CU",
            weight: 8.92f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "CUO",
        StaticMaterialInfo {
            material_id: "b25e1857f8234dc9779d83d7abea3d77",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "copperOre",
            ticker: "CUO",
            weight: 4.01f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "DA",
        StaticMaterialInfo {
            material_id: "6da55ef2896023daf2a2f119895e2078",
            category_name: "software tools",
            category_id: "1dfe943824ceb52d332b4650ce457218",
            name: "dataAnalyzer",
            ticker: "DA",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareTools,
        },
    );
    map.insert(
        "DCH",
        StaticMaterialInfo {
            material_id: "751120b64901019887c843e4125cd571",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "droneChassis",
            ticker: "DCH",
            weight: 0.2f32,
            volume: 0.03f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "DCL",
        StaticMaterialInfo {
            material_id: "d4c590dd12f7fbd0f3533c27da6c9f19",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "largeDeviceCover",
            ticker: "DCL",
            weight: 0.08f32,
            volume: 0.8f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "DCM",
        StaticMaterialInfo {
            material_id: "36d0064e2b9415e1a1214aed16b17ed2",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "mediumDeviceCover",
            ticker: "DCM",
            weight: 0.04f32,
            volume: 0.4f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "DCS",
        StaticMaterialInfo {
            material_id: "52b88b137a9f71a66a0ff41aaa7075a5",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "smallDeviceCover",
            ticker: "DCS",
            weight: 0.01f32,
            volume: 0.1f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "DD",
        StaticMaterialInfo {
            material_id: "c100446dafeb6899f37f97217e8409af",
            category_name: "software tools",
            category_id: "1dfe943824ceb52d332b4650ce457218",
            name: "distributedDatabase",
            ticker: "DD",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareTools,
        },
    );
    map.insert(
        "DDT",
        StaticMaterialInfo {
            material_id: "fe5be65546733ffa4f619bbc76ac532d",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "pesticides",
            ticker: "DDT",
            weight: 0.11f32,
            volume: 0.1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "DEC",
        StaticMaterialInfo {
            material_id: "1ddffd7f8b1ff5391113582ffac2eff0",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "decorativeElements",
            ticker: "DEC",
            weight: 0.5f32,
            volume: 2f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "DIS",
        StaticMaterialInfo {
            material_id: "71437224d7590e8ff92265a56cb70dee",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "twoDimensionalDisplay",
            ticker: "DIS",
            weight: 0.02f32,
            volume: 0.02f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "DOU",
        StaticMaterialInfo {
            material_id: "3da2d0cffdac26559504aa8fadfcdee2",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "droneOperationsUnit",
            ticker: "DOU",
            weight: 5f32,
            volume: 4f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "DRF",
        StaticMaterialInfo {
            material_id: "ab5016ca0c184474d80e00ee5f8a8b66",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "droneFrame",
            ticker: "DRF",
            weight: 0.1f32,
            volume: 0.02f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "DV",
        StaticMaterialInfo {
            material_id: "005936105ae3d0f6197349ad693e9240",
            category_name: "software tools",
            category_id: "1dfe943824ceb52d332b4650ce457218",
            name: "dataVisualizer",
            ticker: "DV",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareTools,
        },
    );
    map.insert(
        "DW",
        StaticMaterialInfo {
            material_id: "4fca6f5b5e6c3b8a1b887c6dc99db146",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "drinkingWater",
            ticker: "DW",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "EDC",
        StaticMaterialInfo {
            material_id: "a36d549d1340c1d489cc4a9ccccf22d0",
            category_name: "software tools",
            category_id: "1dfe943824ceb52d332b4650ce457218",
            name: "entertainmentDataCore",
            ticker: "EDC",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareTools,
        },
    );
    map.insert(
        "EES",
        StaticMaterialInfo {
            material_id: "082332472d3285c985367678a5716e71",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "enrichedEinsteinium",
            ticker: "EES",
            weight: 9.2f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "ENG",
        StaticMaterialInfo {
            material_id: "39cbe35342c34d16bc02009130920bb4",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "standardEngine",
            ticker: "ENG",
            weight: 8f32,
            volume: 4f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "EPO",
        StaticMaterialInfo {
            material_id: "78321af6ac08c494186fafff3965034f",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "epoxy",
            ticker: "EPO",
            weight: 0.04f32,
            volume: 0.02f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "ES",
        StaticMaterialInfo {
            material_id: "a528afba5c7d4bf1fd2f2c69fc7f247f",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "einsteinium",
            ticker: "ES",
            weight: 0.88f32,
            volume: 0.1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "ETC",
        StaticMaterialInfo {
            material_id: "a9d7758454f7a446565e3ce1cd49386a",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "enrichedTechnetium",
            ticker: "ETC",
            weight: 4.1f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "EXO",
        StaticMaterialInfo {
            material_id: "87dd9b56a69e118e8c778de2dce11e8c",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "settlerClothing",
            ticker: "EXO",
            weight: 0.1f32,
            volume: 0.05f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "F",
        StaticMaterialInfo {
            material_id: "ee320953d3d725c83bfbc594449979a2",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "fluorine",
            ticker: "F",
            weight: 1.696f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "FAL",
        StaticMaterialInfo {
            material_id: "78b8206e505d8970f0d9a8f56885ec97",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "aluminiumIronAlloy",
            ticker: "FAL",
            weight: 3.22f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "FAN",
        StaticMaterialInfo {
            material_id: "09a313c77887ec04c209c0153f7dee75",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "coolingFan",
            ticker: "FAN",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "FC",
        StaticMaterialInfo {
            material_id: "a2b6316f3bc85f3e1b05ae36e49faa4c",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "flowControl",
            ticker: "FC",
            weight: 0.5f32,
            volume: 0.25f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "FE",
        StaticMaterialInfo {
            material_id: "7921cd20f7add0b26ff77f76e3e911f8",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "iron",
            ticker: "FE",
            weight: 7.874f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "FEO",
        StaticMaterialInfo {
            material_id: "b9640b0d66e7d0ca7e4d3132711c97fc",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "ironOre",
            ticker: "FEO",
            weight: 5.9f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "FET",
        StaticMaterialInfo {
            material_id: "d7ac798b0457e67da544fbccfa9367f6",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "ironTitaniumAlloy",
            ticker: "FET",
            weight: 6.85f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "FF",
        StaticMaterialInfo {
            material_id: "7a2a12cd6872b89a6b97532618788a4d",
            category_name: "fuels",
            category_id: "ba98fa0cf77040a96cd8a608ad0d08e9",
            name: "ftlFuel",
            ticker: "FF",
            weight: 0.05f32,
            volume: 0.01f32,
            category: MaterialCategory::Fuels,
        },
    );
    map.insert(
        "FFC",
        StaticMaterialInfo {
            material_id: "11cfa0cf3614e7a2405a31039c44632e",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "ftlFieldController",
            ticker: "FFC",
            weight: 50f32,
            volume: 16f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "FIM",
        StaticMaterialInfo {
            material_id: "73e138a825968b0132230d02591af63f",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "engineerFood",
            ticker: "FIM",
            weight: 0.55f32,
            volume: 0.5f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "FIR",
        StaticMaterialInfo {
            material_id: "c2ea542d15989d97a9d1b9c89f2b0b07",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "fissionReactor",
            ticker: "FIR",
            weight: 7f32,
            volume: 4.9f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "FLO",
        StaticMaterialInfo {
            material_id: "cc12f1c859c25f13867485ea395f34c0",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "floatingTank",
            ticker: "FLO",
            weight: 1f32,
            volume: 2f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "FLP",
        StaticMaterialInfo {
            material_id: "f1d402957a88fd396f302372a6912158",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "fluidPiping",
            ticker: "FLP",
            weight: 0.3f32,
            volume: 2f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "FLX",
        StaticMaterialInfo {
            material_id: "51c829b6e62e70223024ffa6688db9f7",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "flux",
            ticker: "FLX",
            weight: 0.25f32,
            volume: 0.1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "FOD",
        StaticMaterialInfo {
            material_id: "c54c5f96af3bec918be316b4469307cb",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "allPurposeFodder",
            ticker: "FOD",
            weight: 1.2f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "FSE",
        StaticMaterialInfo {
            material_id: "d643ca86b7ddbc8ab087ac9e2fcb0980",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "fuelSavingEngine",
            ticker: "FSE",
            weight: 6f32,
            volume: 3f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "FUN",
        StaticMaterialInfo {
            material_id: "10469071872e3221bb8ec42c79368f96",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "entertainmentUnit",
            ticker: "FUN",
            weight: 5f32,
            volume: 4f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "GAL",
        StaticMaterialInfo {
            material_id: "c5236c5b79b9e75ddbce16887aeda338",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "galerite",
            ticker: "GAL",
            weight: 2.51f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "GC",
        StaticMaterialInfo {
            material_id: "ce50248c2878dc89de170da84bbee064",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "gasContainer",
            ticker: "GC",
            weight: 0.05f32,
            volume: 0.1f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "GCH",
        StaticMaterialInfo {
            material_id: "8665fabacd64adfbc74ee255b00ae019",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "glassCombustionChamber",
            ticker: "GCH",
            weight: 1f32,
            volume: 0.6f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "GEN",
        StaticMaterialInfo {
            material_id: "4c38903fc8fd2f87b1877ab9d47cb57e",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "glassEngine",
            ticker: "GEN",
            weight: 5f32,
            volume: 3f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "GIN",
        StaticMaterialInfo {
            material_id: "a9c439ab4a206915c370ad05e0af62d9",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "engineerLuxuryDrink",
            ticker: "GIN",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "GL",
        StaticMaterialInfo {
            material_id: "fbb04a8587195e52ce6df8bc238675dd",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "translucentMaterial",
            ticker: "GL",
            weight: 0.016f32,
            volume: 0.01f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "GNZ",
        StaticMaterialInfo {
            material_id: "77638e63e83ace0a7e33667d362a3a91",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "glassNozzle",
            ticker: "GNZ",
            weight: 1.5f32,
            volume: 1f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "GRA",
        StaticMaterialInfo {
            material_id: "ccef250eff466c569c12c2b711a0d5c2",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "grapes",
            ticker: "GRA",
            weight: 1.1f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "GRN",
        StaticMaterialInfo {
            material_id: "83d83e8cf6fb9018ae7138a1b5d4b19c",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "carbohydrateGrains",
            ticker: "GRN",
            weight: 0.9f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "GV",
        StaticMaterialInfo {
            material_id: "67516facbdb77c8d6fe49162f2ca3917",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "gasVent",
            ticker: "GV",
            weight: 0.25f32,
            volume: 0.15f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "H",
        StaticMaterialInfo {
            material_id: "c04de28c3e717f236d6ed177b89d9523",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "hydrogen",
            ticker: "H",
            weight: 0.07f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "H2O",
        StaticMaterialInfo {
            material_id: "ec8dbb1d3f51d89c61b6f58fdd64a7f0",
            category_name: "liquids",
            category_id: "473218d3618453bf25d2cd0b5616a72f",
            name: "water",
            ticker: "H2O",
            weight: 0.2f32,
            volume: 0.2f32,
            category: MaterialCategory::Liquids,
        },
    );
    map.insert(
        "HAB",
        StaticMaterialInfo {
            material_id: "cbe44387c37259ac71716382c0016bbf",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "habitatUnit",
            ticker: "HAB",
            weight: 10f32,
            volume: 8f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "HAL",
        StaticMaterialInfo {
            material_id: "7e3f98a49ce2d3c8361975de8b4bbea4",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "halite",
            ticker: "HAL",
            weight: 2.17f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "HCC",
        StaticMaterialInfo {
            material_id: "eab56d3d6b0742ccbf19a7e955209b0e",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "redGoldConnectors",
            ticker: "HCC",
            weight: 0.01f32,
            volume: 0.002f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "HCP",
        StaticMaterialInfo {
            material_id: "c76b4100059661084315938999623c26",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "hydrocarbonPlants",
            ticker: "HCP",
            weight: 0.8f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "HD",
        StaticMaterialInfo {
            material_id: "9409ba6022b2767eb16bbee6d680cf8c",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "holographicDisplay",
            ticker: "HD",
            weight: 0.05f32,
            volume: 2f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "HE",
        StaticMaterialInfo {
            material_id: "e9dc6ec89adb62b9ef83031d89e3ce99",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "helium",
            ticker: "HE",
            weight: 0.145f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "HE3",
        StaticMaterialInfo {
            material_id: "4e10b93baa6166496b204e0531347959",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "helium3",
            ticker: "HE3",
            weight: 0.145f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "HER",
        StaticMaterialInfo {
            material_id: "1a1f13b7fc3e0fb6aa292b7c28764f93",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "herbs",
            ticker: "HER",
            weight: 0.4f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "HEX",
        StaticMaterialInfo {
            material_id: "2adebc00bf7efaa693021426a68a3e65",
            category_name: "liquids",
            category_id: "473218d3618453bf25d2cd0b5616a72f",
            name: "heliotropeExtract",
            ticker: "HEX",
            weight: 1.1f32,
            volume: 1f32,
            category: MaterialCategory::Liquids,
        },
    );
    map.insert(
        "HHP",
        StaticMaterialInfo {
            material_id: "bb576032a158719b514c2b3f3e87e30a",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "hardenedHullPlate",
            ticker: "HHP",
            weight: 14f32,
            volume: 10f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "HMS",
        StaticMaterialInfo {
            material_id: "5b0e02208dbbb99695707001d626963a",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "technicianClothing",
            ticker: "HMS",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "HNZ",
        StaticMaterialInfo {
            material_id: "f4316553b9594a88e21536a897f0cab4",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "hyperthrustNozzle",
            ticker: "HNZ",
            weight: 6f32,
            volume: 12f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "HOG",
        StaticMaterialInfo {
            material_id: "488a45ca9f9a3be0bbbe7b4f53ef5b31",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "holographicGlasses",
            ticker: "HOG",
            weight: 0.01f32,
            volume: 0.01f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "HOP",
        StaticMaterialInfo {
            material_id: "d7a9a96a18f278aa7c3f9af30fc68d19",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "antibacterialTreeFlowers",
            ticker: "HOP",
            weight: 0.35f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "HPC",
        StaticMaterialInfo {
            material_id: "3795b86284befb6c493ccd380258c970",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "touchDeviceBlank",
            ticker: "HPC",
            weight: 0.003f32,
            volume: 0.003f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "HPR",
        StaticMaterialInfo {
            material_id: "93a78d6bcf54aef3f2ea93f1238ce79a",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "highPowerReactor",
            ticker: "HPR",
            weight: 16f32,
            volume: 15f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "HSE",
        StaticMaterialInfo {
            material_id: "ec3491b37050a8c780dc514033131843",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "hardenedStructuralElements",
            ticker: "HSE",
            weight: 3.1f32,
            volume: 0.7f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "HSS",
        StaticMaterialInfo {
            material_id: "379f7bc12b6d2800c0a9f72c4943628b",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "engineerClothing",
            ticker: "HSS",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "HTE",
        StaticMaterialInfo {
            material_id: "7075b9314a25ea13f973a1f2d2dad18e",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "hyperthrustEngine",
            ticker: "HTE",
            weight: 16f32,
            volume: 10f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "HYR",
        StaticMaterialInfo {
            material_id: "844341ef73a82d657b64050be2632c53",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "hyperPowerReactor",
            ticker: "HYR",
            weight: 25f32,
            volume: 25f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "I",
        StaticMaterialInfo {
            material_id: "18237d01e53d08212ee0b67dc1ad3df1",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "iodine",
            ticker: "I",
            weight: 4.93f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "IDC",
        StaticMaterialInfo {
            material_id: "5cd2b3729f1de46d80cbf7300c905bef",
            category_name: "software systems",
            category_id: "9e8bc2a092cc3361b6949eca3e95921a",
            name: "informationDataCore",
            ticker: "IDC",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareSystems,
        },
    );
    map.insert(
        "IMM",
        StaticMaterialInfo {
            material_id: "6f09054d36298a23458bfb9942aea6ac",
            category_name: "software systems",
            category_id: "9e8bc2a092cc3361b6949eca3e95921a",
            name: "informationManagementSystem",
            ticker: "IMM",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareSystems,
        },
    );
    map.insert(
        "IND",
        StaticMaterialInfo {
            material_id: "01dfc4e798680d1174fd785db6578627",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "indigo",
            ticker: "IND",
            weight: 0.21f32,
            volume: 0.2f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "INS",
        StaticMaterialInfo {
            material_id: "b88aaf3dba6caa7e6d437e49e7620f0b",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "insuFoam",
            ticker: "INS",
            weight: 0.06f32,
            volume: 0.1f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "JUI",
        StaticMaterialInfo {
            material_id: "f8fdd918a757d6a9888f544e3375e585",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "sedativeSubstance",
            ticker: "JUI",
            weight: 1.2f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "KOM",
        StaticMaterialInfo {
            material_id: "a1d1d61f8085de53455dd4e8a7813788",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "settlerLuxuryDrink",
            ticker: "KOM",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "KV",
        StaticMaterialInfo {
            material_id: "76e7790d43219588122f401f2b9f0534",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "kevlar",
            ticker: "KV",
            weight: 1.65f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "LBH",
        StaticMaterialInfo {
            material_id: "772408d48cf5842fb003698aaa7a6a94",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "lightweightBulkhead",
            ticker: "LBH",
            weight: 0.2f32,
            volume: 0.6f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "LC",
        StaticMaterialInfo {
            material_id: "d6c283d4480cacbca31bec0bbb55b940",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "scientistClothing",
            ticker: "LC",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "LCB",
        StaticMaterialInfo {
            material_id: "d140bdc3f13c831ab8931e330c8d6120",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "largeCargoBay",
            ticker: "LCB",
            weight: 200f32,
            volume: 200f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "LCR",
        StaticMaterialInfo {
            material_id: "17a9e15840c1d90848c5e59949f7abc2",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "liquidCrystals",
            ticker: "LCR",
            weight: 0.15f32,
            volume: 0.1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "LD",
        StaticMaterialInfo {
            material_id: "2fae5ed4c15f7d668b9e708dbfc77008",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "localDatabase",
            ticker: "LD",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "LDE",
        StaticMaterialInfo {
            material_id: "3010d90fde69729f4754b092fdcda826",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "lightweightDeckElements",
            ticker: "LDE",
            weight: 0.1f32,
            volume: 1.2f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "LDI",
        StaticMaterialInfo {
            material_id: "291cd8f5c28b371afe0793f78f5499a8",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "laserDiode",
            ticker: "LDI",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "LES",
        StaticMaterialInfo {
            material_id: "14d35a43454dd8e9797fc36f4ee59698",
            category_name: "liquids",
            category_id: "473218d3618453bf25d2cd0b5616a72f",
            name: "liquidEinsteinium",
            ticker: "LES",
            weight: 8.84f32,
            volume: 1f32,
            category: MaterialCategory::Liquids,
        },
    );
    map.insert(
        "LFE",
        StaticMaterialInfo {
            material_id: "e892dd87f4f499f96bdbaa2ca5965ce9",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "largeEmitter",
            ticker: "LFE",
            weight: 0.4f32,
            volume: 1.6f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "LFL",
        StaticMaterialInfo {
            material_id: "98750f7d48ab2115293c8a8334b0120e",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "largeFtlTank",
            ticker: "LFL",
            weight: 60f32,
            volume: 10f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "LFP",
        StaticMaterialInfo {
            material_id: "b2ba495fc0643c5e5c71195be14aef0e",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "lowHeatFuelPump",
            ticker: "LFP",
            weight: 0.5f32,
            volume: 0.1f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "LHP",
        StaticMaterialInfo {
            material_id: "f3f12b5b3adebdd25f3067e5aa4589f0",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "lightweightHullPlate",
            ticker: "LHP",
            weight: 5f32,
            volume: 10f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "LI",
        StaticMaterialInfo {
            material_id: "295bef52967a15fda26f5968976d29b8",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "lithium",
            ticker: "LI",
            weight: 0.55f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "LIO",
        StaticMaterialInfo {
            material_id: "6dd01a9f12e493d44bbb8ce578f7a2d2",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "lithiumOre",
            ticker: "LIO",
            weight: 2.75f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "LIS",
        StaticMaterialInfo {
            material_id: "05be9d04aad5bb7ea6ed6db76102399f",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "lifeSupportSystem",
            ticker: "LIS",
            weight: 5.6f32,
            volume: 7f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "LIT",
        StaticMaterialInfo {
            material_id: "3395e6f815495c32d54823f350e5c15d",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "neonLightingSystem",
            ticker: "LIT",
            weight: 1f32,
            volume: 2f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "LOG",
        StaticMaterialInfo {
            material_id: "d0d20f604c3b3348cad620a02c664d57",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "logisticsSystem",
            ticker: "LOG",
            weight: 0.5f32,
            volume: 1.5f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "LSE",
        StaticMaterialInfo {
            material_id: "8d4e0986419d2769fe50ac6497be6b7e",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "lightweightStructuralElements",
            ticker: "LSE",
            weight: 0.3f32,
            volume: 1.2f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "LSL",
        StaticMaterialInfo {
            material_id: "684d77b82143e23d746461d1a55a6711",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "largeStlTank",
            ticker: "LSL",
            weight: 125f32,
            volume: 100f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "LST",
        StaticMaterialInfo {
            material_id: "1f9a0293d9ba9bf519f71432e695edeb",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "limestone",
            ticker: "LST",
            weight: 2.73f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "LTA",
        StaticMaterialInfo {
            material_id: "b29a0e901300857a8672f0643f7d796a",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "lightweightWindow",
            ticker: "LTA",
            weight: 0.3f32,
            volume: 0.5f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "LU",
        StaticMaterialInfo {
            material_id: "1b05fb8506fcd5be63a465171dd4c656",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "laboratoryUnit",
            ticker: "LU",
            weight: 8f32,
            volume: 6f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "MAG",
        StaticMaterialInfo {
            material_id: "24a2518b138203909eb13f330ac298d3",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "magnetite",
            ticker: "MAG",
            weight: 5.15f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "MAI",
        StaticMaterialInfo {
            material_id: "a381f9434f9105a4f8179f6c6cffe007",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "carbohydrateMaize",
            ticker: "MAI",
            weight: 1.3f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "MB",
        StaticMaterialInfo {
            material_id: "0995a123c2c2e60c9b0d4f120abac7bb",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "motherBoard",
            ticker: "MB",
            weight: 0.075f32,
            volume: 0.075f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "MCB",
        StaticMaterialInfo {
            material_id: "ca35c8015a968c985e6a2a8fbb6b7664",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "mediumCargoBay",
            ticker: "MCB",
            weight: 100f32,
            volume: 100f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "MCG",
        StaticMaterialInfo {
            material_id: "cc5fb1618f0506e80bfbcee9ae2605ab",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "mineralConstructionGranulate",
            ticker: "MCG",
            weight: 0.24f32,
            volume: 0.1f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "MEA",
        StaticMaterialInfo {
            material_id: "a5de3e325e87c874fe395949728bb276",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "scientistFood",
            ticker: "MEA",
            weight: 0.6f32,
            volume: 0.5f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "MED",
        StaticMaterialInfo {
            material_id: "5d36829593b5fd61674320cd66e4bb5c",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "technicianHealth",
            ticker: "MED",
            weight: 0.3f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "MFE",
        StaticMaterialInfo {
            material_id: "943fff6ce1b470ea031432a15a9afe47",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "mediumEmitter",
            ticker: "MFE",
            weight: 0.2f32,
            volume: 0.8f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "MFK",
        StaticMaterialInfo {
            material_id: "432414a579f1fccd66136b257eb44408",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "fastenerKitMedium",
            ticker: "MFK",
            weight: 0.1f32,
            volume: 0.05f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "MFL",
        StaticMaterialInfo {
            material_id: "4fce81e03539c346cc5c493c197eac69",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "mediumFtlTank",
            ticker: "MFL",
            weight: 24f32,
            volume: 4f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "MG",
        StaticMaterialInfo {
            material_id: "833efac228452f5d770c0693c3fb3d50",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "magnesium",
            ticker: "MG",
            weight: 0.27f32,
            volume: 0.16f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "MGC",
        StaticMaterialInfo {
            material_id: "4f26e8b4f9cb1373311fcd3323a53bdd",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "magneticFloor",
            ticker: "MGC",
            weight: 0.6f32,
            volume: 0.9f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "MGS",
        StaticMaterialInfo {
            material_id: "51531f3012276e93e60cb06b194bac7f",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "magnesite",
            ticker: "MGS",
            weight: 1.73f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "MHL",
        StaticMaterialInfo {
            material_id: "9282cf0c801f5b5adcee3b44621ea1bb",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "metalHalideLamp",
            ticker: "MHL",
            weight: 0.1f32,
            volume: 0.05f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "MHP",
        StaticMaterialInfo {
            material_id: "33ae4b6e0e01638f33a18ab9b0bf917d",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "microHeadphones",
            ticker: "MHP",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "MLI",
        StaticMaterialInfo {
            material_id: "4d3d0c15f18a19ec8147adb0f3bd39e6",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "machineLearningInterface",
            ticker: "MLI",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "MPC",
        StaticMaterialInfo {
            material_id: "ef1a8b643e865195823ec884b3343c6a",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "microProcessor",
            ticker: "MPC",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "MSL",
        StaticMaterialInfo {
            material_id: "11fa820041249299d95d85191dc378fc",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "mediumStlTank",
            ticker: "MSL",
            weight: 50f32,
            volume: 50f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "MTC",
        StaticMaterialInfo {
            material_id: "edea6e957426c9531b19ce53fec47ae7",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "megaTubeCoating",
            ticker: "MTC",
            weight: 0.032f32,
            volume: 0.01f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "MTP",
        StaticMaterialInfo {
            material_id: "52cc5c8aeab6bb1778e7ce93a1bbdc80",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "meat",
            ticker: "MTP",
            weight: 0.7f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "MUS",
        StaticMaterialInfo {
            material_id: "005252da5904d755c10d3abd4442bdeb",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "mushrooms",
            ticker: "MUS",
            weight: 0.8f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "MWF",
        StaticMaterialInfo {
            material_id: "ef36cda3528c490360fa8d513a53e6c6",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "waferMedium",
            ticker: "MWF",
            weight: 0.008f32,
            volume: 0.008f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "N",
        StaticMaterialInfo {
            material_id: "5d59ddddb1ce9a26eb81b78b8378b3ac",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "nitrogen",
            ticker: "N",
            weight: 0.807f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "NA",
        StaticMaterialInfo {
            material_id: "11ec662cd8ba85e87b111a7ba7aa4973",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "sodium",
            ticker: "NA",
            weight: 0.97f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "NAB",
        StaticMaterialInfo {
            material_id: "710cb599231cd6b974bcc5feea9603c7",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "sodiumBorohydride",
            ticker: "NAB",
            weight: 0.1f32,
            volume: 0.05f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "NCS",
        StaticMaterialInfo {
            material_id: "08b3d1994643df5efe095b335be3a029",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "nanoCarbonSheeting",
            ticker: "NCS",
            weight: 0.028f32,
            volume: 0.01f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "NE",
        StaticMaterialInfo {
            material_id: "adca908f9dd26ff505d01bc5ffc0d2e1",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "neon",
            ticker: "NE",
            weight: 0.9f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "NF",
        StaticMaterialInfo {
            material_id: "d26bd2ac52ea6061a923f000cd104967",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "networkingFramework",
            ticker: "NF",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "NFI",
        StaticMaterialInfo {
            material_id: "2689b2d4859b0cb73e7efa9466301297",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "nanoFiber",
            ticker: "NFI",
            weight: 0.032f32,
            volume: 0.01f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "NG",
        StaticMaterialInfo {
            material_id: "8faa9fb6d21ebd5731043b0c122bc5b6",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "nanoGlass",
            ticker: "NG",
            weight: 0.022f32,
            volume: 0.01f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "NL",
        StaticMaterialInfo {
            material_id: "12fb94d0184c18455533bffb4b8b7e51",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "nylon",
            ticker: "NL",
            weight: 1.13f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "NN",
        StaticMaterialInfo {
            material_id: "c06935922eac3dccb7d7a368a899d309",
            category_name: "software tools",
            category_id: "1dfe943824ceb52d332b4650ce457218",
            name: "neuralNetwork",
            ticker: "NN",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareTools,
        },
    );
    map.insert(
        "NOZ",
        StaticMaterialInfo {
            material_id: "f4cb7556739ec63fe3e019ee0f798263",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "basicNozzle",
            ticker: "NOZ",
            weight: 3f32,
            volume: 1.5f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "NR",
        StaticMaterialInfo {
            material_id: "17229492d319a4d6a5d0d45ee5bef188",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "nanoResin",
            ticker: "NR",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "NS",
        StaticMaterialInfo {
            material_id: "d9cfb2948188f49bfae2b6d9972837c8",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "nutrientSolution",
            ticker: "NS",
            weight: 0.6f32,
            volume: 0.5f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "NST",
        StaticMaterialInfo {
            material_id: "a636755a8531daa76ab6b1f95fd5d5e3",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "scientistLuxuryHealth",
            ticker: "NST",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "NUT",
        StaticMaterialInfo {
            material_id: "87488698340ed6d9db87e26e35360c20",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "fattyNuts",
            ticker: "NUT",
            weight: 0.9f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "NV1",
        StaticMaterialInfo {
            material_id: "e8e7a34cb5e2a62aaf8d83dc79d81ee5",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "navigation1",
            ticker: "NV1",
            weight: 4.2f32,
            volume: 2f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "NV2",
        StaticMaterialInfo {
            material_id: "ccdecb3fc5eb9f0d04c67cc1c9fb47dd",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "navigation2",
            ticker: "NV2",
            weight: 6.7f32,
            volume: 3f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "O",
        StaticMaterialInfo {
            material_id: "6e16dbf050b98d9c4fc9c615b3367a0f",
            category_name: "gases",
            category_id: "54dec1e42eba44dda423dd774393326a",
            name: "oxygen",
            ticker: "O",
            weight: 1.141f32,
            volume: 1f32,
            category: MaterialCategory::Gases,
        },
    );
    map.insert(
        "OFF",
        StaticMaterialInfo {
            material_id: "26c05c3b63aa57025e68dd67d0001ba2",
            category_name: "utility",
            category_id: "4eb124000ee0e38af1daf5aced313823",
            name: "officeSupplies",
            ticker: "OFF",
            weight: 0.02f32,
            volume: 0.2f32,
            category: MaterialCategory::Utility,
        },
    );
    map.insert(
        "OLF",
        StaticMaterialInfo {
            material_id: "aa6636f6018e23e92e52aa74d65e3b7a",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "olfactorySubstances",
            ticker: "OLF",
            weight: 0.01f32,
            volume: 0.001f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "OS",
        StaticMaterialInfo {
            material_id: "8bdcb737ea5ad7fde81d2b8c78cfd0aa",
            category_name: "software tools",
            category_id: "1dfe943824ceb52d332b4650ce457218",
            name: "operatingSystem",
            ticker: "OS",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareTools,
        },
    );
    map.insert(
        "OVE",
        StaticMaterialInfo {
            material_id: "9788890100fd2191fb065cb0d5e624cb",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "pioneerClothing",
            ticker: "OVE",
            weight: 0.02f32,
            volume: 0.025f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "PCB",
        StaticMaterialInfo {
            material_id: "9da5f471dfc7759a11858456e74fbc8f",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "printedCircuitBoard",
            ticker: "PCB",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "PDA",
        StaticMaterialInfo {
            material_id: "1d2a14dda2d950fe0dfa51cb76e79e54",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "engineerTools",
            ticker: "PDA",
            weight: 0.002f32,
            volume: 0.002f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "PE",
        StaticMaterialInfo {
            material_id: "ee8e92a632d1b5f4847fe9d4e376aeca",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "polyEthylene",
            ticker: "PE",
            weight: 0.01f32,
            volume: 0.01f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "PFE",
        StaticMaterialInfo {
            material_id: "d767c625cdf6f6f97df9fe8e49766912",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "premiumFertilizer",
            ticker: "PFE",
            weight: 0.9f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "PG",
        StaticMaterialInfo {
            material_id: "64666b2eb0faaba594a62818e48229b8",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "polymerGranulate",
            ticker: "PG",
            weight: 0.002f32,
            volume: 0.001f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "PIB",
        StaticMaterialInfo {
            material_id: "746983ed94132dfa021aac40b67754b2",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "pineberries",
            ticker: "PIB",
            weight: 0.3f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "PK",
        StaticMaterialInfo {
            material_id: "4854eb1479ec6225b01d337a172d7b02",
            category_name: "medical equipment",
            category_id: "ef05c6b81cf109f142dd415ce6d36e67",
            name: "painkillers",
            ticker: "PK",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::MedicalEquipment,
        },
    );
    map.insert(
        "POW",
        StaticMaterialInfo {
            material_id: "edb9e2c57c966f28c66f441bda0b030f",
            category_name: "energy systems",
            category_id: "0509ed0a396f5a004d24ef08e33786fb",
            name: "powerCell",
            ticker: "POW",
            weight: 0.05f32,
            volume: 0.1f32,
            category: MaterialCategory::EnergySystems,
        },
    );
    map.insert(
        "PPA",
        StaticMaterialInfo {
            material_id: "5f2011b6c605256dfa4626776cc7cf3c",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "proteinPaste",
            ticker: "PPA",
            weight: 2f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "PSH",
        StaticMaterialInfo {
            material_id: "eebc3c4618186afa73bbe0a19be73eed",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "pressureShielding",
            ticker: "PSH",
            weight: 4.2f32,
            volume: 0.8f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "PSL",
        StaticMaterialInfo {
            material_id: "db4fbfd5c434a3952e7e1d37ae36158f",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "largePlasticsBoard",
            ticker: "PSL",
            weight: 0.08f32,
            volume: 0.8f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "PSM",
        StaticMaterialInfo {
            material_id: "d1da92df0f3cd523b647245fb79b5025",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "mediumPlasticsBoard",
            ticker: "PSM",
            weight: 0.04f32,
            volume: 0.4f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "PSS",
        StaticMaterialInfo {
            material_id: "7f1b9df7face3f9b3d99e34f9a51c6b2",
            category_name: "plastics",
            category_id: "6316282906a9f68b0c7bb4396a26aa95",
            name: "smallPlasticsBoard",
            ticker: "PSS",
            weight: 0.01f32,
            volume: 0.1f32,
            category: MaterialCategory::Plastics,
        },
    );
    map.insert(
        "PT",
        StaticMaterialInfo {
            material_id: "b5744244bb7913cee262ec29abc3b9d0",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "settlerTools",
            ticker: "PT",
            weight: 0.25f32,
            volume: 0.2f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "PWO",
        StaticMaterialInfo {
            material_id: "820d081096fd3e8fbd98b4344f6250ad",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "pioneerLuxuryClothing",
            ticker: "PWO",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "QCR",
        StaticMaterialInfo {
            material_id: "07ba4d80010a9d5c869aa6f05ea78208",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "quickChargeReactor",
            ticker: "QCR",
            weight: 14f32,
            volume: 10f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "RAD",
        StaticMaterialInfo {
            material_id: "c8e78ba6113f8db69d14ae620ddbad06",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "radioDevice",
            ticker: "RAD",
            weight: 0.003f32,
            volume: 0.005f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "RAG",
        StaticMaterialInfo {
            material_id: "c855d1fe267c1f5fca20459db2bcd1b0",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "radioisotopeGenerator",
            ticker: "RAG",
            weight: 5f32,
            volume: 3.4f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "RAM",
        StaticMaterialInfo {
            material_id: "206aa44d627bd8e886a20c565b4bd058",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "memoryBank",
            ticker: "RAM",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "RAT",
        StaticMaterialInfo {
            material_id: "83dd61885cf6879ff49fe1419f068f10",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "rations",
            ticker: "RAT",
            weight: 0.21f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "RBH",
        StaticMaterialInfo {
            material_id: "9930460f6d54364954dcf0af905029c5",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "reinforcedBulkhead",
            ticker: "RBH",
            weight: 2.4f32,
            volume: 0.9f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "RCO",
        StaticMaterialInfo {
            material_id: "1205efe7810a94592524a53e87c3299b",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "cottonRaw",
            ticker: "RCO",
            weight: 0.95f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "RCS",
        StaticMaterialInfo {
            material_id: "e5bea4cd29eccc0162885f338cd1c235",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "reactorControlSystem",
            ticker: "RCS",
            weight: 0.67f32,
            volume: 0.5f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "RCT",
        StaticMaterialInfo {
            material_id: "17147bdc09c3762514c78f12dc17b79a",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "standardReactor",
            ticker: "RCT",
            weight: 7f32,
            volume: 4f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "RDE",
        StaticMaterialInfo {
            material_id: "165dab5e1f913b2a4769f57f64c2505e",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "reinforcedDeckElements",
            ticker: "RDE",
            weight: 1.4f32,
            volume: 1.5f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "RDL",
        StaticMaterialInfo {
            material_id: "4d7a3d9cf07cd3c3be0cd0092afd0ea8",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "largeShipRepairDroneUnit",
            ticker: "RDL",
            weight: 150f32,
            volume: 30f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "RDS",
        StaticMaterialInfo {
            material_id: "addf3b08bfd443feb2c5f51c776155f5",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "smallShipRepairDroneUnit",
            ticker: "RDS",
            weight: 50f32,
            volume: 10f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "REA",
        StaticMaterialInfo {
            material_id: "dd806abbca77956b09bc043c1633f51e",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "chemicalReagents",
            ticker: "REA",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "RED",
        StaticMaterialInfo {
            material_id: "0700a1ac4bfbbb10fbe08f9a3c08bc13",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "rescueDrone",
            ticker: "RED",
            weight: 0.3f32,
            volume: 0.05f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "REP",
        StaticMaterialInfo {
            material_id: "7171933481f022ca74200b216f98ca60",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "settlerLuxuryTools",
            ticker: "REP",
            weight: 0.006f32,
            volume: 0.002f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "RG",
        StaticMaterialInfo {
            material_id: "11205b2106f47b28c189a82494733841",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "reinforcedTranslucentMaterial",
            ticker: "RG",
            weight: 0.032f32,
            volume: 0.01f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "RGO",
        StaticMaterialInfo {
            material_id: "184e8b2fa39edeeb942a838c7502bcc1",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "goldCopperAlloy",
            ticker: "RGO",
            weight: 19.32f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "RHP",
        StaticMaterialInfo {
            material_id: "e59f70623448014f8fac5af9922de5c3",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "reinforcedHullPlate",
            ticker: "RHP",
            weight: 12f32,
            volume: 10f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "ROM",
        StaticMaterialInfo {
            material_id: "23b9ee06710318a6d3e9eec4d0b58cb7",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "nonVolatileMemory",
            ticker: "ROM",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "RSE",
        StaticMaterialInfo {
            material_id: "0fd977ac87d641a5bfcadea33e24641c",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "reinforcedStructuralElements",
            ticker: "RSE",
            weight: 1.9f32,
            volume: 0.7f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "RSH",
        StaticMaterialInfo {
            material_id: "3b32f1da809a1cd9083a22f7aa0c8af2",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "radiationShielding",
            ticker: "RSH",
            weight: 3.7f32,
            volume: 0.8f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "RSI",
        StaticMaterialInfo {
            material_id: "81dabbc84c110d1214ac8e090592ad31",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "silkRaw",
            ticker: "RSI",
            weight: 1.1f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "RTA",
        StaticMaterialInfo {
            material_id: "a24540c6c24595ca5bf47f091f11c8f9",
            category_name: "construction prefabs",
            category_id: "ef423f673d9e8c82043b4c5c63f6b55e",
            name: "reinforcedWindow",
            ticker: "RTA",
            weight: 1.5f32,
            volume: 0.5f32,
            category: MaterialCategory::ConstructionPrefabs,
        },
    );
    map.insert(
        "S",
        StaticMaterialInfo {
            material_id: "2f2863037da2f5b88e13609b2cc21e3b",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "sulfur",
            ticker: "S",
            weight: 0.52f32,
            volume: 0.25f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "SA",
        StaticMaterialInfo {
            material_id: "cebf6ae4dbea53789660ed6b9a4a5e86",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "searchAlgorithm",
            ticker: "SA",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "SAL",
        StaticMaterialInfo {
            material_id: "460c114b10d44a8ac99f7a0a47910f42",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "sortingAlgorithm",
            ticker: "SAL",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "SAR",
        StaticMaterialInfo {
            material_id: "5c9630043c3f2d66206977ae394d8cf0",
            category_name: "electronic devices",
            category_id: "19d621bb3f297c0425e34cdf0b138ece",
            name: "sensorArray",
            ticker: "SAR",
            weight: 1.7f32,
            volume: 2f32,
            category: MaterialCategory::ElectronicDevices,
        },
    );
    map.insert(
        "SC",
        StaticMaterialInfo {
            material_id: "5952bfbf043018679334e7ea71b1504e",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "technicianLuxuryHealth",
            ticker: "SC",
            weight: 0.04f32,
            volume: 0.01f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "SCB",
        StaticMaterialInfo {
            material_id: "c006dedb53cf3bb8cb964c8151d9a744",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "smallCargoBay",
            ticker: "SCB",
            weight: 50f32,
            volume: 50f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "SCN",
        StaticMaterialInfo {
            material_id: "66369803dbe73bee8a78105d3ca00554",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "technicianTools",
            ticker: "SCN",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "SCR",
        StaticMaterialInfo {
            material_id: "a4a8a25d01f80586f7bad3488fa11f36",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "sulfurCrystals",
            ticker: "SCR",
            weight: 2.05f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "SDR",
        StaticMaterialInfo {
            material_id: "663f2e01b48535883100f1e16fc75f45",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "surgicalDrone",
            ticker: "SDR",
            weight: 0.3f32,
            volume: 0.05f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "SEA",
        StaticMaterialInfo {
            material_id: "04970fd8cd9c3bc423b46503985e55de",
            category_name: "construction materials",
            category_id: "156bbcce730fba6169338d560f05fd26",
            name: "sealant",
            ticker: "SEA",
            weight: 0.15f32,
            volume: 0.07f32,
            category: MaterialCategory::ConstructionMaterials,
        },
    );
    map.insert(
        "SEN",
        StaticMaterialInfo {
            material_id: "cbb28f4ebfa4e7b26f09d5b585608e3d",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "sensor",
            ticker: "SEN",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "SEQ",
        StaticMaterialInfo {
            material_id: "4094d1cf6704085fbe6ff7a6e1862e4a",
            category_name: "medical equipment",
            category_id: "ef05c6b81cf109f142dd415ce6d36e67",
            name: "surgicalEquipment",
            ticker: "SEQ",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::MedicalEquipment,
        },
    );
    map.insert(
        "SF",
        StaticMaterialInfo {
            material_id: "7756a0779efe67f0f293f41eda4944d4",
            category_name: "fuels",
            category_id: "ba98fa0cf77040a96cd8a608ad0d08e9",
            name: "stlFuel",
            ticker: "SF",
            weight: 0.06f32,
            volume: 0.06f32,
            category: MaterialCategory::Fuels,
        },
    );
    map.insert(
        "SFE",
        StaticMaterialInfo {
            material_id: "bbfa7555fbfa8ef2852d4479f7aa703e",
            category_name: "ship engines",
            category_id: "16b943147bb16fe84c0d62de8a4316a7",
            name: "smallEmitter",
            ticker: "SFE",
            weight: 0.1f32,
            volume: 0.4f32,
            category: MaterialCategory::ShipEngines,
        },
    );
    map.insert(
        "SFK",
        StaticMaterialInfo {
            material_id: "24cb7e11e07449e2a6966b92d4ee307d",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "fastenerKitSmall",
            ticker: "SFK",
            weight: 0.04f32,
            volume: 0.02f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "SFL",
        StaticMaterialInfo {
            material_id: "e8b8b2f9a1e27ff69fe0c15b25a1d424",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "smallFtlTank",
            ticker: "SFL",
            weight: 9f32,
            volume: 1.5f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "SI",
        StaticMaterialInfo {
            material_id: "69e09abe369419b53675148d64164b3a",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "silicon",
            ticker: "SI",
            weight: 2.329f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "SIL",
        StaticMaterialInfo {
            material_id: "8302200bd88502383e932ae257bcb9b4",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "silkProcessed",
            ticker: "SIL",
            weight: 1.21f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "SIO",
        StaticMaterialInfo {
            material_id: "eeff5ee913d34354be84177847fbfe48",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "siliconOre",
            ticker: "SIO",
            weight: 1.79f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "SNM",
        StaticMaterialInfo {
            material_id: "0247f5b671722c53f2bb4cb83af68f2b",
            category_name: "software systems",
            category_id: "9e8bc2a092cc3361b6949eca3e95921a",
            name: "universeMap",
            ticker: "SNM",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareSystems,
        },
    );
    map.insert(
        "SOI",
        StaticMaterialInfo {
            material_id: "93fb7db0a58375534661afd0dc88f4ad",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "artificialSoil",
            ticker: "SOI",
            weight: 0.9f32,
            volume: 1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "SOL",
        StaticMaterialInfo {
            material_id: "bda47fe5bfb1769235f9447569b0d369",
            category_name: "energy systems",
            category_id: "0509ed0a396f5a004d24ef08e33786fb",
            name: "solarCell",
            ticker: "SOL",
            weight: 0.015f32,
            volume: 0.01f32,
            category: MaterialCategory::EnergySystems,
        },
    );
    map.insert(
        "SP",
        StaticMaterialInfo {
            material_id: "4602969555899f2a4955e3b08d2fbdb6",
            category_name: "energy systems",
            category_id: "0509ed0a396f5a004d24ef08e33786fb",
            name: "solarPanel",
            ticker: "SP",
            weight: 0.15f32,
            volume: 0.1f32,
            category: MaterialCategory::EnergySystems,
        },
    );
    map.insert(
        "SRD",
        StaticMaterialInfo {
            material_id: "9c418f805a867af115600ac3657cce0d",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "shipRepairDrone",
            ticker: "SRD",
            weight: 0.3f32,
            volume: 0.05f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "SRP",
        StaticMaterialInfo {
            material_id: "9e783e064cfecd477c04cd25f0cf0389",
            category_name: "ship shields",
            category_id: "a8e32e7e16219394989aea62fc0cd943",
            name: "specializedRadiationShielding",
            ticker: "SRP",
            weight: 0.1f32,
            volume: 0.2f32,
            category: MaterialCategory::ShipShields,
        },
    );
    map.insert(
        "SSC",
        StaticMaterialInfo {
            material_id: "b700f9670131981590e77238d7c207d3",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "structuralSpacecraftComponent",
            ticker: "SSC",
            weight: 1f32,
            volume: 1f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "SSL",
        StaticMaterialInfo {
            material_id: "251192d17a5b0e770cd3681383a495fd",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "smallStlTank",
            ticker: "SSL",
            weight: 20f32,
            volume: 20f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "STL",
        StaticMaterialInfo {
            material_id: "aaef669fed967abcfc188df59e1c4f4b",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "steel",
            ticker: "STL",
            weight: 7.85f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "STR",
        StaticMaterialInfo {
            material_id: "3ca4ad9d9fa814530aa9ba66b74c1d06",
            category_name: "medical equipment",
            category_id: "ef05c6b81cf109f142dd415ce6d36e67",
            name: "medicalStretcher",
            ticker: "STR",
            weight: 0.01f32,
            volume: 1f32,
            category: MaterialCategory::MedicalEquipment,
        },
    );
    map.insert(
        "STS",
        StaticMaterialInfo {
            material_id: "83cadb15cbdc91cbdf616710b05afb34",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "stabilitySupportSystem",
            ticker: "STS",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "SU",
        StaticMaterialInfo {
            material_id: "04aac5d04eadadd5a34806f60334abc3",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "surgeryUnit",
            ticker: "SU",
            weight: 6f32,
            volume: 5f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "SUD",
        StaticMaterialInfo {
            material_id: "08f48a785d3012a7fcfa7e0c3e2c571c",
            category_name: "drones",
            category_id: "6eb09f14ed9fec3d0407114be8bc8882",
            name: "surveillanceDrone",
            ticker: "SUD",
            weight: 0.3f32,
            volume: 0.05f32,
            category: MaterialCategory::Drones,
        },
    );
    map.insert(
        "SUN",
        StaticMaterialInfo {
            material_id: "e8308b1283cb813e2962d64fdb42ec33",
            category_name: "utility",
            category_id: "4eb124000ee0e38af1daf5aced313823",
            name: "safetyUniform",
            ticker: "SUN",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::Utility,
        },
    );
    map.insert(
        "SWF",
        StaticMaterialInfo {
            material_id: "f1778b3b4e6643f6424e01ffb79e87a0",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "waferSmall",
            ticker: "SWF",
            weight: 0.003f32,
            volume: 0.003f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "TA",
        StaticMaterialInfo {
            material_id: "f2b6f7e3711dc54e44da33efec4aadb7",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "tantalum",
            ticker: "TA",
            weight: 16.65f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "TAC",
        StaticMaterialInfo {
            material_id: "346d2efba85c3e9c1f874fa6845e8c23",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "targetingComputer",
            ticker: "TAC",
            weight: 0.15f32,
            volume: 1f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "TAI",
        StaticMaterialInfo {
            material_id: "ea550373234a7c171de0f525922af5be",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "tantalite",
            ticker: "TAI",
            weight: 7.94f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "TC",
        StaticMaterialInfo {
            material_id: "fc7f4fd0177c86ab262a99c9bde5a0d0",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "technetium",
            ticker: "TC",
            weight: 11.8f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map.insert(
        "TCB",
        StaticMaterialInfo {
            material_id: "3aa3bc318e699090ecfbe327f044a219",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "tinyCargoBay",
            ticker: "TCB",
            weight: 20f32,
            volume: 20f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "TCL",
        StaticMaterialInfo {
            material_id: "97ba20adb4d05fdcf365048b6878eaa7",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "tclAcid",
            ticker: "TCL",
            weight: 0.09f32,
            volume: 0.1f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "TCO",
        StaticMaterialInfo {
            material_id: "a0f92faf8cf9487b04fda968030527bf",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "technetiumOxide",
            ticker: "TCO",
            weight: 9.8f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "TCS",
        StaticMaterialInfo {
            material_id: "ccfd9b3d2d89ab9876ef33f7e915294f",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "technetiumStabilizers",
            ticker: "TCS",
            weight: 3.4f32,
            volume: 1.2f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "TCU",
        StaticMaterialInfo {
            material_id: "7923f6f2e04ed63a27fed11aa350a48b",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "traumaCareUnit",
            ticker: "TCU",
            weight: 5f32,
            volume: 4f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "THF",
        StaticMaterialInfo {
            material_id: "580c70e310bcd1543a46116d6be612e5",
            category_name: "chemicals",
            category_id: "9b32ea2333917ee2632ba7ae7675643f",
            name: "thermoFluid",
            ticker: "THF",
            weight: 0.6f32,
            volume: 0.35f32,
            category: MaterialCategory::Chemicals,
        },
    );
    map.insert(
        "THP",
        StaticMaterialInfo {
            material_id: "683145c220ccb5ed327326d7c4e26c6d",
            category_name: "ship parts",
            category_id: "d23b9225010ad7978e5496e55359e19b",
            name: "basicThermalProtectionMaterial",
            ticker: "THP",
            weight: 2.2f32,
            volume: 1f32,
            category: MaterialCategory::ShipParts,
        },
    );
    map.insert(
        "TI",
        StaticMaterialInfo {
            material_id: "59dc3f23a82660bb9816d4a28775bd17",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "titanium",
            ticker: "TI",
            weight: 4.5f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "TIO",
        StaticMaterialInfo {
            material_id: "4e66cc6157f132ba934a79ae38776521",
            category_name: "ores",
            category_id: "4c0430b40fbf7fa175ed4a5a13682c6f",
            name: "titaniumOre",
            ticker: "TIO",
            weight: 1.58f32,
            volume: 1f32,
            category: MaterialCategory::Ores,
        },
    );
    map.insert(
        "TK",
        StaticMaterialInfo {
            material_id: "563f37e86831a8361e79fa62ebe059eb",
            category_name: "textiles",
            category_id: "7028d7d532ccd4c6c878b069e7bbb634",
            name: "technoKevlar",
            ticker: "TK",
            weight: 1.89f32,
            volume: 1f32,
            category: MaterialCategory::Textiles,
        },
    );
    map.insert(
        "TPU",
        StaticMaterialInfo {
            material_id: "7cc9acecdf3e9565c9b4cbe9c96f0ab5",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "tensorProcessingUnit",
            ticker: "TPU",
            weight: 0.002f32,
            volume: 0.002f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "TRA",
        StaticMaterialInfo {
            material_id: "b805d86e129e9cf885ac92fabc6aa59f",
            category_name: "electronic parts",
            category_id: "8e15000dfbda2f46ce73458deaeeb358",
            name: "audioTransmitter",
            ticker: "TRA",
            weight: 0.005f32,
            volume: 0.002f32,
            category: MaterialCategory::ElectronicParts,
        },
    );
    map.insert(
        "TRN",
        StaticMaterialInfo {
            material_id: "1c9b53bea13393865b625d8310bd2ced",
            category_name: "electronic pieces",
            category_id: "5e022d8f199de1c70f8de4dfd2fdef64",
            name: "transistor",
            ticker: "TRN",
            weight: 0.001f32,
            volume: 0.001f32,
            category: MaterialCategory::ElectronicPieces,
        },
    );
    map.insert(
        "TRU",
        StaticMaterialInfo {
            material_id: "ec53c7423b75ca0d868f9580ae921561",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "truss",
            ticker: "TRU",
            weight: 0.1f32,
            volume: 1.5f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "TS",
        StaticMaterialInfo {
            material_id: "a2e3d2f078583c564476f8d4062c2d1a",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "tectosilisite",
            ticker: "TS",
            weight: 2.4f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "TSH",
        StaticMaterialInfo {
            material_id: "56afb81e665cf39883cdffbb26f81bf5",
            category_name: "construction parts",
            category_id: "f8aebc7dd84ce14fae131aa0203d4498",
            name: "thermalShielding",
            ticker: "TSH",
            weight: 2.4f32,
            volume: 1.5f32,
            category: MaterialCategory::ConstructionParts,
        },
    );
    map.insert(
        "TUB",
        StaticMaterialInfo {
            material_id: "9a1ae6a2b5bd58d6ea3ec91c9499cfc4",
            category_name: "medical equipment",
            category_id: "ef05c6b81cf109f142dd415ce6d36e67",
            name: "testTubes",
            ticker: "TUB",
            weight: 0.002f32,
            volume: 0.002f32,
            category: MaterialCategory::MedicalEquipment,
        },
    );
    map.insert(
        "UTS",
        StaticMaterialInfo {
            material_id: "22b3e4f9114304fb15a093e2b8f22a91",
            category_name: "utility",
            category_id: "4eb124000ee0e38af1daf5aced313823",
            name: "universalToolset",
            ticker: "UTS",
            weight: 0.05f32,
            volume: 0.05f32,
            category: MaterialCategory::Utility,
        },
    );
    map.insert(
        "VCB",
        StaticMaterialInfo {
            material_id: "4f44983ac27bcbac1f0fd4af7b5a2b7b",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "highVolumeCargoBay",
            ticker: "VCB",
            weight: 200f32,
            volume: 200f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "VEG",
        StaticMaterialInfo {
            material_id: "cc2dcb3699225b81cf63bf3e044a4f05",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "fattyVegetables",
            ticker: "VEG",
            weight: 1.1f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "VG",
        StaticMaterialInfo {
            material_id: "9c80515790fcd5fefc84d56c80f4bb2a",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "engineerLuxuryHealth",
            ticker: "VG",
            weight: 0.21f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "VIT",
        StaticMaterialInfo {
            material_id: "44424f913bab7837217786c8543bf44e",
            category_name: "agricultural products",
            category_id: "510e7751c0eb6a51fd0fa73565365d1c",
            name: "vitaEssence",
            ticker: "VIT",
            weight: 0.9f32,
            volume: 1f32,
            category: MaterialCategory::AgriculturalProducts,
        },
    );
    map.insert(
        "VSC",
        StaticMaterialInfo {
            material_id: "6fb814bde2cab570e5d8717251b96c00",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "verySmallCargoBay",
            ticker: "VSC",
            weight: 35f32,
            volume: 35f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "W",
        StaticMaterialInfo {
            material_id: "96b43016ff0f9872d92a35196045c07d",
            category_name: "metals",
            category_id: "cae48680d4c4e6f98d593a8c3778cb56",
            name: "wolfram",
            ticker: "W",
            weight: 7.519f32,
            volume: 1f32,
            category: MaterialCategory::Metals,
        },
    );
    map.insert(
        "WAI",
        StaticMaterialInfo {
            material_id: "4b78d4a814b42073673aecab2e7c5dbc",
            category_name: "software systems",
            category_id: "9e8bc2a092cc3361b6949eca3e95921a",
            name: "weakArtificalIntelligence",
            ticker: "WAI",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareSystems,
        },
    );
    map.insert(
        "WAL",
        StaticMaterialInfo {
            material_id: "21da4b6b92096523c67d016f97e7cc09",
            category_name: "alloys",
            category_id: "37e95782d80aa1c1ded89f2c4b834dd0",
            name: "tungstenAluminiumAlloy",
            ticker: "WAL",
            weight: 6.25f32,
            volume: 1f32,
            category: MaterialCategory::Alloys,
        },
    );
    map.insert(
        "WCB",
        StaticMaterialInfo {
            material_id: "cc7eb7e114d1c7f0db547595d683850a",
            category_name: "ship kits",
            category_id: "b7102e43e58120a254cf161c226d4177",
            name: "highLoadCargoBay",
            ticker: "WCB",
            weight: 200f32,
            volume: 200f32,
            category: MaterialCategory::ShipKits,
        },
    );
    map.insert(
        "WIN",
        StaticMaterialInfo {
            material_id: "9030ca612b7a761bc458240461fc8057",
            category_name: "consumables (luxury)",
            category_id: "8a0bd8b6a329c3d632da9da63c818b3d",
            name: "scientistLuxuryDrink",
            ticker: "WIN",
            weight: 0.1f32,
            volume: 0.1f32,
            category: MaterialCategory::ConsumablesLuxury,
        },
    );
    map.insert(
        "WM",
        StaticMaterialInfo {
            material_id: "a64beff205f211baec5a6d782cd6ec87",
            category_name: "software components",
            category_id: "ebec0c297d5b499a84dd051fa6f7c8ec",
            name: "windowManager",
            ticker: "WM",
            weight: 0.001f32,
            volume: 0.01f32,
            category: MaterialCategory::SoftwareComponents,
        },
    );
    map.insert(
        "WOR",
        StaticMaterialInfo {
            material_id: "34c4ff00c83e6b8f247f9b3948f7fe1a",
            category_name: "unit prefabs",
            category_id: "2c6ca18d24e03fc9250d8ca7d577568b",
            name: "handcraftWorkshopUnit",
            ticker: "WOR",
            weight: 5f32,
            volume: 5f32,
            category: MaterialCategory::UnitPrefabs,
        },
    );
    map.insert(
        "WR",
        StaticMaterialInfo {
            material_id: "1e7c5ae5b1518398b98e2ffdb28c0934",
            category_name: "electronic systems",
            category_id: "1e1a3a579c46d5f9c4e9959650c6b7a3",
            name: "waterRecycler",
            ticker: "WR",
            weight: 6.4f32,
            volume: 5f32,
            category: MaterialCategory::ElectronicSystems,
        },
    );
    map.insert(
        "WS",
        StaticMaterialInfo {
            material_id: "a4eeeb1b4c85b0637cfa7ca0a47c78d6",
            category_name: "consumables (basic)",
            category_id: "3f047ec3043bdd795fd7272d6be98799",
            name: "scientistTools",
            ticker: "WS",
            weight: 0.05f32,
            volume: 0.5f32,
            category: MaterialCategory::ConsumablesBasic,
        },
    );
    map.insert(
        "ZIR",
        StaticMaterialInfo {
            material_id: "82583e13472538b113667abc82421bda",
            category_name: "minerals",
            category_id: "79f3917448cc4f6034ece865c41aea34",
            name: "zircon",
            ticker: "ZIR",
            weight: 4.85f32,
            volume: 1f32,
            category: MaterialCategory::Minerals,
        },
    );
    map.insert(
        "ZR",
        StaticMaterialInfo {
            material_id: "e2a7a3a35c5e390f42db1faa468459dd",
            category_name: "elements",
            category_id: "ef1d03e7ef924395c2173fc4e4ee8a6f",
            name: "zirconium",
            ticker: "ZR",
            weight: 6.53f32,
            volume: 1f32,
            category: MaterialCategory::Elements,
        },
    );
    map
}

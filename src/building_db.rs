use crate::materials::MaterialCategory;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
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
    pub building_cost: &'static [(&'static str, u32)],
}
static DB: once_cell::sync::OnceCell<HashMap<&'static str, StaticBuildingInfo>> = OnceCell::new();
pub fn get_building_db() -> &'static HashMap<&'static str, StaticBuildingInfo> {
    DB.get_or_init(|| construct_building_db())
}
pub fn construct_building_db() -> HashMap<&'static str, StaticBuildingInfo> {
    let mut map = HashMap::new();
    let x = StaticBuildingInfo {
        name: "planetaryProjectComfortBig",
        ticker: "4DA",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("LIT", 2u32),
            ("LDE", 42u32),
            ("LTA", 24u32),
            ("TRU", 40u32),
            ("ADS", 1u32),
            ("FUN", 10u32),
            ("MCG", 600u32),
            ("BMF", 2u32),
            ("LBH", 42u32),
            ("LSE", 42u32),
        ],
    };
    map.insert("4DA", x);
    map.insert("planetaryProjectComfortBig", x);
    let x = StaticBuildingInfo {
        name: "advancedAppliancesFactory",
        ticker: "AAF",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 40u32,
        scientists: 20u32,
        area_cost: 50u32,
        building_cost: &[
            ("TRU", 8u32),
            ("ADE", 8u32),
            ("ABH", 6u32),
            ("ASE", 8u32),
            ("ATA", 1u32),
        ],
    };
    map.insert("AAF", x);
    map.insert("advancedAppliancesFactory", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectComfortCulture",
        ticker: "ACA",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("MCG", 300u32),
            ("TRU", 20u32),
            ("DEC", 32u32),
            ("RTA", 12u32),
            ("RDE", 24u32),
            ("RBH", 16u32),
            ("RSE", 24u32),
        ],
    };
    map.insert("ACA", x);
    map.insert("planetaryProjectComfortCulture", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectAdminCenter",
        ticker: "ADM",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 400u32,
        building_cost: &[
            ("BMF", 2u32),
            ("LSE", 25u32),
            ("BWS", 10u32),
            ("LBH", 16u32),
            ("RTA", 5u32),
            ("LDE", 32u32),
            ("MCG", 750u32),
        ],
    };
    map.insert("ADM", x);
    map.insert("planetaryProjectAdminCenter", x);
    let x = StaticBuildingInfo {
        name: "advancedMaterialLab",
        ticker: "AML",
        expertise: Some("CHEMISTRY"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 20u32,
        engineers: 30u32,
        scientists: 0u32,
        area_cost: 45u32,
        building_cost: &[("ABH", 10u32), ("ASE", 6u32), ("TRU", 6u32), ("ADE", 6u32)],
    };
    map.insert("AML", x);
    map.insert("advancedMaterialLab", x);
    let x = StaticBuildingInfo {
        name: "appliancesFactory",
        ticker: "APF",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 60u32,
        engineers: 20u32,
        scientists: 0u32,
        area_cost: 40u32,
        building_cost: &[("ABH", 6u32), ("ASE", 10u32), ("TRU", 8u32), ("ADE", 10u32)],
    };
    map.insert("APF", x);
    map.insert("appliancesFactory", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectCultureSmall",
        ticker: "ART",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 500u32,
        building_cost: &[
            ("LTA", 16u32),
            ("DEC", 10u32),
            ("LSE", 32u32),
            ("LBH", 24u32),
            ("WOR", 2u32),
            ("LDE", 32u32),
            ("TRU", 20u32),
            ("MCG", 300u32),
        ],
    };
    map.insert("ART", x);
    map.insert("planetaryProjectCultureSmall", x);
    let x = StaticBuildingInfo {
        name: "advancedSmelter",
        ticker: "ASM",
        expertise: Some("METALLURGY"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 50u32,
        engineers: 20u32,
        scientists: 0u32,
        area_cost: 34u32,
        building_cost: &[
            ("LTA", 2u32),
            ("LSE", 6u32),
            ("LDE", 2u32),
            ("LBH", 4u32),
            ("TRU", 4u32),
        ],
    };
    map.insert("ASM", x);
    map.insert("advancedSmelter", x);
    let x = StaticBuildingInfo {
        name: "basicMaterialsPlant",
        ticker: "BMP",
        expertise: Some("MANUFACTURING"),
        pioneers: 100u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 12u32,
        building_cost: &[("BSE", 6u32), ("BBH", 4u32), ("BDE", 2u32)],
    };
    map.insert("BMP", x);
    map.insert("basicMaterialsPlant", x);
    let x = StaticBuildingInfo {
        name: "corporationProjectFTLLaboratory",
        ticker: "CFL",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 100u32,
        building_cost: &[
            ("RTA", 32u32),
            ("RSE", 16u32),
            ("RDE", 24u32),
            ("RBH", 24u32),
            ("TAC", 6u32),
            ("FFC", 1u32),
            ("SP", 32u32),
        ],
    };
    map.insert("CFL", x);
    map.insert("corporationProjectFTLLaboratory", x);
    let x = StaticBuildingInfo {
        name: "chemPlant",
        ticker: "CHP",
        expertise: Some("CHEMISTRY"),
        pioneers: 20u32,
        settlers: 60u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 18u32,
        building_cost: &[("BSE", 3u32), ("BDE", 3u32), ("BBH", 3u32), ("TRU", 4u32)],
    };
    map.insert("CHP", x);
    map.insert("chemPlant", x);
    let x = StaticBuildingInfo {
        name: "corporationProjectImmortality",
        ticker: "CIM",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 100u32,
        building_cost: &[
            ("SP", 36u32),
            ("RTA", 16u32),
            ("RSE", 16u32),
            ("RDE", 16u32),
            ("LIS", 1u32),
            ("RBH", 32u32),
            ("CRU", 4u32),
        ],
    };
    map.insert("CIM", x);
    map.insert("corporationProjectImmortality", x);
    let x = StaticBuildingInfo {
        name: "clothingFactory",
        ticker: "CLF",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 40u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 37u32,
        building_cost: &[("BSE", 2u32), ("LDE", 4u32), ("LSE", 2u32), ("TRU", 8u32)],
    };
    map.insert("CLF", x);
    map.insert("clothingFactory", x);
    let x = StaticBuildingInfo {
        name: "cleanRoom",
        ticker: "CLR",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 25u32,
        technicians: 15u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 27u32,
        building_cost: &[
            ("LSE", 4u32),
            ("LDE", 4u32),
            ("TRU", 4u32),
            ("LTA", 4u32),
            ("LBH", 6u32),
        ],
    };
    map.insert("CLR", x);
    map.insert("cleanRoom", x);
    let x = StaticBuildingInfo {
        name: "coreModule",
        ticker: "CM",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[
            ("LDE", 4u32),
            ("TRU", 8u32),
            ("LTA", 4u32),
            ("PSL", 12u32),
            ("LSE", 4u32),
        ],
    };
    map.insert("CM", x);
    map.insert("coreModule", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectCogc",
        ticker: "COG",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("LSE", 24u32),
            ("LTA", 32u32),
            ("LBH", 32u32),
            ("BMF", 1u32),
            ("BWS", 16u32),
            ("LDE", 16u32),
            ("SP", 32u32),
        ],
    };
    map.insert("COG", x);
    map.insert("planetaryProjectCogc", x);
    let x = StaticBuildingInfo {
        name: "collector",
        ticker: "COL",
        expertise: Some("RESOURCE_EXTRACTION"),
        pioneers: 50u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 15u32,
        building_cost: &[("BSE", 16u32)],
    };
    map.insert("COL", x);
    map.insert("collector", x);
    let x = StaticBuildingInfo {
        name: "corporationProjectHeadquarters",
        ticker: "COR",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 100u32,
        building_cost: &[
            ("RDE", 32u32),
            ("SP", 24u32),
            ("RSE", 32u32),
            ("RBH", 16u32),
            ("RTA", 8u32),
            ("BMF", 2u32),
            ("BWS", 8u32),
        ],
    };
    map.insert("COR", x);
    map.insert("corporationProjectHeadquarters", x);
    let x = StaticBuildingInfo {
        name: "corporationProjectRepresentationCenter",
        ticker: "CRC",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 100u32,
        building_cost: &[
            ("LIT", 40u32),
            ("DEC", 80u32),
            ("LOG", 1u32),
            ("ATA", 100u32),
            ("TRU", 400u32),
            ("ABH", 100u32),
            ("ADE", 100u32),
            ("ADS", 1u32),
            ("ASE", 100u32),
            ("SP", 200u32),
            ("CBL", 4u32),
        ],
    };
    map.insert("CRC", x);
    map.insert("corporationProjectRepresentationCenter", x);
    let x = StaticBuildingInfo {
        name: "corporationProjectTerraforming",
        ticker: "CTE",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 100u32,
        building_cost: &[
            ("LIS", 1u32),
            ("SP", 44u32),
            ("RTA", 8u32),
            ("RDE", 32u32),
            ("RSE", 32u32),
            ("RBH", 32u32),
            ("CC", 5u32),
        ],
    };
    map.insert("CTE", x);
    map.insert("corporationProjectTerraforming", x);
    let x = StaticBuildingInfo {
        name: "droneShop",
        ticker: "DRS",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 40u32,
        engineers: 20u32,
        scientists: 0u32,
        area_cost: 30u32,
        building_cost: &[
            ("TRU", 6u32),
            ("RSE", 6u32),
            ("RTA", 2u32),
            ("RBH", 4u32),
            ("RDE", 6u32),
        ],
    };
    map.insert("DRS", x);
    map.insert("droneShop", x);
    let x = StaticBuildingInfo {
        name: "energyComponentAssembly",
        ticker: "ECA",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 20u32,
        technicians: 20u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 35u32,
        building_cost: &[
            ("RTA", 2u32),
            ("RSE", 6u32),
            ("RDE", 6u32),
            ("TRU", 6u32),
            ("RBH", 4u32),
        ],
    };
    map.insert("ECA", x);
    map.insert("energyComponentAssembly", x);
    let x = StaticBuildingInfo {
        name: "electronicDeviceManufactory",
        ticker: "EDM",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 50u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 30u32,
        building_cost: &[("LDE", 4u32), ("TRU", 4u32), ("LBH", 4u32), ("LSE", 6u32)],
    };
    map.insert("EDM", x);
    map.insert("electronicDeviceManufactory", x);
    let x = StaticBuildingInfo {
        name: "einsteiniumEnrichmentPlant",
        ticker: "EEP",
        expertise: Some("CHEMISTRY"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 60u32,
        scientists: 40u32,
        area_cost: 100u32,
        building_cost: &[
            ("RTA", 2u32),
            ("RDE", 6u32),
            ("RBH", 6u32),
            ("RSE", 12u32),
            ("TRU", 6u32),
        ],
    };
    map.insert("EEP", x);
    map.insert("einsteiniumEnrichmentPlant", x);
    let x = StaticBuildingInfo {
        name: "electronicsPlant",
        ticker: "ELP",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 40u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 35u32,
        building_cost: &[("RDE", 4u32), ("TRU", 6u32), ("RSE", 6u32), ("RBH", 4u32)],
    };
    map.insert("ELP", x);
    map.insert("electronicsPlant", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectSafetyHealth",
        ticker: "EMC",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("LDE", 32u32),
            ("TCU", 1u32),
            ("DOU", 1u32),
            ("BTA", 16u32),
            ("LSE", 24u32),
            ("LBH", 32u32),
            ("MCG", 300u32),
        ],
    };
    map.insert("EMC", x);
    map.insert("planetaryProjectSafetyHealth", x);
    let x = StaticBuildingInfo {
        name: "extractor",
        ticker: "EXT",
        expertise: Some("RESOURCE_EXTRACTION"),
        pioneers: 60u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[("BSE", 16u32)],
    };
    map.insert("EXT", x);
    map.insert("extractor", x);
    let x = StaticBuildingInfo {
        name: "fermentationFacility",
        ticker: "FER",
        expertise: Some("FOOD_INDUSTRIES"),
        pioneers: 0u32,
        settlers: 60u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[("LDE", 2u32), ("LBH", 2u32), ("LSE", 2u32), ("TRU", 5u32)],
    };
    map.insert("FER", x);
    map.insert("fermentationFacility", x);
    let x = StaticBuildingInfo {
        name: "foodProcessor",
        ticker: "FP",
        expertise: Some("FOOD_INDUSTRIES"),
        pioneers: 40u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 12u32,
        building_cost: &[("BBH", 3u32), ("BDE", 3u32), ("BSE", 3u32)],
    };
    map.insert("FP", x);
    map.insert("foodProcessor", x);
    let x = StaticBuildingInfo {
        name: "farm",
        ticker: "FRM",
        expertise: Some("AGRICULTURE"),
        pioneers: 50u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 30u32,
        building_cost: &[("BSE", 4u32), ("BBH", 4u32)],
    };
    map.insert("FRM", x);
    map.insert("farm", x);
    let x = StaticBuildingInfo {
        name: "fineSmithy",
        ticker: "FS",
        expertise: Some("METALLURGY"),
        pioneers: 0u32,
        settlers: 50u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[("BBH", 2u32), ("TRU", 4u32), ("LDE", 2u32), ("LBH", 2u32)],
    };
    map.insert("FS", x);
    map.insert("fineSmithy", x);
    let x = StaticBuildingInfo {
        name: "glassFurnace",
        ticker: "GF",
        expertise: Some("METALLURGY"),
        pioneers: 0u32,
        settlers: 80u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 27u32,
        building_cost: &[("LBH", 4u32), ("TRU", 5u32), ("LSE", 6u32)],
    };
    map.insert("GF", x);
    map.insert("glassFurnace", x);
    let x = StaticBuildingInfo {
        name: "habitationPioneer",
        ticker: "HB1",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 10u32,
        building_cost: &[("BTA", 1u32), ("BDE", 2u32), ("BBH", 4u32), ("BSE", 2u32)],
    };
    map.insert("HB1", x);
    map.insert("habitationPioneer", x);
    let x = StaticBuildingInfo {
        name: "habitationSettler",
        ticker: "HB2",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 12u32,
        building_cost: &[
            ("BSE", 2u32),
            ("TRU", 2u32),
            ("BDE", 2u32),
            ("BBH", 2u32),
            ("BTA", 2u32),
        ],
    };
    map.insert("HB2", x);
    map.insert("habitationSettler", x);
    let x = StaticBuildingInfo {
        name: "habitationTechnician",
        ticker: "HB3",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 14u32,
        building_cost: &[("LBH", 4u32), ("LTA", 8u32), ("LDE", 4u32), ("LSE", 4u32)],
    };
    map.insert("HB3", x);
    map.insert("habitationTechnician", x);
    let x = StaticBuildingInfo {
        name: "habitationEngineer",
        ticker: "HB4",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 16u32,
        building_cost: &[("RDE", 6u32), ("RBH", 6u32), ("RSE", 4u32), ("RTA", 4u32)],
    };
    map.insert("HB4", x);
    map.insert("habitationEngineer", x);
    let x = StaticBuildingInfo {
        name: "habitationScientist",
        ticker: "HB5",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 18u32,
        building_cost: &[("ATA", 4u32), ("ABH", 6u32), ("ADE", 6u32), ("ASE", 4u32)],
    };
    map.insert("HB5", x);
    map.insert("habitationScientist", x);
    let x = StaticBuildingInfo {
        name: "habitationBarracks",
        ticker: "HBB",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 14u32,
        building_cost: &[("LSE", 2u32), ("BBH", 4u32), ("BDE", 4u32), ("LTA", 2u32)],
    };
    map.insert("HBB", x);
    map.insert("habitationBarracks", x);
    let x = StaticBuildingInfo {
        name: "habitationCommune",
        ticker: "HBC",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 17u32,
        building_cost: &[
            ("RTA", 4u32),
            ("LDE", 2u32),
            ("LBH", 4u32),
            ("TRU", 2u32),
            ("RSE", 2u32),
        ],
    };
    map.insert("HBC", x);
    map.insert("habitationCommune", x);
    let x = StaticBuildingInfo {
        name: "habitationLuxury",
        ticker: "HBL",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 22u32,
        building_cost: &[("ATA", 6u32), ("ABH", 8u32), ("ADE", 8u32), ("ASE", 6u32)],
    };
    map.insert("HBL", x);
    map.insert("habitationLuxury", x);
    let x = StaticBuildingInfo {
        name: "habitationManagers",
        ticker: "HBM",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 20u32,
        building_cost: &[("ASE", 6u32), ("RBH", 8u32), ("RDE", 8u32), ("ATA", 4u32)],
    };
    map.insert("HBM", x);
    map.insert("habitationManagers", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectHealthBig",
        ticker: "HOS",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("RTA", 16u32),
            ("TRU", 16u32),
            ("MCG", 400u32),
            ("RDE", 20u32),
            ("SU", 1u32),
            ("RSE", 20u32),
            ("DOU", 1u32),
            ("RBH", 24u32),
            ("TCU", 1u32),
        ],
    };
    map.insert("HOS", x);
    map.insert("planetaryProjectHealthBig", x);
    let x = StaticBuildingInfo {
        name: "hullWeldingPlant",
        ticker: "HWP",
        expertise: Some("METALLURGY"),
        pioneers: 0u32,
        settlers: 40u32,
        technicians: 10u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[
            ("TRU", 4u32),
            ("BDE", 6u32),
            ("LTA", 2u32),
            ("BSE", 6u32),
            ("BBH", 4u32),
        ],
    };
    map.insert("HWP", x);
    map.insert("hullWeldingPlant", x);
    let x = StaticBuildingInfo {
        name: "hydroponicsFarm",
        ticker: "HYF",
        expertise: Some("AGRICULTURE"),
        pioneers: 40u32,
        settlers: 20u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 15u32,
        building_cost: &[("BSE", 2u32), ("MHL", 16u32), ("TRU", 4u32), ("LBH", 4u32)],
    };
    map.insert("HYF", x);
    map.insert("hydroponicsFarm", x);
    let x = StaticBuildingInfo {
        name: "incinerator",
        ticker: "INC",
        expertise: Some("RESOURCE_EXTRACTION"),
        pioneers: 40u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 10u32,
        building_cost: &[("BSE", 4u32), ("BDE", 2u32), ("BBH", 3u32), ("BTA", 1u32)],
    };
    map.insert("INC", x);
    map.insert("incinerator", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectHealthSmall",
        ticker: "INF",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 500u32,
        building_cost: &[
            ("MCG", 300u32),
            ("BSE", 16u32),
            ("BDE", 24u32),
            ("BBH", 24u32),
            ("BTA", 12u32),
            ("TRU", 10u32),
        ],
    };
    map.insert("INF", x);
    map.insert("planetaryProjectHealthSmall", x);
    let x = StaticBuildingInfo {
        name: "inVitroPlant",
        ticker: "IVP",
        expertise: Some("FOOD_INDUSTRIES"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 70u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 32u32,
        building_cost: &[
            ("TRU", 6u32),
            ("RDE", 4u32),
            ("RTA", 2u32),
            ("RBH", 6u32),
            ("RSE", 4u32),
        ],
    };
    map.insert("IVP", x);
    map.insert("inVitroPlant", x);
    let x = StaticBuildingInfo {
        name: "laboratory",
        ticker: "LAB",
        expertise: Some("CHEMISTRY"),
        pioneers: 0u32,
        settlers: 20u32,
        technicians: 70u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[("LDE", 12u32), ("TRU", 8u32), ("LSE", 6u32)],
    };
    map.insert("LAB", x);
    map.insert("laboratory", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectEducationSmall",
        ticker: "LIB",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 500u32,
        building_cost: &[
            ("ABH", 8u32),
            ("ASE", 12u32),
            ("ATA", 6u32),
            ("ADE", 12u32),
            ("MCG", 250u32),
            ("TRU", 20u32),
            ("LOG", 1u32),
            ("COM", 1u32),
        ],
    };
    map.insert("LIB", x);
    map.insert("planetaryProjectEducationSmall", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectLocalMarket",
        ticker: "LM",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 500u32,
        building_cost: &[
            ("BDE", 8u32),
            ("LBH", 8u32),
            ("BTA", 8u32),
            ("TRU", 10u32),
            ("BSE", 12u32),
        ],
    };
    map.insert("LM", x);
    map.insert("planetaryProjectLocalMarket", x);
    let x = StaticBuildingInfo {
        name: "mediumComponentsAssembly",
        ticker: "MCA",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 20u32,
        technicians: 20u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 35u32,
        building_cost: &[("RDE", 6u32), ("RBH", 4u32), ("TRU", 6u32), ("RSE", 4u32)],
    };
    map.insert("MCA", x);
    map.insert("mediumComponentsAssembly", x);
    let x = StaticBuildingInfo {
        name: "orchard",
        ticker: "ORC",
        expertise: Some("AGRICULTURE"),
        pioneers: 0u32,
        settlers: 70u32,
        technicians: 10u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 120u32,
        building_cost: &[("TRU", 2u32), ("ASE", 8u32), ("ATA", 4u32)],
    };
    map.insert("ORC", x);
    map.insert("orchard", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectComfortSmall",
        ticker: "PAR",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 500u32,
        building_cost: &[
            ("BTA", 10u32),
            ("SOI", 100u32),
            ("BSE", 20u32),
            ("MCG", 300u32),
            ("HAB", 5u32),
            ("BDE", 16u32),
            ("BBH", 16u32),
        ],
    };
    map.insert("PAR", x);
    map.insert("planetaryProjectComfortSmall", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectCultureEducation",
        ticker: "PBH",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("TRU", 30u32),
            ("RSE", 24u32),
            ("MCG", 300u32),
            ("RTA", 12u32),
            ("ADS", 1u32),
            ("RDE", 24u32),
            ("COM", 1u32),
            ("RBH", 16u32),
        ],
    };
    map.insert("PBH", x);
    map.insert("planetaryProjectCultureEducation", x);
    let x = StaticBuildingInfo {
        name: "pharmaFactory",
        ticker: "PHF",
        expertise: Some("CHEMISTRY"),
        pioneers: 0u32,
        settlers: 25u32,
        technicians: 25u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 35u32,
        building_cost: &[
            ("LSE", 8u32),
            ("LTA", 2u32),
            ("LBH", 4u32),
            ("LDE", 6u32),
            ("TRU", 6u32),
        ],
    };
    map.insert("PHF", x);
    map.insert("pharmaFactory", x);
    let x = StaticBuildingInfo {
        name: "polymerPlant",
        ticker: "POL",
        expertise: Some("CHEMISTRY"),
        pioneers: 10u32,
        settlers: 25u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 15u32,
        building_cost: &[("BDE", 4u32), ("BSE", 4u32), ("TRU", 2u32), ("LBH", 8u32)],
    };
    map.insert("POL", x);
    map.insert("polymerPlant", x);
    let x = StaticBuildingInfo {
        name: "prefabPlant1",
        ticker: "PP1",
        expertise: Some("CONSTRUCTION"),
        pioneers: 80u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 19u32,
        building_cost: &[("BDE", 3u32), ("BSE", 4u32), ("BBH", 3u32)],
    };
    map.insert("PP1", x);
    map.insert("prefabPlant1", x);
    let x = StaticBuildingInfo {
        name: "prefabPlant2",
        ticker: "PP2",
        expertise: Some("CONSTRUCTION"),
        pioneers: 25u32,
        settlers: 25u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[("BSE", 3u32), ("TRU", 4u32), ("BDE", 6u32), ("BBH", 6u32)],
    };
    map.insert("PP2", x);
    map.insert("prefabPlant2", x);
    let x = StaticBuildingInfo {
        name: "prefabPlant3",
        ticker: "PP3",
        expertise: Some("CONSTRUCTION"),
        pioneers: 0u32,
        settlers: 20u32,
        technicians: 40u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 32u32,
        building_cost: &[("TRU", 6u32), ("LDE", 6u32), ("LSE", 4u32)],
    };
    map.insert("PP3", x);
    map.insert("prefabPlant3", x);
    let x = StaticBuildingInfo {
        name: "prefabPlant4",
        ticker: "PP4",
        expertise: Some("CONSTRUCTION"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 40u32,
        engineers: 30u32,
        scientists: 0u32,
        area_cost: 40u32,
        building_cost: &[("RBH", 6u32), ("RSE", 8u32), ("RDE", 6u32), ("TRU", 4u32)],
    };
    map.insert("PP4", x);
    map.insert("prefabPlant4", x);
    let x = StaticBuildingInfo {
        name: "plasticsPrinterFacility",
        ticker: "PPF",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 50u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 16u32,
        building_cost: &[("LBH", 4u32), ("TRU", 2u32), ("LDE", 2u32), ("BSE", 2u32)],
    };
    map.insert("PPF", x);
    map.insert("plasticsPrinterFacility", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectShipyard",
        ticker: "PSY",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("ATA", 8u32),
            ("ADE", 24u32),
            ("ABH", 32u32),
            ("ASE", 24u32),
            ("TRU", 24u32),
        ],
    };
    map.insert("PSY", x);
    map.insert("planetaryProjectShipyard", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectWarehouse",
        ticker: "PWH",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("MCG", 300u32),
            ("BBH", 24u32),
            ("BDE", 24u32),
            ("BSE", 12u32),
            ("TRU", 20u32),
        ],
    };
    map.insert("PWH", x);
    map.insert("planetaryProjectWarehouse", x);
    let x = StaticBuildingInfo {
        name: "refinery",
        ticker: "REF",
        expertise: Some("FUEL_REFINING"),
        pioneers: 60u32,
        settlers: 20u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 25u32,
        building_cost: &[("BBH", 6u32), ("BDE", 6u32), ("BSE", 6u32)],
    };
    map.insert("REF", x);
    map.insert("refinery", x);
    let x = StaticBuildingInfo {
        name: "rig",
        ticker: "RIG",
        expertise: Some("RESOURCE_EXTRACTION"),
        pioneers: 30u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 10u32,
        building_cost: &[("BSE", 12u32)],
    };
    map.insert("RIG", x);
    map.insert("rig", x);
    let x = StaticBuildingInfo {
        name: "smallComponentsAssembly",
        ticker: "SCA",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 30u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 35u32,
        building_cost: &[("TRU", 4u32), ("RDE", 6u32), ("RSE", 2u32), ("RBH", 3u32)],
    };
    map.insert("SCA", x);
    map.insert("smallComponentsAssembly", x);
    let x = StaticBuildingInfo {
        name: "softwareDevelopment",
        ticker: "SD",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 50u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 20u32,
        building_cost: &[
            ("LDE", 6u32),
            ("LSE", 10u32),
            ("TRU", 8u32),
            ("LBH", 6u32),
            ("LTA", 4u32),
        ],
    };
    map.insert("SD", x);
    map.insert("softwareDevelopment", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectSafetyBig",
        ticker: "SDP",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("RTA", 8u32),
            ("DOU", 1u32),
            ("RDE", 16u32),
            ("RSE", 10u32),
            ("MCG", 300u32),
            ("TRU", 8u32),
            ("ADS", 1u32),
            ("BMF", 1u32),
            ("RBH", 16u32),
        ],
    };
    map.insert("SDP", x);
    map.insert("planetaryProjectSafetyBig", x);
    let x = StaticBuildingInfo {
        name: "softwareEngineering",
        ticker: "SE",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 30u32,
        scientists: 0u32,
        area_cost: 20u32,
        building_cost: &[
            ("TRU", 4u32),
            ("RSE", 6u32),
            ("RBH", 4u32),
            ("RDE", 6u32),
            ("RTA", 2u32),
        ],
    };
    map.insert("SE", x);
    map.insert("softwareEngineering", x);
    let x = StaticBuildingInfo {
        name: "shipKitFactory",
        ticker: "SKF",
        expertise: Some("METALLURGY"),
        pioneers: 0u32,
        settlers: 40u32,
        technicians: 10u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 40u32,
        building_cost: &[
            ("BBH", 4u32),
            ("BDE", 6u32),
            ("LTA", 2u32),
            ("TRU", 6u32),
            ("BSE", 4u32),
        ],
    };
    map.insert("SKF", x);
    map.insert("shipKitFactory", x);
    let x = StaticBuildingInfo {
        name: "softwareLabs",
        ticker: "SL",
        expertise: Some("ELECTRONICS"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 20u32,
        area_cost: 20u32,
        building_cost: &[
            ("RSE", 12u32),
            ("TRU", 8u32),
            ("LTA", 6u32),
            ("LDE", 8u32),
            ("RBH", 12u32),
        ],
    };
    map.insert("SL", x);
    map.insert("softwareLabs", x);
    let x = StaticBuildingInfo {
        name: "smelter",
        ticker: "SME",
        expertise: Some("METALLURGY"),
        pioneers: 50u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 17u32,
        building_cost: &[("BDE", 4u32), ("BBH", 4u32), ("BSE", 6u32)],
    };
    map.insert("SME", x);
    map.insert("smelter", x);
    let x = StaticBuildingInfo {
        name: "spacecraftPropulsionFactory",
        ticker: "SPF",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 50u32,
        scientists: 20u32,
        area_cost: 40u32,
        building_cost: &[
            ("RBH", 8u32),
            ("TRU", 6u32),
            ("RDE", 6u32),
            ("RSE", 6u32),
            ("RTA", 2u32),
        ],
    };
    map.insert("SPF", x);
    map.insert("spacecraftPropulsionFactory", x);
    let x = StaticBuildingInfo {
        name: "spacecraftPrefabPlant",
        ticker: "SPP",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 50u32,
        engineers: 30u32,
        scientists: 0u32,
        area_cost: 60u32,
        building_cost: &[
            ("RTA", 2u32),
            ("RBH", 12u32),
            ("RSE", 6u32),
            ("RDE", 6u32),
            ("TRU", 8u32),
        ],
    };
    map.insert("SPP", x);
    map.insert("spacecraftPrefabPlant", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectSafetySmall",
        ticker: "SST",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 500u32,
        building_cost: &[
            ("BTA", 6u32),
            ("BBH", 24u32),
            ("TRU", 10u32),
            ("MCG", 300u32),
            ("BSE", 12u32),
            ("BDE", 24u32),
        ],
    };
    map.insert("SST", x);
    map.insert("planetaryProjectSafetySmall", x);
    let x = StaticBuildingInfo {
        name: "storageFacility",
        ticker: "STO",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 15u32,
        building_cost: &[("BDE", 6u32), ("BBH", 6u32), ("BSE", 2u32)],
    };
    map.insert("STO", x);
    map.insert("storageFacility", x);
    let x = StaticBuildingInfo {
        name: "technetiumProcessing",
        ticker: "TNP",
        expertise: Some("CHEMISTRY"),
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 80u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 30u32,
        building_cost: &[("LSE", 12u32), ("TRU", 8u32), ("LBH", 6u32), ("LDE", 8u32)],
    };
    map.insert("TNP", x);
    map.insert("technetiumProcessing", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectEducationBig",
        ticker: "UNI",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("ADE", 16u32),
            ("ABH", 12u32),
            ("MCG", 400u32),
            ("TRU", 30u32),
            ("ATA", 12u32),
            ("ASE", 16u32),
            ("LOG", 1u32),
            ("LU", 4u32),
            ("BMF", 2u32),
        ],
    };
    map.insert("UNI", x);
    map.insert("planetaryProjectEducationBig", x);
    let x = StaticBuildingInfo {
        name: "unitPrefabPlant",
        ticker: "UPF",
        expertise: Some("CONSTRUCTION"),
        pioneers: 20u32,
        settlers: 50u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 50u32,
        building_cost: &[
            ("TRU", 8u32),
            ("BSE", 10u32),
            ("BTA", 3u32),
            ("BBH", 8u32),
            ("BDE", 10u32),
        ],
    };
    map.insert("UPF", x);
    map.insert("unitPrefabPlant", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectCultureBig",
        ticker: "VRT",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("DEC", 20u32),
            ("RDE", 32u32),
            ("RBH", 24u32),
            ("RSE", 32u32),
            ("ADS", 1u32),
            ("MCG", 500u32),
            ("TRU", 30u32),
            ("RTA", 12u32),
        ],
    };
    map.insert("VRT", x);
    map.insert("planetaryProjectCultureBig", x);
    let x = StaticBuildingInfo {
        name: "planetaryProjectHealthComfort",
        ticker: "WCE",
        expertise: None,
        pioneers: 0u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 1000u32,
        building_cost: &[
            ("MCG", 500u32),
            ("TRU", 16u32),
            ("LTA", 24u32),
            ("LSE", 32u32),
            ("FLO", 12u32),
            ("LDE", 36u32),
            ("LBH", 36u32),
            ("DEC", 40u32),
        ],
    };
    map.insert("WCE", x);
    map.insert("planetaryProjectHealthComfort", x);
    let x = StaticBuildingInfo {
        name: "weldingPlant",
        ticker: "WEL",
        expertise: Some("CONSTRUCTION"),
        pioneers: 70u32,
        settlers: 0u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 19u32,
        building_cost: &[("BBH", 8u32), ("BSE", 6u32)],
    };
    map.insert("WEL", x);
    map.insert("weldingPlant", x);
    let x = StaticBuildingInfo {
        name: "weavingPlant",
        ticker: "WPL",
        expertise: Some("MANUFACTURING"),
        pioneers: 0u32,
        settlers: 70u32,
        technicians: 0u32,
        engineers: 0u32,
        scientists: 0u32,
        area_cost: 40u32,
        building_cost: &[("LBH", 6u32), ("TRU", 6u32), ("BDE", 2u32), ("LSE", 3u32)],
    };
    map.insert("WPL", x);
    map.insert("weavingPlant", x);
    map
}

use std::fmt::{self, Display};

use crate::material_db::get_material_db;

pub struct MaterialWithColor {
    name: String,
    bg_color: (u8, u8, u8),
    fg_color: (u8, u8, u8),
    amount: Option<i32>,
}

impl MaterialWithColor {
    pub fn new(name: &str) -> Self {
        let material_info = get_material_db().get(name).unwrap();
        let bg_color = material_info.category.get_bg_color();
        let fg_color = material_info.category.get_fg_color();
        Self {
            name: name.into(),
            bg_color,
            fg_color,
            amount: None,
        }
    }
    pub fn with_amount(mut self, amount: i32) -> Self {
        self.amount = Some(amount);
        self
    }
}

impl Display for MaterialWithColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        let output = if let Some(amount) = self.amount {
            format!("{} {}", amount, self.name)
                .truecolor(self.fg_color.0, self.fg_color.1, self.fg_color.2)
                .on_truecolor(self.bg_color.0, self.bg_color.1, self.bg_color.2)
        } else {
            self.name
                .truecolor(self.fg_color.0, self.fg_color.1, self.fg_color.2)
                .on_truecolor(self.bg_color.0, self.bg_color.1, self.bg_color.2)
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub enum MaterialCategory {
    // background: linear-gradient(135deg, rgb(92, 18, 18), rgb(117, 43, 43)); color: rgb(219, 145, 145);
    AgriculturalProducts,

    // background: linear-gradient(135deg, rgb(123, 76, 30), rgb(148, 101, 55)); color: rgb(250, 203, 157);
    Alloys,

    // background: linear-gradient(135deg, rgb(183, 46, 91), rgb(208, 71, 116)); color: rgb(255, 173, 218);
    Chemicals,

    // background: linear-gradient(135deg, rgb(24, 91, 211), rgb(49, 116, 236)); color: rgb(151, 218, 255);
    ConstructionMaterials,

    // background: linear-gradient(135deg, rgb(41, 77, 107), rgb(66, 102, 132)); color: rgb(168, 204, 234);
    ConstructionParts,

    // background: linear-gradient(135deg, rgb(15, 30, 98), rgb(40, 55, 123)); color: rgb(142, 157, 225);
    ConstructionPrefabs,

    // background: linear-gradient(135deg, rgb(149, 46, 46), rgb(174, 71, 71)); color: rgb(255, 173, 173);
    ConsumablesBasic,

    // background: linear-gradient(135deg, rgb(136, 24, 39), rgb(161, 49, 64)); color: rgb(255, 151, 166);
    ConsumablesLuxury,

    // background: linear-gradient(135deg, rgb(62, 10, 17), rgb(87, 35, 42)); color: rgb(189, 137, 144);
    ConsumableBundle,

    // background: linear-gradient(135deg, rgb(140, 52, 18), rgb(165, 77, 43)); color: rgb(255, 179, 145);
    Drones,

    // background: linear-gradient(135deg, rgb(86, 20, 147), rgb(111, 45, 172)); color: rgb(213, 147, 255);
    ElectronicDevices,

    // background: linear-gradient(135deg, rgb(91, 46, 183), rgb(116, 71, 208)); color: rgb(218, 173, 255);
    ElectronicParts,

    // background: linear-gradient(135deg, rgb(119, 82, 189), rgb(144, 107, 214)); color: rgb(246, 209, 255);
    ElectronicPieces,

    // background: linear-gradient(135deg, rgb(51, 26, 76), rgb(76, 51, 101)); color: rgb(178, 153, 203);
    ElectronicSystems,

    // background: linear-gradient(135deg, rgb(61, 46, 32), rgb(86, 71, 57)); color: rgb(188, 173, 159);
    Elements,

    // background: linear-gradient(135deg, rgb(21, 62, 39), rgb(46, 87, 64)); color: rgb(148, 189, 166);
    EnergySystems,

    // background: linear-gradient(135deg, rgb(30, 123, 30), rgb(55, 148, 55)); color: rgb(157, 250, 157);
    Fuels,

    // background: linear-gradient(135deg, rgb(0, 105, 107), rgb(25, 130, 132)); color: rgb(127, 232, 234);
    Gases,

    // background: linear-gradient(135deg, rgb(114, 164, 202), rgb(139, 189, 227)); color: rgb(241, 255, 255);
    Liquids,

    // background: linear-gradient(135deg, rgb(85, 170, 85), rgb(110, 195, 110)); color: rgb(212, 255, 212);
    MedicalEquipment,

    // background: linear-gradient(135deg, rgb(54, 54, 54), rgb(79, 79, 79)); color: rgb(181, 181, 181);
    Metals,

    // background: linear-gradient(135deg, rgb(153, 113, 73), rgb(178, 138, 98)); color: rgb(255, 240, 200);
    Minerals,

    // background: linear-gradient(135deg, rgb(82, 87, 97), rgb(107, 112, 122)); color: rgb(209, 214, 224);
    Ores,
    // background: linear-gradient(135deg, rgb(121, 31, 60), rgb(146, 56, 85)); color: rgb(248, 158, 187);
    Plastics,

    // background: linear-gradient(135deg, rgb(153, 41, 0), rgb(178, 66, 25)); color: rgb(255, 168, 127);
    ShipEngines,

    // background: linear-gradient(135deg, rgb(153, 84, 0), rgb(178, 109, 25)); color: rgb(255, 211, 127);
    ShipKits,

    // background: linear-gradient(135deg, rgb(153, 99, 0), rgb(178, 124, 25)); color: rgb(255, 226, 127);
    ShipParts,

    // background: linear-gradient(135deg, rgb(224, 131, 0), rgb(249, 156, 25)); color: rgb(255, 255, 127);
    ShipShields,

    // background: linear-gradient(135deg, rgb(136, 121, 47), rgb(161, 146, 72)); color: rgb(255, 248, 174);
    SoftwareComponents,

    // background: linear-gradient(135deg, rgb(60, 53, 5), rgb(85, 78, 30)); color: rgb(187, 180, 132);
    SoftwareSystems,

    // background: linear-gradient(135deg, rgb(129, 98, 19), rgb(154, 123, 44));
    SoftwareTools,

    // background: linear-gradient(135deg, rgb(82, 90, 33), rgb(107, 115, 58)); color: rgb(209, 217, 160);
    Textiles,

    // background: linear-gradient(135deg, rgb(29, 27, 28), rgb(54, 52, 53)); color: rgb(156, 154, 155);
    UnitPrefabs,

    // background: linear-gradient(135deg, rgb(161, 148, 136), rgb(186, 173, 161)); color: rgb(255, 255, 255);
    Utility,
}

impl MaterialCategory {
    pub fn from_name(name: &str) -> Option<MaterialCategory> {
        match name {
            "alloys" => Some(MaterialCategory::Alloys),
            "agricultural products" => Some(MaterialCategory::AgriculturalProducts),
            "chemicals" => Some(MaterialCategory::Chemicals),
            "construction materials" => Some(MaterialCategory::ConstructionMaterials),
            "construction parts" => Some(MaterialCategory::ConstructionParts),
            "construction prefabs" => Some(MaterialCategory::ConstructionPrefabs),
            "consumables (basic)" => Some(MaterialCategory::ConsumablesBasic),
            "consumables (luxury)" => Some(MaterialCategory::ConsumablesLuxury),
            "consumable bundles" => Some(MaterialCategory::ConsumableBundle),
            "drones" => Some(MaterialCategory::Drones),
            "electronic devices" => Some(MaterialCategory::ElectronicDevices),
            "electronic parts" => Some(MaterialCategory::ElectronicParts),
            "electronic pieces" => Some(MaterialCategory::ElectronicPieces),
            "electronic systems" => Some(MaterialCategory::ElectronicSystems),
            "elements" => Some(MaterialCategory::Elements),
            "energy systems" => Some(MaterialCategory::EnergySystems),
            "fuels" => Some(MaterialCategory::Fuels),
            "gases" => Some(MaterialCategory::Gases),
            "liquids" => Some(MaterialCategory::Liquids),
            "medical equipment" => Some(MaterialCategory::MedicalEquipment),
            "metals" => Some(MaterialCategory::Metals),
            "minerals" => Some(MaterialCategory::Minerals),
            "ores" => Some(MaterialCategory::Ores),
            "plastics" => Some(MaterialCategory::Plastics),
            "ship engines" => Some(MaterialCategory::ShipEngines),
            "ship kits" => Some(MaterialCategory::ShipKits),
            "ship parts" => Some(MaterialCategory::ShipParts),
            "ship shields" => Some(MaterialCategory::ShipShields),
            "software components" => Some(MaterialCategory::SoftwareComponents),
            "software systems" => Some(MaterialCategory::SoftwareSystems),
            "software tools" => Some(MaterialCategory::SoftwareTools),
            "textiles" => Some(MaterialCategory::Textiles),
            "unit prefabs" => Some(MaterialCategory::UnitPrefabs),
            "utility" => Some(MaterialCategory::Utility),
            _ => None,
        }
    }

    pub fn get_bg_color(&self) -> (u8, u8, u8) {
        match self {
            MaterialCategory::ConstructionPrefabs => average_rgb((15, 30, 98), (40, 55, 123)),
            MaterialCategory::ConsumablesBasic => average_rgb((149, 46, 46), (174, 71, 71)),
            MaterialCategory::ConsumablesLuxury => average_rgb((136, 24, 39), (161, 49, 64)),
            MaterialCategory::ConsumableBundle => average_rgb((62, 10, 17), (87, 35, 42)),
            MaterialCategory::Metals => average_rgb((54, 54, 54), (79, 79, 79)),
            MaterialCategory::Gases => average_rgb((0, 105, 107), (25, 130, 132)),
            MaterialCategory::Fuels => average_rgb((30, 123, 30), (55, 148, 55)),
            MaterialCategory::Ores => average_rgb((82, 87, 97), (107, 112, 122)),
            MaterialCategory::Elements => average_rgb((61, 46, 32), (86, 71, 57)),
            MaterialCategory::Textiles => average_rgb((82, 90, 33), (107, 115, 58)),
            MaterialCategory::ElectronicPieces => average_rgb((119, 82, 189), (144, 107, 214)),
            MaterialCategory::SoftwareComponents => average_rgb((136, 121, 47), (161, 146, 72)),
            MaterialCategory::Chemicals => average_rgb((183, 46, 91), (208, 71, 116)),
            MaterialCategory::ConstructionMaterials => average_rgb((24, 91, 211), (49, 116, 236)),
            MaterialCategory::ShipParts => average_rgb((153, 99, 0), (178, 124, 25)),
            MaterialCategory::AgriculturalProducts => average_rgb((92, 18, 18), (117, 43, 43)),
            MaterialCategory::Minerals => average_rgb((153, 113, 73), (178, 138, 98)),
            MaterialCategory::Utility => average_rgb((161, 148, 136), (186, 173, 161)),
            MaterialCategory::Liquids => average_rgb((114, 164, 202), (139, 189, 227)),
            MaterialCategory::ConstructionParts => average_rgb((41, 77, 107), (66, 102, 132)),
            MaterialCategory::Plastics => average_rgb((121, 31, 60), (146, 56, 85)),
            MaterialCategory::EnergySystems => average_rgb((21, 62, 39), (46, 87, 64)),
            MaterialCategory::Alloys => average_rgb((123, 76, 30), (148, 101, 55)),
            MaterialCategory::Drones => average_rgb((140, 52, 18), (165, 77, 43)),
            MaterialCategory::ElectronicDevices => average_rgb((86, 20, 147), (111, 45, 172)),
            MaterialCategory::ElectronicParts => average_rgb((91, 46, 183), (116, 71, 208)),
            MaterialCategory::ElectronicSystems => average_rgb((51, 26, 76), (76, 51, 101)),
            MaterialCategory::MedicalEquipment => average_rgb((85, 170, 85), (110, 195, 110)),
            MaterialCategory::ShipEngines => average_rgb((153, 41, 0), (178, 66, 25)),
            MaterialCategory::ShipKits => average_rgb((153, 84, 0), (178, 109, 25)),
            MaterialCategory::ShipShields => average_rgb((224, 131, 0), (249, 156, 25)),
            MaterialCategory::SoftwareSystems => average_rgb((60, 53, 5), (85, 78, 30)),
            MaterialCategory::SoftwareTools => average_rgb((129, 98, 19), (154, 123, 44)),
            MaterialCategory::UnitPrefabs => average_rgb((29, 27, 28), (54, 52, 53)),
        }
    }

    pub fn get_fg_color(&self) -> (u8, u8, u8) {
        match self {
            MaterialCategory::AgriculturalProducts => (219, 145, 145),
            MaterialCategory::Alloys => (250, 203, 157),
            MaterialCategory::Chemicals => (255, 173, 218),
            MaterialCategory::ConstructionMaterials => (151, 218, 255),
            MaterialCategory::ConstructionParts => (168, 204, 234),
            MaterialCategory::ConstructionPrefabs => (142, 157, 225),
            MaterialCategory::ConsumablesBasic => (255, 173, 173),
            MaterialCategory::ConsumablesLuxury => (255, 151, 166),
            MaterialCategory::ConsumableBundle => (189, 137, 144),
            MaterialCategory::Drones => (255, 179, 145),
            MaterialCategory::ElectronicDevices => (213, 147, 255),
            MaterialCategory::ElectronicParts => (218, 173, 255),
            MaterialCategory::ElectronicPieces => (246, 209, 255),
            MaterialCategory::ElectronicSystems => (178, 153, 203),
            MaterialCategory::Elements => (188, 173, 159),
            MaterialCategory::EnergySystems => (148, 189, 166),
            MaterialCategory::Fuels => (157, 250, 157),
            MaterialCategory::Gases => (127, 232, 234),
            MaterialCategory::Liquids => (241, 255, 255),
            MaterialCategory::MedicalEquipment => (212, 255, 212),
            MaterialCategory::Metals => (181, 181, 181),
            MaterialCategory::Minerals => (255, 240, 200),
            MaterialCategory::Ores => (209, 214, 224),
            MaterialCategory::Plastics => (248, 158, 187),
            MaterialCategory::ShipEngines => (255, 168, 127),
            MaterialCategory::ShipKits => (255, 211, 127),
            MaterialCategory::ShipParts => (255, 226, 127),
            MaterialCategory::ShipShields => (255, 255, 127),
            MaterialCategory::SoftwareComponents => (255, 248, 174),
            MaterialCategory::SoftwareSystems => (187, 180, 132),
            MaterialCategory::SoftwareTools => (255, 226, 127),
            MaterialCategory::Textiles => (209, 217, 160),
            MaterialCategory::UnitPrefabs => (156, 154, 155),
            MaterialCategory::Utility => (255, 255, 255),
        }
    }
}

fn average_rgb(a: (u8, u8, u8), b: (u8, u8, u8)) -> (u8, u8, u8) {
    let (r1, g1, b1) = a;
    let (r2, g2, b2) = b;

    // bummer this can't be a const function :(
    let r = ((r1 as f32 * r1 as f32 + r2 as f32 * r2 as f32) / 2.0)
        .sqrt()
        .round() as u8;
    let g = ((g1 as f32 * g1 as f32 + g2 as f32 * g2 as f32) / 2.0)
        .sqrt()
        .round() as u8;
    let b = ((b1 as f32 * b1 as f32 + b2 as f32 * b2 as f32) / 2.0)
        .sqrt()
        .round() as u8;

    (r, g, b)
}

#[derive(Debug, Copy, Clone)]
pub struct StaticMaterialInfo {
    pub material_id: &'static str,
    pub category_name: &'static str,
    pub category_id: &'static str,
    pub name: &'static str,
    pub ticker: &'static str,
    pub weight: f32,
    pub volume: f32,
}

// static MATERIAL_DB: OnceCell<HashMap<&'static str, &'static str>> = OnceCell::new();

// pub fn get_material_db() -> HashMap<&'static str, StaticMaterialInfo> {
//     std::
// }

#[test]
fn test() {
    let a = (15, 30, 98);
    let b = (40, 55, 123);
    let c = average_rgb(a, b);
    assert_eq!(c, (30, 44, 111));
}

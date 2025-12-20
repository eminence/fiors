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
        StaticRecipeInfo { building_ticker : "AAF", recipe_name :
        "1000xSPT 4xTRS 1xSST 2xTOR 1xSNM 10xSAR=>1xGWS", standard_recipe_name :
        "AAF:10xSAR-1xSNM-1000xSPT-1xSST-2xTOR-4xTRS=>1xGWS", duration :
        std::time::Duration::from_millis(691200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SAR", amount : 10u32, }, StaticRecipeMaterial { ticker : "SPT",
        amount : 1000u32, }, StaticRecipeMaterial { ticker : "TRS", amount : 4u32, },
        StaticRecipeMaterial { ticker : "TOR", amount : 2u32, }, StaticRecipeMaterial {
        ticker : "SST", amount : 1u32, }, StaticRecipeMaterial { ticker : "SNM", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "GWS", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "AAF", recipe_name :
        "1xSA 1xBMF 1xDA=>1xTAC", standard_recipe_name : "AAF:1xBMF-1xDA-1xSA=>1xTAC",
        duration : std::time::Duration::from_millis(207360000u64), inputs : &
        [StaticRecipeMaterial { ticker : "SA", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "BMF", amount : 1u32, }, StaticRecipeMaterial { ticker : "DA", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "TAC", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "AAF", recipe_name :
        "1xBMF 1xSNM=>1xNV2", standard_recipe_name : "AAF:1xBMF-1xSNM=>1xNV2", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SNM", amount : 1u32, }, StaticRecipeMaterial { ticker : "BMF", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "NV2", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "AAF", recipe_name :
        "1xCST 1xWAI 10xES 1xPCB 1xSAR 6xFLP 3xFC=>1xCRU", standard_recipe_name :
        "AAF:1xCST-10xES-3xFC-6xFLP-1xPCB-1xSAR-1xWAI=>1xCRU", duration :
        std::time::Duration::from_millis(190080000u64), inputs : & [StaticRecipeMaterial
        { ticker : "WAI", amount : 1u32, }, StaticRecipeMaterial { ticker : "CST", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "SAR", amount : 1u32, },
        StaticRecipeMaterial { ticker : "ES", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "PCB", amount : 1u32, }, StaticRecipeMaterial { ticker : "FLP", amount :
        6u32, }, StaticRecipeMaterial { ticker : "FC", amount : 3u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CRU", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "AAF", recipe_name : "1xPFG 1xSDM 200xWRH=>1xSST",
        standard_recipe_name : "AAF:1xPFG-1xSDM-200xWRH=>1xSST", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SDM", amount : 1u32, }, StaticRecipeMaterial { ticker : "PFG", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "WRH", amount : 200u32, }], outputs :
        & [StaticRecipeMaterial { ticker : "SST", amount : 1u32, }], }, StaticRecipeInfo
        { building_ticker : "AAF", recipe_name : "1xWAI 1xTPU=>1xSTS",
        standard_recipe_name : "AAF:1xTPU-1xWAI=>1xSTS", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "TPU", amount : 1u32, }, StaticRecipeMaterial { ticker : "WAI", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "STS", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "AAF", recipe_name :
        "2xFIR 2xRAG 2xCBM 1xFFC=>1xPFG", standard_recipe_name :
        "AAF:2xCBM-1xFFC-2xFIR-2xRAG=>1xPFG", duration :
        std::time::Duration::from_millis(518400000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FFC", amount : 1u32, }, StaticRecipeMaterial { ticker : "CBM", amount
        : 2u32, }, StaticRecipeMaterial { ticker : "RAG", amount : 2u32, },
        StaticRecipeMaterial { ticker : "FIR", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PFG", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "AAF", recipe_name :
        "1xWAI 1xPCB 10xSAR 100xGV 8xFC 20xTHF=>1xCC", standard_recipe_name :
        "AAF:8xFC-100xGV-1xPCB-10xSAR-20xTHF-1xWAI=>1xCC", duration :
        std::time::Duration::from_millis(207360000u64), inputs : & [StaticRecipeMaterial
        { ticker : "THF", amount : 20u32, }, StaticRecipeMaterial { ticker : "PCB",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "SAR", amount : 10u32, },
        StaticRecipeMaterial { ticker : "GV", amount : 100u32, }, StaticRecipeMaterial {
        ticker : "FC", amount : 8u32, }, StaticRecipeMaterial { ticker : "WAI", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CC", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "AML", recipe_name :
        "2xBER=>1xBE 1xAL 1xSIO", standard_recipe_name : "AML:2xBER=>1xAL-1xBE-1xSIO",
        duration : std::time::Duration::from_millis(8640000u64), inputs : &
        [StaticRecipeMaterial { ticker : "BER", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SIO", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "BE", amount :
        1u32, }], }, StaticRecipeInfo { building_ticker : "AML", recipe_name :
        "2xTAI=>1xTA 1xFE", standard_recipe_name : "AML:2xTAI=>1xFE-1xTA", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TAI", amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "TA", amount : 1u32, }, StaticRecipeMaterial { ticker : "FE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "AML", recipe_name : "2xZIR=>1xZR 2xSIO",
        standard_recipe_name : "AML:2xZIR=>2xSIO-1xZR", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ZIR", amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "SIO", amount : 2u32, }, StaticRecipeMaterial { ticker : "ZR", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "AML", recipe_name :
        "1xBOR 1xSI 4xAL=>4xBOS", standard_recipe_name : "AML:4xAL-1xBOR-1xSI=>4xBOS",
        duration : std::time::Duration::from_millis(43200000u64), inputs : &
        [StaticRecipeMaterial { ticker : "AL", amount : 4u32, }, StaticRecipeMaterial {
        ticker : "SI", amount : 1u32, }, StaticRecipeMaterial { ticker : "BOR", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BOS", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "AML", recipe_name : "5xBTS=>1xW",
        standard_recipe_name : "AML:5xBTS=>1xW", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BTS", amount : 5u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "W", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "APF",
        recipe_name : "1xBWS 1xAAR 4xRAD=>1xCOM", standard_recipe_name :
        "APF:1xAAR-1xBWS-4xRAD=>1xCOM", duration :
        std::time::Duration::from_millis(64800000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AAR", amount : 1u32, }, StaticRecipeMaterial { ticker : "BWS", amount :
        1u32, }, StaticRecipeMaterial { ticker : "RAD", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "COM", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "APF", recipe_name : "1xWR 1xAIR 1xBMF 1xWAI=>1xLIS",
        standard_recipe_name : "APF:1xAIR-1xBMF-1xWAI-1xWR=>1xLIS", duration :
        std::time::Duration::from_millis(216000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "WAI", amount : 1u32, }, StaticRecipeMaterial { ticker : "AIR", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "WR", amount : 1u32, },
        StaticRecipeMaterial { ticker : "BMF", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "LIS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "APF", recipe_name :
        "12xFLP 1xFC 1xAWF 1xPCB 1xSEN 1xWAI=>1xWR", standard_recipe_name :
        "APF:1xAWF-1xFC-12xFLP-1xPCB-1xSEN-1xWAI=>1xWR", duration :
        std::time::Duration::from_millis(146880000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FLP", amount : 12u32, }, StaticRecipeMaterial { ticker : "WAI",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "PCB", amount : 1u32, },
        StaticRecipeMaterial { ticker : "AWF", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "FC", amount : 1u32, }, StaticRecipeMaterial { ticker : "SEN", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "WR", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "APF", recipe_name :
        "1xHCP 1xNS 1xH2O 1xWAI 1xPCB 1xSAR 1xGV 1xFC 1xBAC=>1xAIR", standard_recipe_name
        : "APF:1xBAC-1xFC-1xGV-1xH2O-1xHCP-1xNS-1xPCB-1xSAR-1xWAI=>1xAIR", duration :
        std::time::Duration::from_millis(216000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "GV", amount : 1u32, }, StaticRecipeMaterial { ticker : "HCP", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "NS", amount : 1u32, },
        StaticRecipeMaterial { ticker : "SAR", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "H2O", amount : 1u32, }, StaticRecipeMaterial { ticker : "WAI", amount :
        1u32, }, StaticRecipeMaterial { ticker : "FC", amount : 1u32, },
        StaticRecipeMaterial { ticker : "PCB", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "BAC", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "AIR", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "APF",
        recipe_name : "5xW 1xWAI 1xDA 1xBMF=>1xFFC", standard_recipe_name :
        "APF:1xBMF-1xDA-5xW-1xWAI=>1xFFC", duration :
        std::time::Duration::from_millis(216000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "W", amount : 5u32, }, StaticRecipeMaterial { ticker : "BMF", amount :
        1u32, }, StaticRecipeMaterial { ticker : "DA", amount : 1u32, },
        StaticRecipeMaterial { ticker : "WAI", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FFC", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "APF", recipe_name : "1xBMF 1xLD 1xSAL=>1xLOG",
        standard_recipe_name : "APF:1xBMF-1xLD-1xSAL=>1xLOG", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LD", amount : 1u32, }, StaticRecipeMaterial { ticker : "SAL", amount :
        1u32, }, StaticRecipeMaterial { ticker : "BMF", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "LOG", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "APF", recipe_name : "1xBWS 1xOS 1xSAR 1xDA=>1xRCS",
        standard_recipe_name : "APF:1xBWS-1xDA-1xOS-1xSAR=>1xRCS", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SAR", amount : 1u32, }, StaticRecipeMaterial { ticker : "DA", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "BWS", amount : 1u32, },
        StaticRecipeMaterial { ticker : "OS", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RCS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "APF", recipe_name : "1xDA 1xBWS=>20xWS", standard_recipe_name
        : "APF:1xBWS-1xDA=>20xWS", duration :
        std::time::Duration::from_millis(155520000u64), inputs : & [StaticRecipeMaterial
        { ticker : "BWS", amount : 1u32, }, StaticRecipeMaterial { ticker : "DA", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "WS", amount : 20u32,
        }], }, StaticRecipeInfo { building_ticker : "APF", recipe_name :
        "1xBWS 8xTRA 1xOS=>1xADS", standard_recipe_name : "APF:1xBWS-1xOS-8xTRA=>1xADS",
        duration : std::time::Duration::from_millis(86400000u64), inputs : &
        [StaticRecipeMaterial { ticker : "TRA", amount : 8u32, }, StaticRecipeMaterial {
        ticker : "BWS", amount : 1u32, }, StaticRecipeMaterial { ticker : "OS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "ADS", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "APF", recipe_name :
        "1xWAI 1xSAR 1xFC 10xTHF=>1xACS", standard_recipe_name :
        "APF:1xFC-1xSAR-10xTHF-1xWAI=>1xACS", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "THF", amount : 10u32, }, StaticRecipeMaterial { ticker : "WAI", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "SAR", amount : 1u32, },
        StaticRecipeMaterial { ticker : "FC", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ACS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "APF", recipe_name : "1xWS 1xSNM=>1xNV1", standard_recipe_name
        : "APF:1xSNM-1xWS=>1xNV1", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SNM", amount : 1u32, }, StaticRecipeMaterial { ticker : "WS", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "NV1", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "APF", recipe_name :
        "10xTRS 1xSTS=>1xSDM", standard_recipe_name : "APF:1xSTS-10xTRS=>1xSDM", duration
        : std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial
        { ticker : "STS", amount : 1u32, }, StaticRecipeMaterial { ticker : "TRS", amount
        : 10u32, }], outputs : & [StaticRecipeMaterial { ticker : "SDM", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "APF", recipe_name :
        "2xHPC 1xLD=>32xPDA", standard_recipe_name : "APF:2xHPC-1xLD=>32xPDA", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HPC", amount : 2u32, }, StaticRecipeMaterial { ticker : "LD", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PDA", amount : 32u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name : "1xAL 3xTI=>4xAST",
        standard_recipe_name : "ASM:1xAL-3xTI=>4xAST", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TI", amount : 3u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "AST", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name : "5xRE 1xAL=>6xALR",
        standard_recipe_name : "ASM:1xAL-5xRE=>6xALR", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "RE", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "ALR", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name : "1xFE 3xTI=>4xFET",
        standard_recipe_name : "ASM:1xFE-3xTI=>4xFET", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }, StaticRecipeMaterial { ticker : "TI", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "FET", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name :
        "2xWAL 1xSI 1xO=>1xCTF", standard_recipe_name : "ASM:1xO-1xSI-2xWAL=>1xCTF",
        duration : std::time::Duration::from_millis(34560000u64), inputs : &
        [StaticRecipeMaterial { ticker : "O", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "WAL", amount : 2u32, }, StaticRecipeMaterial { ticker : "SI", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CTF", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name : "1xW 3xAL=>4xWAL",
        standard_recipe_name : "ASM:3xAL-1xW=>4xWAL", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "W", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "WAL", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name : "3xAL 3xFE=>6xFAL",
        standard_recipe_name : "ASM:3xAL-3xFE=>6xFAL", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 3u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "FAL", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "ASM", recipe_name : "4xRE 2xW=>6xWRH",
        standard_recipe_name : "ASM:4xRE-2xW=>6xWRH", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RE", amount : 4u32, }, StaticRecipeMaterial { ticker : "W", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "WRH", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "100xPE 25xPG=>20xOVE", standard_recipe_name : "BMP:100xPE-25xPG=>20xOVE",
        duration : std::time::Duration::from_millis(51840000u64), inputs : &
        [StaticRecipeMaterial { ticker : "PE", amount : 100u32, }, StaticRecipeMaterial {
        ticker : "PG", amount : 25u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "OVE", amount : 20u32, }], }, StaticRecipeInfo { building_ticker : "BMP",
        recipe_name : "20xOVE 10xC=>20xSUN", standard_recipe_name :
        "BMP:10xC-20xOVE=>20xSUN", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "OVE", amount : 20u32, }, StaticRecipeMaterial { ticker : "C", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "SUN", amount : 20u32,
        }], }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "1xMFK 1xSFK 10xINS=>4xREP", standard_recipe_name :
        "BMP:10xINS-1xMFK-1xSFK=>4xREP", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 1u32, }, StaticRecipeMaterial { ticker : "INS", amount :
        10u32, }, StaticRecipeMaterial { ticker : "SFK", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "REP", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "BMP", recipe_name : "10xOVE 1xAL 1xSWF=>10xEXO",
        standard_recipe_name : "BMP:1xAL-10xOVE-1xSWF=>10xEXO", duration :
        std::time::Duration::from_millis(12960000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SWF", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        1u32, }, StaticRecipeMaterial { ticker : "OVE", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "EXO", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "BMP", recipe_name : "10xOVE 1xAL=>10xEXO",
        standard_recipe_name : "BMP:1xAL-10xOVE=>10xEXO", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "OVE", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "EXO", amount : 10u32,
        }], }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "10xOVE 1xAL 1xMFK=>10xEXO", standard_recipe_name :
        "BMP:1xAL-1xMFK-10xOVE=>10xEXO", duration :
        std::time::Duration::from_millis(14688000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "MFK", amount :
        1u32, }, StaticRecipeMaterial { ticker : "OVE", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "EXO", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "BMP", recipe_name : "1xC 2xH=>200xPE", standard_recipe_name :
        "BMP:1xC-2xH=>200xPE", duration : std::time::Duration::from_millis(24192000u64),
        inputs : & [StaticRecipeMaterial { ticker : "C", amount : 1u32, },
        StaticRecipeMaterial { ticker : "H", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PE", amount : 200u32, }], }, StaticRecipeInfo {
        building_ticker : "BMP", recipe_name : "1xCOT 10xPG=>30xOVE",
        standard_recipe_name : "BMP:1xCOT-10xPG=>30xOVE", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "COT", amount : 1u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "OVE", amount : 30u32,
        }], }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "1xCOT 50xPG=>20xPWO", standard_recipe_name : "BMP:1xCOT-50xPG=>20xPWO", duration
        : std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PG", amount : 50u32, }, StaticRecipeMaterial { ticker : "COT", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PWO", amount : 20u32,
        }], }, StaticRecipeInfo { building_ticker : "BMP", recipe_name : "1xMGS=>6xMG",
        standard_recipe_name : "BMP:1xMGS=>6xMG", duration :
        std::time::Duration::from_millis(10368000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MGS", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "MG", amount : 6u32, }], }, StaticRecipeInfo { building_ticker : "BMP",
        recipe_name : "1xSTL 1xTRN=>7xPT", standard_recipe_name :
        "BMP:1xSTL-1xTRN=>7xPT", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TRN", amount : 1u32, }, StaticRecipeMaterial { ticker : "STL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PT", amount : 7u32, }],
        }, StaticRecipeInfo { building_ticker : "BMP", recipe_name : "1xSTL 1xW=>15xPT",
        standard_recipe_name : "BMP:1xSTL-1xW=>15xPT", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "STL", amount : 1u32, }, StaticRecipeMaterial { ticker : "W", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PT", amount : 15u32, }],
        }, StaticRecipeInfo { building_ticker : "BMP", recipe_name : "1xSTL=>5xPT",
        standard_recipe_name : "BMP:1xSTL=>5xPT", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "STL", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "PT", amount : 5u32, }], }, StaticRecipeInfo { building_ticker : "BMP",
        recipe_name : "20xPG 20xEPO=>50xOFF", standard_recipe_name :
        "BMP:20xEPO-20xPG=>50xOFF", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 20u32, }, StaticRecipeMaterial { ticker : "EPO", amount :
        20u32, }], outputs : & [StaticRecipeMaterial { ticker : "OFF", amount : 50u32,
        }], }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "1xSTL 2xSFK=>6xPT", standard_recipe_name : "BMP:2xSFK-1xSTL=>6xPT", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SFK", amount : 2u32, }, StaticRecipeMaterial { ticker : "STL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PT", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "1xS 1xSI 30xPG=>30xSEA", standard_recipe_name : "BMP:30xPG-1xS-1xSI=>30xSEA",
        duration : std::time::Duration::from_millis(25920000u64), inputs : &
        [StaticRecipeMaterial { ticker : "PG", amount : 30u32, }, StaticRecipeMaterial {
        ticker : "S", amount : 1u32, }, StaticRecipeMaterial { ticker : "SI", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "SEA", amount : 30u32, }],
        }, StaticRecipeInfo { building_ticker : "BMP", recipe_name : "3xCLI=>1xI",
        standard_recipe_name : "BMP:3xCLI=>1xI", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CLI", amount : 3u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "I", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "BMP",
        recipe_name : "50xPE 4xAL=>2xSTR", standard_recipe_name :
        "BMP:4xAL-50xPE=>2xSTR", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 50u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "STR", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "4xLST 2xSIO=>50xMCG", standard_recipe_name : "BMP:4xLST-2xSIO=>50xMCG", duration
        : std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SIO", amount : 2u32, }, StaticRecipeMaterial { ticker : "LST", amount
        : 4u32, }], outputs : & [StaticRecipeMaterial { ticker : "MCG", amount : 50u32,
        }], }, StaticRecipeInfo { building_ticker : "BMP", recipe_name :
        "1xRCO 50xPE=>20xOVE", standard_recipe_name : "BMP:50xPE-1xRCO=>20xOVE", duration
        : std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial
        { ticker : "RCO", amount : 1u32, }, StaticRecipeMaterial { ticker : "PE", amount
        : 50u32, }], outputs : & [StaticRecipeMaterial { ticker : "OVE", amount : 20u32,
        }], }, StaticRecipeInfo { building_ticker : "CHP", recipe_name :
        "10xAMM 4xREA=>10xPFE", standard_recipe_name : "CHP:10xAMM-4xREA=>10xPFE",
        duration : std::time::Duration::from_millis(69120000u64), inputs : &
        [StaticRecipeMaterial { ticker : "REA", amount : 4u32, }, StaticRecipeMaterial {
        ticker : "AMM", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "PFE", amount : 10u32, }], }, StaticRecipeInfo { building_ticker : "CHP",
        recipe_name : "6xVEG 10xREA=>6xOLF", standard_recipe_name :
        "CHP:10xREA-6xVEG=>6xOLF", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "VEG", amount : 6u32, }, StaticRecipeMaterial { ticker : "REA", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "OLF", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "CHP", recipe_name :
        "50xPE 1xAL 4xCOT 1xHER=>16xMED", standard_recipe_name :
        "CHP:1xAL-4xCOT-1xHER-50xPE=>16xMED", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 50u32, }, StaticRecipeMaterial { ticker : "HER", amount :
        1u32, }, StaticRecipeMaterial { ticker : "COT", amount : 4u32, },
        StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "MED", amount : 16u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "1xNA 1xBOR 5xH=>20xNAB",
        standard_recipe_name : "CHP:1xBOR-5xH-1xNA=>20xNAB", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H", amount : 5u32, }, StaticRecipeMaterial { ticker : "NA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "BOR", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "NAB", amount : 20u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "1xMG 1xCU 1xS=>1xIND",
        standard_recipe_name : "CHP:1xCU-1xMG-1xS=>1xIND", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MG", amount : 1u32, }, StaticRecipeMaterial { ticker : "S", amount :
        1u32, }, StaticRecipeMaterial { ticker : "CU", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "IND", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "3xHAL 1xH2O=>2xNA 1xCL",
        standard_recipe_name : "CHP:1xH2O-3xHAL=>1xCL-2xNA", duration :
        std::time::Duration::from_millis(50112000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HAL", amount : 3u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CL", amount : 1u32, },
        StaticRecipeMaterial { ticker : "NA", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "1xLST=>10xFLX", standard_recipe_name :
        "CHP:1xLST=>10xFLX", duration : std::time::Duration::from_millis(43200000u64),
        inputs : & [StaticRecipeMaterial { ticker : "LST", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FLX", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "1xLST=>1xCA", standard_recipe_name :
        "CHP:1xLST=>1xCA", duration : std::time::Duration::from_millis(25920000u64),
        inputs : & [StaticRecipeMaterial { ticker : "LST", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CA", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "1xSI 1xO=>1xLCR", standard_recipe_name :
        "CHP:1xO-1xSI=>1xLCR", duration : std::time::Duration::from_millis(56160000u64),
        inputs : & [StaticRecipeMaterial { ticker : "O", amount : 1u32, },
        StaticRecipeMaterial { ticker : "SI", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "LCR", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "10xCLI 20xBRM=>16xSOI",
        standard_recipe_name : "CHP:20xBRM-10xCLI=>16xSOI", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CLI", amount : 10u32, }, StaticRecipeMaterial { ticker : "BRM", amount
        : 20u32, }], outputs : & [StaticRecipeMaterial { ticker : "SOI", amount : 16u32,
        }], }, StaticRecipeInfo { building_ticker : "CHP", recipe_name :
        "25xBRM=>10xREA", standard_recipe_name : "CHP:25xBRM=>10xREA", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BRM", amount : 25u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "REA", amount : 10u32, }], }, StaticRecipeInfo { building_ticker : "CHP",
        recipe_name : "2xCA 1xMG 1xTA=>20xSC", standard_recipe_name :
        "CHP:2xCA-1xMG-1xTA=>20xSC", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CA", amount : 2u32, }, StaticRecipeMaterial { ticker : "TA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "MG", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SC", amount : 20u32, }], }, StaticRecipeInfo {
        building_ticker : "CHP", recipe_name : "2xH2O 2xN 1xLST=>4xNS",
        standard_recipe_name : "CHP:2xH2O-1xLST-2xN=>4xNS", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LST", amount : 1u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        2u32, }, StaticRecipeMaterial { ticker : "N", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "NS", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "CLF", recipe_name : "2xSIL 1xBLE 10xHD 10xPCB=>10xLC",
        standard_recipe_name : "CLF:1xBLE-10xHD-10xPCB-2xSIL=>10xLC", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PCB", amount : 10u32, }, StaticRecipeMaterial { ticker : "SIL", amount
        : 2u32, }, StaticRecipeMaterial { ticker : "BLE", amount : 1u32, },
        StaticRecipeMaterial { ticker : "HD", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "LC", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "CLF", recipe_name : "5xNL 1xBLE 5xSEN 5xPCB=>20xHSS",
        standard_recipe_name : "CLF:1xBLE-5xNL-5xPCB-5xSEN=>20xHSS", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SEN", amount : 5u32, }, StaticRecipeMaterial { ticker : "PCB", amount :
        5u32, }, StaticRecipeMaterial { ticker : "NL", amount : 5u32, },
        StaticRecipeMaterial { ticker : "BLE", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HSS", amount : 20u32, }], }, StaticRecipeInfo {
        building_ticker : "CLF", recipe_name : "1xKV 10xMTC=>100xSPT",
        standard_recipe_name : "CLF:1xKV-10xMTC=>100xSPT", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "KV", amount : 1u32, }, StaticRecipeMaterial { ticker : "MTC", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "SPT", amount : 100u32,
        }], }, StaticRecipeInfo { building_ticker : "CLF", recipe_name :
        "1xNL 1xIND 6xGC=>10xHMS", standard_recipe_name : "CLF:6xGC-1xIND-1xNL=>10xHMS",
        duration : std::time::Duration::from_millis(60480000u64), inputs : &
        [StaticRecipeMaterial { ticker : "IND", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "NL", amount : 1u32, }, StaticRecipeMaterial { ticker : "GC", amount :
        6u32, }], outputs : & [StaticRecipeMaterial { ticker : "HMS", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name :
        "10xC 2xSI=>1200xNFI", standard_recipe_name : "CLR:10xC-2xSI=>1200xNFI", duration
        : std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SI", amount : 2u32, }, StaticRecipeMaterial { ticker : "C", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "NFI", amount : 1200u32,
        }], }, StaticRecipeInfo { building_ticker : "CLR", recipe_name :
        "1xAL 1xSI=>10xTRN", standard_recipe_name : "CLR:1xAL-1xSI=>10xTRN", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "SI", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "TRN", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "1xSI 1xAL=>30xSWF",
        standard_recipe_name : "CLR:1xAL-1xSI=>30xSWF", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SI", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "SWF", amount : 30u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "1xC=>100xNCS",
        standard_recipe_name : "CLR:1xC=>100xNCS", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "C", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "NCS", amount : 100u32, }], }, StaticRecipeInfo { building_ticker : "CLR",
        recipe_name : "1xCAP 1xSWF 1xTRN=>1xTRA", standard_recipe_name :
        "CLR:1xCAP-1xSWF-1xTRN=>1xTRA", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TRN", amount : 1u32, }, StaticRecipeMaterial { ticker : "CAP", amount :
        1u32, }, StaticRecipeMaterial { ticker : "SWF", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "TRA", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "CLR", recipe_name : "4xCF 1xCBM=>1xMFE", standard_recipe_name
        : "CLR:1xCBM-4xCF=>1xMFE", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CF", amount : 4u32, }, StaticRecipeMaterial { ticker : "CBM", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "MFE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "2xCF 1xCBS=>1xSFE",
        standard_recipe_name : "CLR:1xCBS-2xCF=>1xSFE", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CBS", amount : 1u32, }, StaticRecipeMaterial { ticker : "CF", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "SFE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "1xSI 1xFE=>15xCAP",
        standard_recipe_name : "CLR:1xFE-1xSI=>15xCAP", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }, StaticRecipeMaterial { ticker : "SI", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CAP", amount : 15u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "1xGAL 1xSI=>6xLDI",
        standard_recipe_name : "CLR:1xGAL-1xSI=>6xLDI", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GAL", amount : 1u32, }, StaticRecipeMaterial { ticker : "SI", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "LDI", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "2xSI 2xAL=>20xMWF",
        standard_recipe_name : "CLR:2xAL-2xSI=>20xMWF", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SI", amount : 2u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "MWF", amount : 20u32, }],
        }, StaticRecipeInfo { building_ticker : "CLR", recipe_name : "2xNCS=>2xMTC",
        standard_recipe_name : "CLR:2xNCS=>2xMTC", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NCS", amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "MTC", amount : 2u32, }], }, StaticRecipeInfo { building_ticker : "COL",
        recipe_name : "=>", standard_recipe_name : "COL:=>", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [], outputs : & [], },
        StaticRecipeInfo { building_ticker : "DRS", recipe_name :
        "1xSAR 1xBSC 1xDCH=>1xSUD", standard_recipe_name :
        "DRS:1xBSC-1xDCH-1xSAR=>1xSUD", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SAR", amount : 1u32, }, StaticRecipeMaterial { ticker : "DCH", amount :
        1u32, }, StaticRecipeMaterial { ticker : "BSC", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SUD", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "DRS", recipe_name : "1xSEQ 1xBSC 1xDCH=>1xSDR",
        standard_recipe_name : "DRS:1xBSC-1xDCH-1xSEQ=>1xSDR", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BSC", amount : 1u32, }, StaticRecipeMaterial { ticker : "DCH", amount :
        1u32, }, StaticRecipeMaterial { ticker : "SEQ", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SDR", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "DRS", recipe_name : "1xBSC 1xDCH=>1xCCD", standard_recipe_name
        : "DRS:1xBSC-1xDCH=>1xCCD", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BSC", amount : 1u32, }, StaticRecipeMaterial { ticker : "DCH", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CCD", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "DRS", recipe_name :
        "1xSAR 2xUTS 1xDCH=>1xSRD", standard_recipe_name :
        "DRS:1xDCH-1xSAR-2xUTS=>1xSRD", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "UTS", amount : 2u32, }, StaticRecipeMaterial { ticker : "DCH", amount :
        1u32, }, StaticRecipeMaterial { ticker : "SAR", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SRD", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "DRS", recipe_name : "1xSAR 1xDCH=>1xRED", standard_recipe_name
        : "DRS:1xDCH-1xSAR=>1xRED", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DCH", amount : 1u32, }, StaticRecipeMaterial { ticker : "SAR", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "RED", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "DRS", recipe_name :
        "50xNFI 1xDCS=>1xDRF", standard_recipe_name : "DRS:1xDCS-50xNFI=>1xDRF", duration
        : std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial
        { ticker : "NFI", amount : 50u32, }, StaticRecipeMaterial { ticker : "DCS",
        amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "DRF", amount :
        1u32, }], }, StaticRecipeInfo { building_ticker : "DRS", recipe_name :
        "1xMPC 1xPOW 1xSOL 1xDRF=>1xDCH", standard_recipe_name :
        "DRS:1xDRF-1xMPC-1xPOW-1xSOL=>1xDCH", duration :
        std::time::Duration::from_millis(24192000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DRF", amount : 1u32, }, StaticRecipeMaterial { ticker : "SOL", amount :
        1u32, }, StaticRecipeMaterial { ticker : "POW", amount : 1u32, },
        StaticRecipeMaterial { ticker : "MPC", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "DCH", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "ECA", recipe_name : "12xLI 12xBE 60xHCC 150xPG=>1xCBL",
        standard_recipe_name : "ECA:12xBE-60xHCC-12xLI-150xPG=>1xCBL", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HCC", amount : 60u32, }, StaticRecipeMaterial { ticker : "BE", amount :
        12u32, }, StaticRecipeMaterial { ticker : "LI", amount : 12u32, },
        StaticRecipeMaterial { ticker : "PG", amount : 150u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CBL", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "ECA", recipe_name :
        "1xPFG 10xKRE 1xAFR 20xCTF 1xACS 1xRCS=>1xVOR", standard_recipe_name :
        "ECA:1xACS-1xAFR-20xCTF-10xKRE-1xPFG-1xRCS=>1xVOR", duration :
        std::time::Duration::from_millis(691200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "KRE", amount : 10u32, }, StaticRecipeMaterial { ticker : "CTF",
        amount : 20u32, }, StaticRecipeMaterial { ticker : "AFR", amount : 1u32, },
        StaticRecipeMaterial { ticker : "PFG", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "ACS", amount : 1u32, }, StaticRecipeMaterial { ticker : "RCS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "VOR", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ECA", recipe_name :
        "4xEES 8xAFR 10xCTF 1xACS 1xRCS 10xWAL=>1xFIR", standard_recipe_name :
        "ECA:1xACS-8xAFR-10xCTF-4xEES-1xRCS-10xWAL=>1xFIR", duration :
        std::time::Duration::from_millis(302400000u64), inputs : & [StaticRecipeMaterial
        { ticker : "EES", amount : 4u32, }, StaticRecipeMaterial { ticker : "AFR", amount
        : 8u32, }, StaticRecipeMaterial { ticker : "CTF", amount : 10u32, },
        StaticRecipeMaterial { ticker : "ACS", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "RCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "WAL", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "FIR", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ECA", recipe_name :
        "4xETC 4xBFR 6xCF 1xACS 1xRCS 8xAST=>1xRAG", standard_recipe_name :
        "ECA:1xACS-8xAST-4xBFR-6xCF-4xETC-1xRCS=>1xRAG", duration :
        std::time::Duration::from_millis(216000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "CF", amount : 6u32, }, StaticRecipeMaterial { ticker : "BFR", amount
        : 4u32, }, StaticRecipeMaterial { ticker : "ETC", amount : 4u32, },
        StaticRecipeMaterial { ticker : "AST", amount : 8u32, }, StaticRecipeMaterial {
        ticker : "RCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "ACS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "RAG", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ECA", recipe_name :
        "6xLI 6xBE 20xBCO 60xPG=>1xCBS", standard_recipe_name :
        "ECA:20xBCO-6xBE-6xLI-60xPG=>1xCBS", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 60u32, }, StaticRecipeMaterial { ticker : "BCO", amount :
        20u32, }, StaticRecipeMaterial { ticker : "BE", amount : 6u32, },
        StaticRecipeMaterial { ticker : "LI", amount : 6u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CBS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "ECA", recipe_name : "2xCU 2xSI 4xBCO 1xBRO=>12xSOL",
        standard_recipe_name : "ECA:4xBCO-1xBRO-2xCU-2xSI=>12xSOL", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SI", amount : 2u32, }, StaticRecipeMaterial { ticker : "BRO", amount :
        1u32, }, StaticRecipeMaterial { ticker : "BCO", amount : 4u32, },
        StaticRecipeMaterial { ticker : "CU", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SOL", amount : 12u32, }], }, StaticRecipeInfo {
        building_ticker : "ECA", recipe_name : "1xNCS 4xLI=>1xPOW", standard_recipe_name
        : "ECA:4xLI-1xNCS=>1xPOW", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "LI", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "POW", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ECA", recipe_name :
        "8xLI 8xBE 40xBGC 120xPG=>1xCBM", standard_recipe_name :
        "ECA:8xBE-40xBGC-8xLI-120xPG=>1xCBM", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BGC", amount : 40u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        120u32, }, StaticRecipeMaterial { ticker : "LI", amount : 8u32, },
        StaticRecipeMaterial { ticker : "BE", amount : 8u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CBM", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "ECA", recipe_name : "8xGL 24xSOL=>12xSP", standard_recipe_name
        : "ECA:8xGL-24xSOL=>12xSP", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SOL", amount : 24u32, }, StaticRecipeMaterial { ticker : "GL", amount :
        8u32, }], outputs : & [StaticRecipeMaterial { ticker : "SP", amount : 12u32, }],
        }, StaticRecipeInfo { building_ticker : "EDM", recipe_name :
        "1xDCS 1xHD 1xNG=>1xHOG", standard_recipe_name : "EDM:1xDCS-1xHD-1xNG=>1xHOG",
        duration : std::time::Duration::from_millis(25920000u64), inputs : &
        [StaticRecipeMaterial { ticker : "NG", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "DCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "HD", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "HOG", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "EDM", recipe_name :
        "1xDCS 1xSFK 1xPOW=>1xRAD", standard_recipe_name :
        "EDM:1xDCS-1xPOW-1xSFK=>1xRAD", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "SFK", amount :
        1u32, }, StaticRecipeMaterial { ticker : "POW", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAD", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "EDM", recipe_name : "1xPCB 1xTRA=>1xMHP", standard_recipe_name
        : "EDM:1xPCB-1xTRA=>1xMHP", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PCB", amount : 1u32, }, StaticRecipeMaterial { ticker : "TRA", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "MHP", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "EDM", recipe_name :
        "2xSAR 2xCD 1xKV=>2xBID", standard_recipe_name : "EDM:2xCD-1xKV-2xSAR=>2xBID",
        duration : std::time::Duration::from_millis(25920000u64), inputs : &
        [StaticRecipeMaterial { ticker : "SAR", amount : 2u32, }, StaticRecipeMaterial {
        ticker : "CD", amount : 2u32, }, StaticRecipeMaterial { ticker : "KV", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BID", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "EDM", recipe_name :
        "100xPE 4xCU=>8xSCN", standard_recipe_name : "EDM:4xCU-100xPE=>8xSCN", duration :
        std::time::Duration::from_millis(103680000u64), inputs : & [StaticRecipeMaterial
        { ticker : "CU", amount : 4u32, }, StaticRecipeMaterial { ticker : "PE", amount :
        100u32, }], outputs : & [StaticRecipeMaterial { ticker : "SCN", amount : 8u32,
        }], }, StaticRecipeInfo { building_ticker : "EEP", recipe_name :
        "16xKR 1xES 4xREA 4xFLX=>4xKRE", standard_recipe_name :
        "EEP:1xES-4xFLX-16xKR-4xREA=>4xKRE", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FLX", amount : 4u32, }, StaticRecipeMaterial { ticker : "KR", amount
        : 16u32, }, StaticRecipeMaterial { ticker : "ES", amount : 1u32, },
        StaticRecipeMaterial { ticker : "REA", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "KRE", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "EEP", recipe_name : "1xES 4xREA 4xFLX=>1xEES",
        standard_recipe_name : "EEP:1xES-4xFLX-4xREA=>1xEES", duration :
        std::time::Duration::from_millis(103680000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FLX", amount : 4u32, }, StaticRecipeMaterial { ticker : "REA", amount
        : 4u32, }, StaticRecipeMaterial { ticker : "ES", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "EES", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "EEP", recipe_name : "1xLES=>10xES", standard_recipe_name :
        "EEP:1xLES=>10xES", duration : std::time::Duration::from_millis(43200000u64),
        inputs : & [StaticRecipeMaterial { ticker : "LES", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ES", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "ELP", recipe_name : "1xMAG 10xBAC 10xPE=>5xAWF",
        standard_recipe_name : "ELP:10xBAC-1xMAG-10xPE=>5xAWF", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BAC", amount : 10u32, }, StaticRecipeMaterial { ticker : "PE", amount :
        10u32, }, StaticRecipeMaterial { ticker : "MAG", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AWF", amount : 5u32, }], }, StaticRecipeInfo {
        building_ticker : "ELP", recipe_name : "3xCTF 1xCBL=>1xLFE", standard_recipe_name
        : "ELP:1xCBL-3xCTF=>1xLFE", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CBL", amount : 1u32, }, StaticRecipeMaterial { ticker : "CTF", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "LFE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ELP", recipe_name :
        "1xMB 1xDCS 1xCD=>1xHPC", standard_recipe_name : "ELP:1xCD-1xDCS-1xMB=>1xHPC",
        duration : std::time::Duration::from_millis(30240000u64), inputs : &
        [StaticRecipeMaterial { ticker : "CD", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "DCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "MB", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "HPC", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ELP", recipe_name :
        "1xTPU 1xFAN 1xMB 1xDCL=>1xBMF", standard_recipe_name :
        "ELP:1xDCL-1xFAN-1xMB-1xTPU=>1xBMF", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DCL", amount : 1u32, }, StaticRecipeMaterial { ticker : "TPU", amount :
        1u32, }, StaticRecipeMaterial { ticker : "MB", amount : 1u32, },
        StaticRecipeMaterial { ticker : "FAN", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BMF", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "ELP", recipe_name : "1xMB 1xDCM 1xHD=>1xBWS",
        standard_recipe_name : "ELP:1xDCM-1xHD-1xMB=>1xBWS", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MB", amount : 1u32, }, StaticRecipeMaterial { ticker : "DCM", amount :
        1u32, }, StaticRecipeMaterial { ticker : "HD", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BWS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "ELP", recipe_name : "1xKV 2xAU 4xPCB 6xSWF=>3xAAR",
        standard_recipe_name : "ELP:2xAU-1xKV-4xPCB-6xSWF=>3xAAR", duration :
        std::time::Duration::from_millis(57888000u64), inputs : & [StaticRecipeMaterial {
        ticker : "KV", amount : 1u32, }, StaticRecipeMaterial { ticker : "PCB", amount :
        4u32, }, StaticRecipeMaterial { ticker : "AU", amount : 2u32, },
        StaticRecipeMaterial { ticker : "SWF", amount : 6u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AAR", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "ELP", recipe_name : "1xDIS 2xBCO=>1xCD", standard_recipe_name
        : "ELP:2xBCO-1xDIS=>1xCD", duration :
        std::time::Duration::from_millis(57888000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BCO", amount : 2u32, }, StaticRecipeMaterial { ticker : "DIS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CD", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "ELP", recipe_name :
        "16xSEN 4xPCB=>1xSAR", standard_recipe_name : "ELP:4xPCB-16xSEN=>1xSAR", duration
        : std::time::Duration::from_millis(57888000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PCB", amount : 4u32, }, StaticRecipeMaterial { ticker : "SEN", amount
        : 16u32, }], outputs : & [StaticRecipeMaterial { ticker : "SAR", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "EXT", recipe_name : "=>",
        standard_recipe_name : "EXT:=>", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [], outputs : & [], },
        StaticRecipeInfo { building_ticker : "FER", recipe_name :
        "10xDW 4xPIB 1xCA 1xI 1xAMM=>20xVIT", standard_recipe_name :
        "FER:1xAMM-1xCA-10xDW-1xI-4xPIB=>20xVIT", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CA", amount : 1u32, }, StaticRecipeMaterial { ticker : "AMM", amount :
        1u32, }, StaticRecipeMaterial { ticker : "I", amount : 1u32, },
        StaticRecipeMaterial { ticker : "PIB", amount : 4u32, }, StaticRecipeMaterial {
        ticker : "DW", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "VIT", amount : 20u32, }], }, StaticRecipeInfo { building_ticker : "FER",
        recipe_name : "20xDW 15xGRA 15xREA 1xAMM=>10xWIN", standard_recipe_name :
        "FER:1xAMM-20xDW-15xGRA-15xREA=>10xWIN", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DW", amount : 20u32, }, StaticRecipeMaterial { ticker : "AMM", amount :
        1u32, }, StaticRecipeMaterial { ticker : "REA", amount : 15u32, },
        StaticRecipeMaterial { ticker : "GRA", amount : 15u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "WIN", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FER", recipe_name : "2xGRN 2xDW 1xES 1xAMM=>4xGIN",
        standard_recipe_name : "FER:1xAMM-2xDW-1xES-2xGRN=>4xGIN", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GRN", amount : 2u32, }, StaticRecipeMaterial { ticker : "DW", amount :
        2u32, }, StaticRecipeMaterial { ticker : "ES", amount : 1u32, },
        StaticRecipeMaterial { ticker : "AMM", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GIN", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "FER", recipe_name : "2xGRN 3xDW 1xHOP 1xAMM=>6xALE",
        standard_recipe_name : "FER:1xAMM-3xDW-2xGRN-1xHOP=>6xALE", duration :
        std::time::Duration::from_millis(55296000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GRN", amount : 2u32, }, StaticRecipeMaterial { ticker : "DW", amount :
        3u32, }, StaticRecipeMaterial { ticker : "AMM", amount : 1u32, },
        StaticRecipeMaterial { ticker : "HOP", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ALE", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "FER", recipe_name : "4xDW 1xHER 1xAMM=>6xKOM",
        standard_recipe_name : "FER:1xAMM-4xDW-1xHER=>6xKOM", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DW", amount : 4u32, }, StaticRecipeMaterial { ticker : "HER", amount :
        1u32, }, StaticRecipeMaterial { ticker : "AMM", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "KOM", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "10xH2O 1xPG=>10xDW", standard_recipe_name
        : "FP:10xH2O-1xPG=>10xDW", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 1u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "DW", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "FP", recipe_name : "10xH2O=>7xDW",
        standard_recipe_name : "FP:10xH2O=>7xDW", duration :
        std::time::Duration::from_millis(8640000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "DW", amount : 7u32, }], }, StaticRecipeInfo { building_ticker : "FP",
        recipe_name : "1xALG 1xH2O 1xBEA=>6xPPA", standard_recipe_name :
        "FP:1xALG-1xBEA-1xH2O=>6xPPA", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ALG", amount : 1u32, }, StaticRecipeMaterial { ticker : "BEA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "H2O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PPA", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xGRN 1xALG 1xNUT=>10xRAT",
        standard_recipe_name : "FP:1xALG-1xGRN-1xNUT=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NUT", amount : 1u32, }, StaticRecipeMaterial { ticker : "ALG", amount :
        1u32, }, StaticRecipeMaterial { ticker : "GRN", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xGRN 1xALG 1xVEG=>10xRAT",
        standard_recipe_name : "FP:1xALG-1xGRN-1xVEG=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GRN", amount : 1u32, }, StaticRecipeMaterial { ticker : "VEG", amount :
        1u32, }, StaticRecipeMaterial { ticker : "ALG", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMAI 1xALG 1xNUT=>10xRAT",
        standard_recipe_name : "FP:1xALG-1xMAI-1xNUT=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ALG", amount : 1u32, }, StaticRecipeMaterial { ticker : "NUT", amount :
        1u32, }, StaticRecipeMaterial { ticker : "MAI", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMAI 1xALG 1xVEG=>10xRAT",
        standard_recipe_name : "FP:1xALG-1xMAI-1xVEG=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "VEG", amount : 1u32, }, StaticRecipeMaterial { ticker : "ALG", amount :
        1u32, }, StaticRecipeMaterial { ticker : "MAI", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xGRN 1xBEA 1xNUT=>10xRAT",
        standard_recipe_name : "FP:1xBEA-1xGRN-1xNUT=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NUT", amount : 1u32, }, StaticRecipeMaterial { ticker : "GRN", amount :
        1u32, }, StaticRecipeMaterial { ticker : "BEA", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xGRN 1xBEA 1xVEG=>10xRAT",
        standard_recipe_name : "FP:1xBEA-1xGRN-1xVEG=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GRN", amount : 1u32, }, StaticRecipeMaterial { ticker : "BEA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "VEG", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMAI 1xBEA 1xNUT=>10xRAT",
        standard_recipe_name : "FP:1xBEA-1xMAI-1xNUT=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NUT", amount : 1u32, }, StaticRecipeMaterial { ticker : "BEA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "MAI", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMAI 1xBEA 1xVEG=>10xRAT",
        standard_recipe_name : "FP:1xBEA-1xMAI-1xVEG=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MAI", amount : 1u32, }, StaticRecipeMaterial { ticker : "BEA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "VEG", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xCAF 3xDW=>3xCOF", standard_recipe_name :
        "FP:1xCAF-3xDW=>3xCOF", duration : std::time::Duration::from_millis(25920000u64),
        inputs : & [StaticRecipeMaterial { ticker : "CAF", amount : 1u32, },
        StaticRecipeMaterial { ticker : "DW", amount : 3u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "COF", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMUS 1xNUT 1xGRN=>10xRAT",
        standard_recipe_name : "FP:1xGRN-1xMUS-1xNUT=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MUS", amount : 1u32, }, StaticRecipeMaterial { ticker : "GRN", amount :
        1u32, }, StaticRecipeMaterial { ticker : "NUT", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMUS 1xVEG 1xGRN=>10xRAT",
        standard_recipe_name : "FP:1xGRN-1xMUS-1xVEG=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "VEG", amount : 1u32, }, StaticRecipeMaterial { ticker : "MUS", amount :
        1u32, }, StaticRecipeMaterial { ticker : "GRN", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "2xRAT 1xHER=>2xFIM", standard_recipe_name
        : "FP:1xHER-2xRAT=>2xFIM", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HER", amount : 1u32, }, StaticRecipeMaterial { ticker : "RAT", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "FIM", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "FP", recipe_name :
        "1xMUS 1xNUT 1xMAI=>10xRAT", standard_recipe_name :
        "FP:1xMAI-1xMUS-1xNUT=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MAI", amount : 1u32, }, StaticRecipeMaterial { ticker : "MUS", amount :
        1u32, }, StaticRecipeMaterial { ticker : "NUT", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "1xMUS 1xVEG 1xMAI=>10xRAT",
        standard_recipe_name : "FP:1xMAI-1xMUS-1xVEG=>10xRAT", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MAI", amount : 1u32, }, StaticRecipeMaterial { ticker : "VEG", amount :
        1u32, }, StaticRecipeMaterial { ticker : "MUS", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "FP", recipe_name : "2xALG 1xH2O=>4xPPA", standard_recipe_name
        : "FP:2xALG-1xH2O=>4xPPA", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ALG", amount : 2u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PPA", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "FP", recipe_name : "2xBEA 1xH2O=>4xPPA",
        standard_recipe_name : "FP:2xBEA-1xH2O=>4xPPA", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BEA", amount : 2u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PPA", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "FP", recipe_name : "2xFIM 1xMTP=>2xMEA",
        standard_recipe_name : "FP:2xFIM-1xMTP=>2xMEA", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FIM", amount : 2u32, }, StaticRecipeMaterial { ticker : "MTP", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "MEA", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "FP", recipe_name :
        "5xNUT 5xMAI 5xVEG=>16xFOD", standard_recipe_name :
        "FP:5xMAI-5xNUT-5xVEG=>16xFOD", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "VEG", amount : 5u32, }, StaticRecipeMaterial { ticker : "NUT", amount :
        5u32, }, StaticRecipeMaterial { ticker : "MAI", amount : 5u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FOD", amount : 16u32, }], }, StaticRecipeInfo {
        building_ticker : "FRM", recipe_name : "4xH2O 1xDDT=>4xHER", standard_recipe_name
        : "FRM:1xDDT-4xH2O=>4xHER", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 4u32, }, StaticRecipeMaterial { ticker : "DDT", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "HER", amount : 4u32,
        }], }, StaticRecipeInfo { building_ticker : "FRM", recipe_name : "1xH2O=>12xNUT",
        standard_recipe_name : "FRM:1xH2O=>12xNUT", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "NUT", amount : 12u32, }], }, StaticRecipeInfo { building_ticker : "FRM",
        recipe_name : "1xH2O=>2xBEA", standard_recipe_name : "FRM:1xH2O=>2xBEA", duration
        : std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "BEA", amount : 2u32, }], }, StaticRecipeInfo { building_ticker : "FRM",
        recipe_name : "1xH2O=>4xGRN", standard_recipe_name : "FRM:1xH2O=>4xGRN", duration
        : std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "GRN", amount : 4u32, }], }, StaticRecipeInfo { building_ticker : "FRM",
        recipe_name : "2xH2O 4xNS=>2xRCO", standard_recipe_name :
        "FRM:2xH2O-4xNS=>2xRCO", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 2u32, }, StaticRecipeMaterial { ticker : "NS", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "RCO", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "FRM", recipe_name : "2xH2O=>1xRCO",
        standard_recipe_name : "FRM:2xH2O=>1xRCO", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "RCO", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "FRM",
        recipe_name : "2xH2O=>4xHCP", standard_recipe_name : "FRM:2xH2O=>4xHCP", duration
        : std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "HCP", amount : 4u32, }], }, StaticRecipeInfo { building_ticker : "FRM",
        recipe_name : "3xH2O=>4xVEG", standard_recipe_name : "FRM:3xH2O=>4xVEG", duration
        : std::time::Duration::from_millis(38880000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 3u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "VEG", amount : 4u32, }], }, StaticRecipeInfo { building_ticker : "FRM",
        recipe_name : "4xH2O=>12xMAI", standard_recipe_name : "FRM:4xH2O=>12xMAI",
        duration : std::time::Duration::from_millis(120960000u64), inputs : &
        [StaticRecipeMaterial { ticker : "H2O", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "MAI", amount : 12u32, }], }, StaticRecipeInfo {
        building_ticker : "FRM", recipe_name : "4xH2O=>4xGRN", standard_recipe_name :
        "FRM:4xH2O=>4xGRN", duration : std::time::Duration::from_millis(32400000u64),
        inputs : & [StaticRecipeMaterial { ticker : "H2O", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GRN", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "FRM", recipe_name : "6xH2O=>4xBEA", standard_recipe_name :
        "FRM:6xH2O=>4xBEA", duration : std::time::Duration::from_millis(34560000u64),
        inputs : & [StaticRecipeMaterial { ticker : "H2O", amount : 6u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BEA", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "FS", recipe_name : "10xAL 4xSTL=>1xFLO", standard_recipe_name
        : "FS:10xAL-4xSTL=>1xFLO", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 10u32, }, StaticRecipeMaterial { ticker : "STL", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "FLO", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "FS", recipe_name : "1xSTL 1xAL=>1xBFR",
        standard_recipe_name : "FS:1xAL-1xSTL=>1xBFR", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "STL", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BFR", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "FS", recipe_name : "1xZR 1xAL=>1xAFR",
        standard_recipe_name : "FS:1xAL-1xZR=>1xAFR", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "ZR", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "AFR", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "FS", recipe_name : "1xAL 2xCU=>3xBRO",
        standard_recipe_name : "FS:1xAL-2xCU=>3xBRO", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "CU", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "BRO", amount : 3u32, }],
        }, StaticRecipeInfo { building_ticker : "FS", recipe_name :
        "1xBGO 300xPE=>10xBGC", standard_recipe_name : "FS:1xBGO-300xPE=>10xBGC",
        duration : std::time::Duration::from_millis(25920000u64), inputs : &
        [StaticRecipeMaterial { ticker : "BGO", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "PE", amount : 300u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "BGC", amount : 10u32, }], }, StaticRecipeInfo { building_ticker : "FS",
        recipe_name : "1xCU 300xPE=>10xBCO", standard_recipe_name :
        "FS:1xCU-300xPE=>10xBCO", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CU", amount : 1u32, }, StaticRecipeMaterial { ticker : "PE", amount :
        300u32, }], outputs : & [StaticRecipeMaterial { ticker : "BCO", amount : 10u32,
        }], }, StaticRecipeInfo { building_ticker : "FS", recipe_name : "1xFE=>16xSFK",
        standard_recipe_name : "FS:1xFE=>16xSFK", duration :
        std::time::Duration::from_millis(10368000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "SFK", amount : 16u32, }], }, StaticRecipeInfo { building_ticker : "FS",
        recipe_name : "2xSTL 2xTI 1xHOG=>1xSEQ", standard_recipe_name :
        "FS:1xHOG-2xSTL-2xTI=>1xSEQ", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HOG", amount : 1u32, }, StaticRecipeMaterial { ticker : "TI", amount :
        2u32, }, StaticRecipeMaterial { ticker : "STL", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SEQ", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "FS", recipe_name : "1xSTL=>8xMFK", standard_recipe_name :
        "FS:1xSTL=>8xMFK", duration : std::time::Duration::from_millis(10368000u64),
        inputs : & [StaticRecipeMaterial { ticker : "STL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "MFK", amount : 8u32, }], }, StaticRecipeInfo {
        building_ticker : "FS", recipe_name : "1xRGO 300xPE=>10xHCC",
        standard_recipe_name : "FS:300xPE-1xRGO=>10xHCC", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 300u32, }, StaticRecipeMaterial { ticker : "RGO", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "HCC", amount : 10u32,
        }], }, StaticRecipeInfo { building_ticker : "FS", recipe_name :
        "4xAU 1xCU=>5xRGO", standard_recipe_name : "FS:4xAU-1xCU=>5xRGO", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AU", amount : 4u32, }, StaticRecipeMaterial { ticker : "CU", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "RGO", amount : 5u32, }],
        }, StaticRecipeInfo { building_ticker : "FS", recipe_name : "4xAU 1xFE=>5xBGO",
        standard_recipe_name : "FS:4xAU-1xFE=>5xBGO", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }, StaticRecipeMaterial { ticker : "AU", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "BGO", amount : 5u32, }],
        }, StaticRecipeInfo { building_ticker : "FS", recipe_name : "2xSFK 6xFE=>1xUTS",
        standard_recipe_name : "FS:6xFE-2xSFK=>1xUTS", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 6u32, }, StaticRecipeMaterial { ticker : "SFK", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "UTS", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "GF", recipe_name :
        "10xGL 10xNCS=>10xNG", standard_recipe_name : "GF:10xGL-10xNCS=>10xNG", duration
        : std::time::Duration::from_millis(103680000u64), inputs : &
        [StaticRecipeMaterial { ticker : "GL", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "NCS", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "NG", amount : 10u32, }], }, StaticRecipeInfo { building_ticker : "GF",
        recipe_name : "10xGL 15xPG 1xSEN=>10xRG", standard_recipe_name :
        "GF:10xGL-15xPG-1xSEN=>10xRG", duration :
        std::time::Duration::from_millis(99360000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 15u32, }, StaticRecipeMaterial { ticker : "GL", amount :
        10u32, }, StaticRecipeMaterial { ticker : "SEN", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RG", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "GF", recipe_name : "10xGL 15xPG=>10xRG", standard_recipe_name
        : "GF:10xGL-15xPG=>10xRG", duration :
        std::time::Duration::from_millis(112320000u64), inputs : & [StaticRecipeMaterial
        { ticker : "GL", amount : 10u32, }, StaticRecipeMaterial { ticker : "PG", amount
        : 15u32, }], outputs : & [StaticRecipeMaterial { ticker : "RG", amount : 10u32,
        }], }, StaticRecipeInfo { building_ticker : "GF", recipe_name : "10xGL=>20xTUB",
        standard_recipe_name : "GF:10xGL=>20xTUB", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GL", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "TUB", amount : 20u32, }], }, StaticRecipeInfo { building_ticker : "GF",
        recipe_name : "16xNE 8xRG=>1xLIT", standard_recipe_name : "GF:16xNE-8xRG=>1xLIT",
        duration : std::time::Duration::from_millis(69120000u64), inputs : &
        [StaticRecipeMaterial { ticker : "RG", amount : 8u32, }, StaticRecipeMaterial {
        ticker : "NE", amount : 16u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "LIT", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "GF",
        recipe_name : "2xSIO 1xNA 1xFLX=>12xGL", standard_recipe_name :
        "GF:1xFLX-1xNA-2xSIO=>12xGL", duration :
        std::time::Duration::from_millis(62208000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FLX", amount : 1u32, }, StaticRecipeMaterial { ticker : "NA", amount :
        1u32, }, StaticRecipeMaterial { ticker : "SIO", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GL", amount : 12u32, }], }, StaticRecipeInfo {
        building_ticker : "GF", recipe_name : "20xRG 1xTI 1xGCH=>1xGNZ",
        standard_recipe_name : "GF:1xGCH-20xRG-1xTI=>1xGNZ", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "TI", amount : 1u32, }, StaticRecipeMaterial { ticker : "GCH", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "RG", amount : 20u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GNZ", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "GF", recipe_name : "20xRG 1xGV=>1xGCH", standard_recipe_name :
        "GF:1xGV-20xRG=>1xGCH", duration : std::time::Duration::from_millis(60480000u64),
        inputs : & [StaticRecipeMaterial { ticker : "GV", amount : 1u32, },
        StaticRecipeMaterial { ticker : "RG", amount : 20u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GCH", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "GF", recipe_name : "2xSIO 1xNA 1xSEN=>10xGL",
        standard_recipe_name : "GF:1xNA-1xSEN-2xSIO=>10xGL", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NA", amount : 1u32, }, StaticRecipeMaterial { ticker : "SIO", amount :
        2u32, }, StaticRecipeMaterial { ticker : "SEN", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GL", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "GF", recipe_name : "2xSIO 1xNA=>10xGL", standard_recipe_name :
        "GF:1xNA-2xSIO=>10xGL", duration : std::time::Duration::from_millis(51840000u64),
        inputs : & [StaticRecipeMaterial { ticker : "SIO", amount : 2u32, },
        StaticRecipeMaterial { ticker : "NA", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GL", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "GF", recipe_name : "4xLFP 4xGNZ 50xNG 10xTHF 1xMFK=>1xGEN",
        standard_recipe_name : "GF:4xGNZ-4xLFP-1xMFK-50xNG-10xTHF=>1xGEN", duration :
        std::time::Duration::from_millis(108000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "THF", amount : 10u32, }, StaticRecipeMaterial { ticker : "GNZ",
        amount : 4u32, }, StaticRecipeMaterial { ticker : "LFP", amount : 4u32, },
        StaticRecipeMaterial { ticker : "NG", amount : 50u32, }, StaticRecipeMaterial {
        ticker : "MFK", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "GEN", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "HWP",
        recipe_name : "12xAL 1xSTL 1xHE=>4xBHP", standard_recipe_name :
        "HWP:12xAL-1xHE-1xSTL=>4xBHP", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 12u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }, StaticRecipeMaterial { ticker : "STL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BHP", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "HWP", recipe_name : "12xAL 4xTI 1xHE=>4xRHP",
        standard_recipe_name : "HWP:12xAL-1xHE-4xTI=>4xRHP", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 12u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }, StaticRecipeMaterial { ticker : "TI", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RHP", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "HWP", recipe_name : "12xAL 1xHE=>4xLHP", standard_recipe_name
        : "HWP:12xAL-1xHE=>4xLHP", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HE", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        12u32, }], outputs : & [StaticRecipeMaterial { ticker : "LHP", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "HWP", recipe_name :
        "12xAL 4xAST 1xHE=>4xHHP", standard_recipe_name : "HWP:12xAL-4xAST-1xHE=>4xHHP",
        duration : std::time::Duration::from_millis(43200000u64), inputs : &
        [StaticRecipeMaterial { ticker : "AL", amount : 12u32, }, StaticRecipeMaterial {
        ticker : "AST", amount : 4u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "HHP", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "HWP", recipe_name :
        "2xBOS 2xCTF 1xHE=>2xATP", standard_recipe_name : "HWP:2xBOS-2xCTF-1xHE=>2xATP",
        duration : std::time::Duration::from_millis(21600000u64), inputs : &
        [StaticRecipeMaterial { ticker : "BOS", amount : 2u32, }, StaticRecipeMaterial {
        ticker : "CTF", amount : 2u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "ATP", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "HWP", recipe_name :
        "2xFAL 4xCF 1xKV 1xHE=>10xBWH", standard_recipe_name :
        "HWP:4xCF-2xFAL-1xHE-1xKV=>10xBWH", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HE", amount : 1u32, }, StaticRecipeMaterial { ticker : "KV", amount :
        1u32, }, StaticRecipeMaterial { ticker : "CF", amount : 4u32, },
        StaticRecipeMaterial { ticker : "FAL", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BWH", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "HWP", recipe_name : "2xTI 4xCTF 2xTK 1xHE=>10xAWH",
        standard_recipe_name : "HWP:4xCTF-1xHE-2xTI-2xTK=>10xAWH", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TK", amount : 2u32, }, StaticRecipeMaterial { ticker : "TI", amount :
        2u32, }, StaticRecipeMaterial { ticker : "CTF", amount : 4u32, },
        StaticRecipeMaterial { ticker : "HE", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AWH", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "HWP", recipe_name : "8xAL 10xFET 1xHE=>4xAHP",
        standard_recipe_name : "HWP:8xAL-10xFET-1xHE=>4xAHP", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FET", amount : 10u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        8u32, }, StaticRecipeMaterial { ticker : "HE", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AHP", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "HYF", recipe_name : "10xH2O 4xNS=>2xRCO", standard_recipe_name
        : "HYF:10xH2O-4xNS=>2xRCO", duration :
        std::time::Duration::from_millis(19008000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NS", amount : 4u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "RCO", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "HYF", recipe_name :
        "14xH2O 1xNS=>8xHCP", standard_recipe_name : "HYF:14xH2O-1xNS=>8xHCP", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 14u32, }, StaticRecipeMaterial { ticker : "NS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "HCP", amount : 8u32, }],
        }, StaticRecipeInfo { building_ticker : "HYF", recipe_name :
        "16xH2O 1xNS=>6xVEG", standard_recipe_name : "HYF:16xH2O-1xNS=>6xVEG", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NS", amount : 1u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        16u32, }], outputs : & [StaticRecipeMaterial { ticker : "VEG", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "HYF", recipe_name :
        "16xH2O 2xNS=>12xALG", standard_recipe_name : "HYF:16xH2O-2xNS=>12xALG", duration
        : std::time::Duration::from_millis(38880000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 16u32, }, StaticRecipeMaterial { ticker : "NS", amount
        : 2u32, }], outputs : & [StaticRecipeMaterial { ticker : "ALG", amount : 12u32,
        }], }, StaticRecipeInfo { building_ticker : "HYF", recipe_name : "1xNS=>4xMUS",
        standard_recipe_name : "HYF:1xNS=>4xMUS", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NS", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "MUS", amount : 4u32, }], }, StaticRecipeInfo { building_ticker : "HYF",
        recipe_name : "20xH2O 2xNS=>12xMAI", standard_recipe_name :
        "HYF:20xH2O-2xNS=>12xMAI", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NS", amount : 2u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        20u32, }], outputs : & [StaticRecipeMaterial { ticker : "MAI", amount : 12u32,
        }], }, StaticRecipeInfo { building_ticker : "HYF", recipe_name :
        "22xH2O 3xNS=>2xCAF", standard_recipe_name : "HYF:22xH2O-3xNS=>2xCAF", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 22u32, }, StaticRecipeMaterial { ticker : "NS", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "CAF", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "HYF", recipe_name : "4xNS=>12xMUS",
        standard_recipe_name : "HYF:4xNS=>12xMUS", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NS", amount : 4u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "MUS", amount : 12u32, }], }, StaticRecipeInfo { building_ticker : "INC",
        recipe_name : "4xHCP 2xGRN 2xMAI=>4xC", standard_recipe_name :
        "INC:2xGRN-4xHCP-2xMAI=>4xC", duration :
        std::time::Duration::from_millis(28512000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HCP", amount : 4u32, }, StaticRecipeMaterial { ticker : "MAI", amount :
        2u32, }, StaticRecipeMaterial { ticker : "GRN", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "C", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "INC", recipe_name : "4xHCP 2xGRN=>4xC", standard_recipe_name :
        "INC:2xGRN-4xHCP=>4xC", duration : std::time::Duration::from_millis(57024000u64),
        inputs : & [StaticRecipeMaterial { ticker : "HCP", amount : 4u32, },
        StaticRecipeMaterial { ticker : "GRN", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "C", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "INC", recipe_name : "4xGRN=>4xC", standard_recipe_name :
        "INC:4xGRN=>4xC", duration : std::time::Duration::from_millis(86400000u64),
        inputs : & [StaticRecipeMaterial { ticker : "GRN", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "C", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "INC", recipe_name : "4xHCP 2xMAI=>4xC", standard_recipe_name :
        "INC:4xHCP-2xMAI=>4xC", duration : std::time::Duration::from_millis(57024000u64),
        inputs : & [StaticRecipeMaterial { ticker : "MAI", amount : 2u32, },
        StaticRecipeMaterial { ticker : "HCP", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "C", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "INC", recipe_name : "4xHCP=>4xC", standard_recipe_name :
        "INC:4xHCP=>4xC", duration : std::time::Duration::from_millis(86400000u64),
        inputs : & [StaticRecipeMaterial { ticker : "HCP", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "C", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "INC", recipe_name : "4xMAI=>4xC", standard_recipe_name :
        "INC:4xMAI=>4xC", duration : std::time::Duration::from_millis(86400000u64),
        inputs : & [StaticRecipeMaterial { ticker : "MAI", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "C", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "IVP", recipe_name : "16xH2O 1xHCP 1xNS=>8xRSI",
        standard_recipe_name : "IVP:16xH2O-1xHCP-1xNS=>8xRSI", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NS", amount : 1u32, }, StaticRecipeMaterial { ticker : "HCP", amount :
        1u32, }, StaticRecipeMaterial { ticker : "H2O", amount : 16u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RSI", amount : 8u32, }], }, StaticRecipeInfo {
        building_ticker : "IVP", recipe_name : "1xPPA 1xNS 1xHCP=>4xMTP",
        standard_recipe_name : "IVP:1xHCP-1xNS-1xPPA=>4xMTP", duration :
        std::time::Duration::from_millis(64800000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PPA", amount : 1u32, }, StaticRecipeMaterial { ticker : "NS", amount :
        1u32, }, StaticRecipeMaterial { ticker : "HCP", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "MTP", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "IVP", recipe_name : "2xVIT 4xREA 2xBAC 2xBL=>20xVG",
        standard_recipe_name : "IVP:2xBAC-2xBL-4xREA-2xVIT=>20xVG", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "VIT", amount : 2u32, }, StaticRecipeMaterial { ticker : "REA", amount :
        4u32, }, StaticRecipeMaterial { ticker : "BL", amount : 2u32, },
        StaticRecipeMaterial { ticker : "BAC", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "VG", amount : 20u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "10xNAB 3xS 2xO=>4xBLE",
        standard_recipe_name : "LAB:10xNAB-2xO-3xS=>4xBLE", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 2u32, }, StaticRecipeMaterial { ticker : "S", amount :
        3u32, }, StaticRecipeMaterial { ticker : "NAB", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BLE", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "1xES 1xALG 1xTHF=>4xNST",
        standard_recipe_name : "LAB:1xALG-1xES-1xTHF=>4xNST", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ALG", amount : 1u32, }, StaticRecipeMaterial { ticker : "THF", amount :
        1u32, }, StaticRecipeMaterial { ticker : "ES", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "NST", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "1xC 1xH 1xCL=>3xDDT",
        standard_recipe_name : "LAB:1xC-1xCL-1xH=>3xDDT", duration :
        std::time::Duration::from_millis(64800000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CL", amount : 1u32, }, StaticRecipeMaterial { ticker : "H", amount :
        1u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "DDT", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "1xCL 1xO 1xH=>3xTCL",
        standard_recipe_name : "LAB:1xCL-1xH-1xO=>3xTCL", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H", amount : 1u32, }, StaticRecipeMaterial { ticker : "CL", amount :
        1u32, }, StaticRecipeMaterial { ticker : "O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "TCL", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "1xNA 1xCL 1xO=>3xBLE",
        standard_recipe_name : "LAB:1xCL-1xNA-1xO=>3xBLE", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }, StaticRecipeMaterial { ticker : "CL", amount :
        1u32, }, StaticRecipeMaterial { ticker : "NA", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BLE", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "1xNS 1xDW=>3xCST", standard_recipe_name :
        "LAB:1xDW-1xNS=>3xCST", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "DW", amount : 1u32, }, StaticRecipeMaterial { ticker : "NS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CST", amount : 3u32, }],
        }, StaticRecipeInfo { building_ticker : "LAB", recipe_name : "1xF 1xO=>2xBL",
        standard_recipe_name : "LAB:1xF-1xO=>2xBL", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "F", amount : 1u32, }, StaticRecipeMaterial { ticker : "O", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BL", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "LAB", recipe_name :
        "1xHCP 1xO 1xS=>10xBAC", standard_recipe_name : "LAB:1xHCP-1xO-1xS=>10xBAC",
        duration : std::time::Duration::from_millis(190080000u64), inputs : &
        [StaticRecipeMaterial { ticker : "HCP", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }, StaticRecipeMaterial { ticker : "S", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BAC", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "LAB", recipe_name :
        "4xAMM 4xH2O 4xNA=>20xTHF", standard_recipe_name :
        "LAB:4xAMM-4xH2O-4xNA=>20xTHF", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AMM", amount : 4u32, }, StaticRecipeMaterial { ticker : "H2O", amount :
        4u32, }, StaticRecipeMaterial { ticker : "NA", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "THF", amount : 20u32, }], }, StaticRecipeInfo {
        building_ticker : "LAB", recipe_name : "50xEPO 75xNCS=>50xNR",
        standard_recipe_name : "LAB:50xEPO-75xNCS=>50xNR", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NCS", amount : 75u32, }, StaticRecipeMaterial { ticker : "EPO", amount
        : 50u32, }], outputs : & [StaticRecipeMaterial { ticker : "NR", amount : 50u32,
        }], }, StaticRecipeInfo { building_ticker : "LAB", recipe_name :
        "10xHEX 5xBL 5xCST 1xPK=>30xJUI", standard_recipe_name :
        "LAB:5xBL-5xCST-10xHEX-1xPK=>30xJUI", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HEX", amount : 10u32, }, StaticRecipeMaterial { ticker : "BL", amount :
        5u32, }, StaticRecipeMaterial { ticker : "CST", amount : 5u32, },
        StaticRecipeMaterial { ticker : "PK", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "JUI", amount : 30u32, }], }, StaticRecipeInfo {
        building_ticker : "MCA", recipe_name : "1xAL 8xDCS 2xSFK=>8xFAN",
        standard_recipe_name : "MCA:1xAL-8xDCS-2xSFK=>8xFAN", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DCS", amount : 8u32, }, StaticRecipeMaterial { ticker : "SFK", amount :
        2u32, }, StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FAN", amount : 8u32, }], }, StaticRecipeInfo {
        building_ticker : "MCA", recipe_name : "1xLCR 3xGL=>1xDIS", standard_recipe_name
        : "MCA:3xGL-1xLCR=>1xDIS", duration :
        std::time::Duration::from_millis(57888000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LCR", amount : 1u32, }, StaticRecipeMaterial { ticker : "GL", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "DIS", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "MCA", recipe_name :
        "4xRAM 4xMPC 4xPSM 4xSFK=>4xMB", standard_recipe_name :
        "MCA:4xMPC-4xPSM-4xRAM-4xSFK=>4xMB", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RAM", amount : 4u32, }, StaticRecipeMaterial { ticker : "MPC", amount :
        4u32, }, StaticRecipeMaterial { ticker : "PSM", amount : 4u32, },
        StaticRecipeMaterial { ticker : "SFK", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "MB", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "MCA", recipe_name : "1xRG 5xDCS 5xLDI=>5xHD",
        standard_recipe_name : "MCA:5xDCS-5xLDI-1xRG=>5xHD", duration :
        std::time::Duration::from_millis(57888000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RG", amount : 1u32, }, StaticRecipeMaterial { ticker : "LDI", amount :
        5u32, }, StaticRecipeMaterial { ticker : "DCS", amount : 5u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HD", amount : 5u32, }], }, StaticRecipeInfo {
        building_ticker : "ORC", recipe_name : "20xH2O 1xDDT 2xSOI=>12xPIB",
        standard_recipe_name : "ORC:1xDDT-20xH2O-2xSOI=>12xPIB", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DDT", amount : 1u32, }, StaticRecipeMaterial { ticker : "SOI", amount :
        2u32, }, StaticRecipeMaterial { ticker : "H2O", amount : 20u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PIB", amount : 12u32, }], }, StaticRecipeInfo {
        building_ticker : "ORC", recipe_name : "30xH2O 1xDDT 3xSOI=>6xGRA",
        standard_recipe_name : "ORC:1xDDT-30xH2O-3xSOI=>6xGRA", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 30u32, }, StaticRecipeMaterial { ticker : "DDT", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "SOI", amount : 3u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GRA", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "ORC", recipe_name : "30xH2O 1xDDT=>10xPIB",
        standard_recipe_name : "ORC:1xDDT-30xH2O=>10xPIB", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H2O", amount : 30u32, }, StaticRecipeMaterial { ticker : "DDT", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PIB", amount : 10u32,
        }], }, StaticRecipeInfo { building_ticker : "ORC", recipe_name :
        "40xH2O 1xDDT=>5xGRA", standard_recipe_name : "ORC:1xDDT-40xH2O=>5xGRA", duration
        : std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial
        { ticker : "DDT", amount : 1u32, }, StaticRecipeMaterial { ticker : "H2O", amount
        : 40u32, }], outputs : & [StaticRecipeMaterial { ticker : "GRA", amount : 5u32,
        }], }, StaticRecipeInfo { building_ticker : "ORC", recipe_name :
        "40xH2O 2xDDT 4xSOI=>18xHOP", standard_recipe_name :
        "ORC:2xDDT-40xH2O-4xSOI=>18xHOP", duration :
        std::time::Duration::from_millis(151200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 40u32, }, StaticRecipeMaterial { ticker : "SOI",
        amount : 4u32, }, StaticRecipeMaterial { ticker : "DDT", amount : 2u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "HOP", amount : 18u32, }], },
        StaticRecipeInfo { building_ticker : "ORC", recipe_name : "60xH2O 2xDDT=>15xHOP",
        standard_recipe_name : "ORC:2xDDT-60xH2O=>15xHOP", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "H2O", amount : 60u32, }, StaticRecipeMaterial { ticker : "DDT",
        amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker : "HOP", amount :
        15u32, }], }, StaticRecipeInfo { building_ticker : "PAC", recipe_name :
        "100xDW 70xMEA 5xMED 2xLC 1xWS 10xWIN 1xNST 8xPSS 280xPE=>1xCBU",
        standard_recipe_name :
        "PAC:100xDW-2xLC-70xMEA-5xMED-1xNST-280xPE-8xPSS-10xWIN-1xWS=>1xCBU", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 280u32, }, StaticRecipeMaterial { ticker : "PSS", amount
        : 8u32, }, StaticRecipeMaterial { ticker : "NST", amount : 1u32, },
        StaticRecipeMaterial { ticker : "WIN", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "WS", amount : 1u32, }, StaticRecipeMaterial { ticker : "MED", amount :
        5u32, }, StaticRecipeMaterial { ticker : "DW", amount : 100u32, },
        StaticRecipeMaterial { ticker : "MEA", amount : 70u32, }, StaticRecipeMaterial {
        ticker : "LC", amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "CBU", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "PAC",
        recipe_name : "100xDW 70xFIM 5xMED 2xHSS 1xPDA 10xGIN 2xVG 4xPSS 160xPE=>1xEBU",
        standard_recipe_name :
        "PAC:100xDW-70xFIM-10xGIN-2xHSS-5xMED-1xPDA-160xPE-4xPSS-2xVG=>1xEBU", duration :
        std::time::Duration::from_millis(24192000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 160u32, }, StaticRecipeMaterial { ticker : "PSS", amount
        : 4u32, }, StaticRecipeMaterial { ticker : "DW", amount : 100u32, },
        StaticRecipeMaterial { ticker : "HSS", amount : 2u32, }, StaticRecipeMaterial {
        ticker : "PDA", amount : 1u32, }, StaticRecipeMaterial { ticker : "FIM", amount :
        70u32, }, StaticRecipeMaterial { ticker : "VG", amount : 2u32, },
        StaticRecipeMaterial { ticker : "MED", amount : 5u32, }, StaticRecipeMaterial {
        ticker : "GIN", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "EBU", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "PAC",
        recipe_name : "75xDW 70xRAT 5xMED 5xHMS 1xSCN 10xALE 1xSC 1xPSS 40xPE=>1xTBU",
        standard_recipe_name :
        "PAC:10xALE-75xDW-5xHMS-5xMED-40xPE-1xPSS-70xRAT-1xSC-1xSCN=>1xTBU", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DW", amount : 75u32, }, StaticRecipeMaterial { ticker : "HMS", amount :
        5u32, }, StaticRecipeMaterial { ticker : "SC", amount : 1u32, },
        StaticRecipeMaterial { ticker : "PSS", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "PE", amount : 40u32, }, StaticRecipeMaterial { ticker : "MED", amount :
        5u32, }, StaticRecipeMaterial { ticker : "RAT", amount : 70u32, },
        StaticRecipeMaterial { ticker : "SCN", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "ALE", amount : 10u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "TBU", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "PAC",
        recipe_name : "50xDW 60xRAT 5xEXO 5xPT 10xKOM 2xREP 100xPE=>1xSBU",
        standard_recipe_name : "PAC:50xDW-5xEXO-10xKOM-100xPE-5xPT-60xRAT-2xREP=>1xSBU",
        duration : std::time::Duration::from_millis(19008000u64), inputs : &
        [StaticRecipeMaterial { ticker : "RAT", amount : 60u32, }, StaticRecipeMaterial {
        ticker : "EXO", amount : 5u32, }, StaticRecipeMaterial { ticker : "PT", amount :
        5u32, }, StaticRecipeMaterial { ticker : "KOM", amount : 10u32, },
        StaticRecipeMaterial { ticker : "PE", amount : 100u32, }, StaticRecipeMaterial {
        ticker : "REP", amount : 2u32, }, StaticRecipeMaterial { ticker : "DW", amount :
        50u32, }], outputs : & [StaticRecipeMaterial { ticker : "SBU", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PAC", recipe_name :
        "40xDW 40xRAT 5xOVE 5xCOF 2xPWO 50xPE=>1xPBU", standard_recipe_name :
        "PAC:5xCOF-40xDW-5xOVE-50xPE-2xPWO-40xRAT=>1xPBU", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 50u32, }, StaticRecipeMaterial { ticker : "COF", amount :
        5u32, }, StaticRecipeMaterial { ticker : "RAT", amount : 40u32, },
        StaticRecipeMaterial { ticker : "DW", amount : 40u32, }, StaticRecipeMaterial {
        ticker : "OVE", amount : 5u32, }, StaticRecipeMaterial { ticker : "PWO", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "PBU", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PHF", recipe_name :
        "1xBSC 20xREA 4xMFK 2xDCM=>1xADR", standard_recipe_name :
        "PHF:1xBSC-2xDCM-4xMFK-20xREA=>1xADR", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "REA", amount : 20u32, }, StaticRecipeMaterial { ticker : "DCM", amount
        : 2u32, }, StaticRecipeMaterial { ticker : "BSC", amount : 1u32, },
        StaticRecipeMaterial { ticker : "MFK", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ADR", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PHF", recipe_name : "1xDCM 1xMFK 1xSAR=>1xBSC",
        standard_recipe_name : "PHF:1xDCM-1xMFK-1xSAR=>1xBSC", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "DCM", amount : 1u32, }, StaticRecipeMaterial { ticker : "MFK", amount :
        1u32, }, StaticRecipeMaterial { ticker : "SAR", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BSC", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PHF", recipe_name : "1xNL 1xSIL=>20xBND", standard_recipe_name
        : "PHF:1xNL-1xSIL=>20xBND", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SIL", amount : 1u32, }, StaticRecipeMaterial { ticker : "NL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BND", amount : 20u32, }],
        }, StaticRecipeInfo { building_ticker : "PHF", recipe_name :
        "2xCA 4xLI 10xREA=>10xPK", standard_recipe_name : "PHF:2xCA-4xLI-10xREA=>10xPK",
        duration : std::time::Duration::from_millis(28512000u64), inputs : &
        [StaticRecipeMaterial { ticker : "REA", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "CA", amount : 2u32, }, StaticRecipeMaterial { ticker : "LI", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "PK", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "POL", recipe_name :
        "1xC 1xH 1xCL 1xO=>50xEPO", standard_recipe_name :
        "POL:1xC-1xCL-1xH-1xO=>50xEPO", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CL", amount : 1u32, }, StaticRecipeMaterial { ticker : "H", amount :
        1u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, },
        StaticRecipeMaterial { ticker : "O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "EPO", amount : 50u32, }], }, StaticRecipeInfo {
        building_ticker : "POL", recipe_name : "1xH 1xC 1xMG=>50xPG",
        standard_recipe_name : "POL:1xC-1xH-1xMG=>50xPG", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MG", amount : 1u32, }, StaticRecipeMaterial { ticker : "H", amount :
        1u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PG", amount : 50u32, }], }, StaticRecipeInfo {
        building_ticker : "POL", recipe_name : "150xPG 70xEPO=>1xDEC",
        standard_recipe_name : "POL:70xEPO-150xPG=>1xDEC", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "EPO", amount : 70u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        150u32, }], outputs : & [StaticRecipeMaterial { ticker : "DEC", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "PP1", recipe_name : "150xPE=>1xBDE",
        standard_recipe_name : "PP1:150xPE=>1xBDE", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 150u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "BDE", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "PP1",
        recipe_name : "1xFE 2xLST=>1xBSE", standard_recipe_name :
        "PP1:1xFE-2xLST=>1xBSE", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LST", amount : 2u32, }, StaticRecipeMaterial { ticker : "FE", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BSE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP1", recipe_name : "1xFE 50xPE=>1xBTA",
        standard_recipe_name : "PP1:1xFE-50xPE=>1xBTA", duration :
        std::time::Duration::from_millis(12960000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }, StaticRecipeMaterial { ticker : "PE", amount :
        50u32, }], outputs : & [StaticRecipeMaterial { ticker : "BTA", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP1", recipe_name : "2xFE 1xLST=>1xBBH",
        standard_recipe_name : "PP1:2xFE-1xLST=>1xBBH", duration :
        std::time::Duration::from_millis(28512000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 2u32, }, StaticRecipeMaterial { ticker : "LST", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BBH", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "1xGL 1xAL=>1xBTA",
        standard_recipe_name : "PP2:1xAL-1xGL=>1xBTA", duration :
        std::time::Duration::from_millis(12960000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "GL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BTA", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "1xAL 2xLST=>1xBSE",
        standard_recipe_name : "PP2:1xAL-2xLST=>1xBSE", duration :
        std::time::Duration::from_millis(19008000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LST", amount : 2u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BSE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "1xAL 5xGL=>1xLTA",
        standard_recipe_name : "PP2:1xAL-5xGL=>1xLTA", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 1u32, }, StaticRecipeMaterial { ticker : "GL", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "LTA", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "50xPG 1xNE=>1xAEF",
        standard_recipe_name : "PP2:1xNE-50xPG=>1xAEF", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 50u32, }, StaticRecipeMaterial { ticker : "NE", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "AEF", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "2xAL 1xLST=>1xBBH",
        standard_recipe_name : "PP2:2xAL-1xLST=>1xBBH", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LST", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "BBH", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name :
        "120xPG 3xAL=>1xLSE", standard_recipe_name : "PP2:3xAL-120xPG=>1xLSE", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 3u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        120u32, }], outputs : & [StaticRecipeMaterial { ticker : "LSE", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "PP2", recipe_name :
        "3xAL 1xNL=>1xLDE", standard_recipe_name : "PP2:3xAL-1xNL=>1xLDE", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "NL", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        3u32, }], outputs : & [StaticRecipeMaterial { ticker : "LDE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "35xPE 3xAL=>1xLBH",
        standard_recipe_name : "PP2:3xAL-35xPE=>1xLBH", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 3u32, }, StaticRecipeMaterial { ticker : "PE", amount :
        35u32, }], outputs : & [StaticRecipeMaterial { ticker : "LBH", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP2", recipe_name : "40xPG=>1xBDE",
        standard_recipe_name : "PP2:40xPG=>1xBDE", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 40u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "BDE", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "PP3",
        recipe_name : "1xLDE 100xEPO 1xKV=>2xRDE", standard_recipe_name :
        "PP3:100xEPO-1xKV-1xLDE=>2xRDE", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "EPO", amount : 100u32, }, StaticRecipeMaterial { ticker : "KV", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "LDE", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RDE", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "PP3", recipe_name : "100xPE 1xAR 1xTHF=>24xINS",
        standard_recipe_name : "PP3:1xAR-100xPE-1xTHF=>24xINS", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 100u32, }, StaticRecipeMaterial { ticker : "AR", amount :
        1u32, }, StaticRecipeMaterial { ticker : "THF", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "INS", amount : 24u32, }], }, StaticRecipeInfo {
        building_ticker : "PP3", recipe_name : "1xBBH 1xSTL 50xEPO=>1xRBH",
        standard_recipe_name : "PP3:1xBBH-50xEPO-1xSTL=>1xRBH", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "EPO", amount : 50u32, }, StaticRecipeMaterial { ticker : "STL", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "BBH", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RBH", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PP3", recipe_name : "1xMAG 1xLDE=>1xMGC", standard_recipe_name
        : "PP3:1xLDE-1xMAG=>1xMGC", duration :
        std::time::Duration::from_millis(95040000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LDE", amount : 1u32, }, StaticRecipeMaterial { ticker : "MAG", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "MGC", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP3", recipe_name : "1xLTA 6xRG=>1xRTA",
        standard_recipe_name : "PP3:1xLTA-6xRG=>1xRTA", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RG", amount : 6u32, }, StaticRecipeMaterial { ticker : "LTA", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "RTA", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP3", recipe_name :
        "1xTI 250xNFI=>1xPSH", standard_recipe_name : "PP3:250xNFI-1xTI=>1xPSH", duration
        : std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial
        { ticker : "NFI", amount : 250u32, }, StaticRecipeMaterial { ticker : "TI",
        amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "PSH", amount :
        1u32, }], }, StaticRecipeInfo { building_ticker : "PP3", recipe_name :
        "2xBSE 1xSTL 225xEPO=>2xRSE", standard_recipe_name :
        "PP3:2xBSE-225xEPO-1xSTL=>2xRSE", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "STL", amount : 1u32, }, StaticRecipeMaterial { ticker : "BSE", amount :
        2u32, }, StaticRecipeMaterial { ticker : "EPO", amount : 225u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RSE", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "PP3", recipe_name : "2xLSE 1xTCS=>2xHSE", standard_recipe_name
        : "PP3:2xLSE-1xTCS=>2xHSE", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TCS", amount : 1u32, }, StaticRecipeMaterial { ticker : "LSE", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "HSE", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "PP4", recipe_name :
        "2xRBH 125xNR=>2xABH", standard_recipe_name : "PP4:125xNR-2xRBH=>2xABH", duration
        : std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial
        { ticker : "NR", amount : 125u32, }, StaticRecipeMaterial { ticker : "RBH",
        amount : 2u32, }], outputs : & [StaticRecipeMaterial { ticker : "ABH", amount :
        2u32, }], }, StaticRecipeInfo { building_ticker : "PP4", recipe_name :
        "1xTA 1xSTL 1xLST=>1xRSH", standard_recipe_name : "PP4:1xLST-1xSTL-1xTA=>1xRSH",
        duration : std::time::Duration::from_millis(77760000u64), inputs : &
        [StaticRecipeMaterial { ticker : "TA", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "LST", amount : 1u32, }, StaticRecipeMaterial { ticker : "STL", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "RSH", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP4", recipe_name : "1xRTA 1xNG=>1xATA",
        standard_recipe_name : "PP4:1xNG-1xRTA=>1xATA", duration :
        std::time::Duration::from_millis(47520000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RTA", amount : 1u32, }, StaticRecipeMaterial { ticker : "NG", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "ATA", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP4", recipe_name : "1xRSE 2xTI=>1xASE",
        standard_recipe_name : "PP4:1xRSE-2xTI=>1xASE", duration :
        std::time::Duration::from_millis(47520000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RSE", amount : 1u32, }, StaticRecipeMaterial { ticker : "TI", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "ASE", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "PP4", recipe_name : "2xLDE 2xKV=>2xADE",
        standard_recipe_name : "PP4:2xKV-2xLDE=>2xADE", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LDE", amount : 2u32, }, StaticRecipeMaterial { ticker : "KV", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "ADE", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "PP4", recipe_name :
        "150xPE 2xLBH 2xTHP=>1xTSH", standard_recipe_name :
        "PP4:2xLBH-150xPE-2xTHP=>1xTSH", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "THP", amount : 2u32, }, StaticRecipeMaterial { ticker : "PE", amount :
        150u32, }, StaticRecipeMaterial { ticker : "LBH", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "TSH", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PP4", recipe_name : "4xPSH 8xHSE 100xMFK=>1xTRS",
        standard_recipe_name : "PP4:8xHSE-100xMFK-4xPSH=>1xTRS", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "HSE", amount : 8u32, }, StaticRecipeMaterial { ticker : "MFK", amount
        : 100u32, }, StaticRecipeMaterial { ticker : "PSH", amount : 4u32, }], outputs :
        & [StaticRecipeMaterial { ticker : "TRS", amount : 1u32, }], }, StaticRecipeInfo
        { building_ticker : "PPF", recipe_name : "4xSFK 10xPG 1xPSS=>2xDCS",
        standard_recipe_name : "PPF:10xPG-1xPSS-4xSFK=>2xDCS", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 10u32, }, StaticRecipeMaterial { ticker : "SFK", amount :
        4u32, }, StaticRecipeMaterial { ticker : "PSS", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "DCS", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "PPF", recipe_name : "10xPG=>1xPSS", standard_recipe_name :
        "PPF:10xPG=>1xPSS", duration : std::time::Duration::from_millis(17280000u64),
        inputs : & [StaticRecipeMaterial { ticker : "PG", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PSS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PPF", recipe_name : "2xPSM 8xSFK 20xPG=>2xDCM",
        standard_recipe_name : "PPF:20xPG-2xPSM-8xSFK=>2xDCM", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 20u32, }, StaticRecipeMaterial { ticker : "PSM", amount :
        2u32, }, StaticRecipeMaterial { ticker : "SFK", amount : 8u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "DCM", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "PPF", recipe_name : "20xPG=>1xPSM", standard_recipe_name :
        "PPF:20xPG=>1xPSM", duration : std::time::Duration::from_millis(34560000u64),
        inputs : & [StaticRecipeMaterial { ticker : "PG", amount : 20u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PSM", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PPF", recipe_name : "2xPSL 2xMFK 40xPG=>2xDCL",
        standard_recipe_name : "PPF:2xMFK-40xPG-2xPSL=>2xDCL", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 2u32, }, StaticRecipeMaterial { ticker : "PSL", amount :
        2u32, }, StaticRecipeMaterial { ticker : "PG", amount : 40u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "DCL", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "PPF", recipe_name : "40xPG=>1xPSL", standard_recipe_name :
        "PPF:40xPG=>1xPSL", duration : std::time::Duration::from_millis(51840000u64),
        inputs : & [StaticRecipeMaterial { ticker : "PG", amount : 40u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PSL", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "PPF", recipe_name : "1xSFK 50xPG 10xTHF=>1xLFP",
        standard_recipe_name : "PPF:50xPG-1xSFK-10xTHF=>1xLFP", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "THF", amount : 10u32, }, StaticRecipeMaterial { ticker : "SFK", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "PG", amount : 50u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "LFP", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "REF", recipe_name : "1xAMM 2xGAL 3xH=>100xSF",
        standard_recipe_name : "REF:1xAMM-2xGAL-3xH=>100xSF", duration :
        std::time::Duration::from_millis(12960000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H", amount : 3u32, }, StaticRecipeMaterial { ticker : "AMM", amount :
        1u32, }, StaticRecipeMaterial { ticker : "GAL", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SF", amount : 100u32, }], }, StaticRecipeInfo {
        building_ticker : "REF", recipe_name : "1xAMM 5xNAB=>150xSF",
        standard_recipe_name : "REF:1xAMM-5xNAB=>150xSF", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AMM", amount : 1u32, }, StaticRecipeMaterial { ticker : "NAB", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "SF", amount : 150u32, }],
        }, StaticRecipeInfo { building_ticker : "REF", recipe_name : "3xTS=>2xHE3",
        standard_recipe_name : "REF:3xTS=>2xHE3", duration :
        std::time::Duration::from_millis(21600000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TS", amount : 3u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "HE3", amount : 2u32, }], }, StaticRecipeInfo { building_ticker : "REF",
        recipe_name : "2xHE3 4xH=>100xFF", standard_recipe_name :
        "REF:4xH-2xHE3=>100xFF", duration :
        std::time::Duration::from_millis(12960000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HE3", amount : 2u32, }, StaticRecipeMaterial { ticker : "H", amount :
        4u32, }], outputs : & [StaticRecipeMaterial { ticker : "FF", amount : 100u32, }],
        }, StaticRecipeInfo { building_ticker : "REF", recipe_name :
        "2xKRE 4xH=>2000xVF", standard_recipe_name : "REF:4xH-2xKRE=>2000xVF", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "H", amount : 4u32, }, StaticRecipeMaterial { ticker : "KRE", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "VF", amount : 2000u32,
        }], }, StaticRecipeInfo { building_ticker : "RIG", recipe_name : "=>",
        standard_recipe_name : "RIG:=>", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [], outputs : & [], },
        StaticRecipeInfo { building_ticker : "SCA", recipe_name :
        "10xSWF 10xTRN 10xBCO=>10xMPC", standard_recipe_name :
        "SCA:10xBCO-10xSWF-10xTRN=>10xMPC", duration :
        std::time::Duration::from_millis(36288000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TRN", amount : 10u32, }, StaticRecipeMaterial { ticker : "SWF", amount
        : 10u32, }, StaticRecipeMaterial { ticker : "BCO", amount : 10u32, }], outputs :
        & [StaticRecipeMaterial { ticker : "MPC", amount : 10u32, }], }, StaticRecipeInfo
        { building_ticker : "SCA", recipe_name : "10xPSS 10xBGC 10xSI=>10xROM",
        standard_recipe_name : "SCA:10xBGC-10xPSS-10xSI=>10xROM", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PSS", amount : 10u32, }, StaticRecipeMaterial { ticker : "BGC", amount
        : 10u32, }, StaticRecipeMaterial { ticker : "SI", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ROM", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "SCA", recipe_name : "20xTRN 10xH 10xN=>20xSEN",
        standard_recipe_name : "SCA:10xH-10xN-20xTRN=>20xSEN", duration :
        std::time::Duration::from_millis(25056000u64), inputs : & [StaticRecipeMaterial {
        ticker : "N", amount : 10u32, }, StaticRecipeMaterial { ticker : "TRN", amount :
        20u32, }, StaticRecipeMaterial { ticker : "H", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SEN", amount : 20u32, }], }, StaticRecipeInfo {
        building_ticker : "SCA", recipe_name : "4xMWF 4xTRN 4xCAP 4xHCC=>4xTPU",
        standard_recipe_name : "SCA:4xCAP-4xHCC-4xMWF-4xTRN=>4xTPU", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MWF", amount : 4u32, }, StaticRecipeMaterial { ticker : "CAP", amount :
        4u32, }, StaticRecipeMaterial { ticker : "HCC", amount : 4u32, },
        StaticRecipeMaterial { ticker : "TRN", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "TPU", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "SCA", recipe_name : "10xSWF 5xBCO 60xPE 1xBGO=>5xPCB",
        standard_recipe_name : "SCA:5xBCO-1xBGO-60xPE-10xSWF=>5xPCB", duration :
        std::time::Duration::from_millis(24192000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PE", amount : 60u32, }, StaticRecipeMaterial { ticker : "BGO", amount :
        1u32, }, StaticRecipeMaterial { ticker : "BCO", amount : 5u32, },
        StaticRecipeMaterial { ticker : "SWF", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "PCB", amount : 5u32, }], }, StaticRecipeInfo {
        building_ticker : "SCA", recipe_name : "6xPSS 6xCAP 6xBCO=>6xRAM",
        standard_recipe_name : "SCA:6xBCO-6xCAP-6xPSS=>6xRAM", duration :
        std::time::Duration::from_millis(36288000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PSS", amount : 6u32, }, StaticRecipeMaterial { ticker : "BCO", amount :
        6u32, }, StaticRecipeMaterial { ticker : "CAP", amount : 6u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RAM", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "SD", recipe_name : "=>1xBAI", standard_recipe_name :
        "SD:=>1xBAI", duration : std::time::Duration::from_millis(172800000u64), inputs :
        & [], outputs : & [StaticRecipeMaterial { ticker : "BAI", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SD", recipe_name : "=>1xLD",
        standard_recipe_name : "SD:=>1xLD", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [], outputs : &
        [StaticRecipeMaterial { ticker : "LD", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SD", recipe_name : "=>1xMLI", standard_recipe_name :
        "SD:=>1xMLI", duration : std::time::Duration::from_millis(259200000u64), inputs :
        & [], outputs : & [StaticRecipeMaterial { ticker : "MLI", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SD", recipe_name : "=>1xNF",
        standard_recipe_name : "SD:=>1xNF", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [], outputs : &
        [StaticRecipeMaterial { ticker : "NF", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SD", recipe_name : "=>1xSA", standard_recipe_name :
        "SD:=>1xSA", duration : std::time::Duration::from_millis(86400000u64), inputs : &
        [], outputs : & [StaticRecipeMaterial { ticker : "SA", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SD", recipe_name : "=>1xSAL",
        standard_recipe_name : "SD:=>1xSAL", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [], outputs : &
        [StaticRecipeMaterial { ticker : "SAL", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SD", recipe_name : "=>1xWM", standard_recipe_name :
        "SD:=>1xWM", duration : std::time::Duration::from_millis(172800000u64), inputs :
        & [], outputs : & [StaticRecipeMaterial { ticker : "WM", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SE", recipe_name : "1xBAI 1xMLI=>1xNN",
        standard_recipe_name : "SE:1xBAI-1xMLI=>1xNN", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "MLI", amount : 1u32, }, StaticRecipeMaterial { ticker : "BAI", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "NN", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SE", recipe_name : "1xNF 1xLD=>1xDD",
        standard_recipe_name : "SE:1xLD-1xNF=>1xDD", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "LD", amount : 1u32, }, StaticRecipeMaterial { ticker : "NF", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "DD", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SE", recipe_name :
        "1xSAL 1xLD 1xROM 1xSA=>1xDA", standard_recipe_name :
        "SE:1xLD-1xROM-1xSA-1xSAL=>1xDA", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "ROM", amount : 1u32, }, StaticRecipeMaterial { ticker : "SA", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "LD", amount : 1u32, },
        StaticRecipeMaterial { ticker : "SAL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "DA", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SE", recipe_name : "1xLD 1xWM 1xROM=>1xOS",
        standard_recipe_name : "SE:1xLD-1xROM-1xWM=>1xOS", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "ROM", amount : 1u32, }, StaticRecipeMaterial { ticker : "WM", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "LD", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "OS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SE", recipe_name : "=>1xDV", standard_recipe_name :
        "SE:=>1xDV", duration : std::time::Duration::from_millis(129600000u64), inputs :
        & [], outputs : & [StaticRecipeMaterial { ticker : "DV", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SE", recipe_name : "=>1xEDC",
        standard_recipe_name : "SE:=>1xEDC", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [], outputs : &
        [StaticRecipeMaterial { ticker : "EDC", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SKF", recipe_name : "100xAST 100xFET 20xMFK=>1xWCB",
        standard_recipe_name : "SKF:100xAST-100xFET-20xMFK=>1xWCB", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AST", amount : 100u32, }, StaticRecipeMaterial { ticker : "FET", amount
        : 100u32, }, StaticRecipeMaterial { ticker : "MFK", amount : 20u32, }], outputs :
        & [StaticRecipeMaterial { ticker : "WCB", amount : 1u32, }], }, StaticRecipeInfo
        { building_ticker : "SKF", recipe_name : "100xAST 16xMFK=>1xMCB",
        standard_recipe_name : "SKF:100xAST-16xMFK=>1xMCB", duration :
        std::time::Duration::from_millis(56160000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 16u32, }, StaticRecipeMaterial { ticker : "AST", amount
        : 100u32, }], outputs : & [StaticRecipeMaterial { ticker : "MCB", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "600xWAL 100xMFK=>1xHCB", standard_recipe_name : "SKF:100xMFK-600xWAL=>1xHCB",
        duration : std::time::Duration::from_millis(172800000u64), inputs : &
        [StaticRecipeMaterial { ticker : "WAL", amount : 600u32, }, StaticRecipeMaterial
        { ticker : "MFK", amount : 100u32, }], outputs : & [StaticRecipeMaterial { ticker
        : "HCB", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SKF",
        recipe_name : "125xFET 8xMFK=>1xLSL", standard_recipe_name :
        "SKF:125xFET-8xMFK=>1xLSL", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 8u32, }, StaticRecipeMaterial { ticker : "FET", amount :
        125u32, }], outputs : & [StaticRecipeMaterial { ticker : "LSL", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "5xZR 1xMFK=>1xSFL", standard_recipe_name : "SKF:1xMFK-5xZR=>1xSFL", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 1u32, }, StaticRecipeMaterial { ticker : "ZR", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "SFL", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "200xAST 20xMFK=>1xLCB", standard_recipe_name : "SKF:200xAST-20xMFK=>1xLCB",
        duration : std::time::Duration::from_millis(64800000u64), inputs : &
        [StaticRecipeMaterial { ticker : "AST", amount : 200u32, }, StaticRecipeMaterial
        { ticker : "MFK", amount : 20u32, }], outputs : & [StaticRecipeMaterial { ticker
        : "LCB", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SKF",
        recipe_name : "200xFET 20xMFK=>1xVCB", standard_recipe_name :
        "SKF:200xFET-20xMFK=>1xVCB", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 20u32, }, StaticRecipeMaterial { ticker : "FET", amount
        : 200u32, }], outputs : & [StaticRecipeMaterial { ticker : "VCB", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "20xFE 4xMFK=>1xTCB", standard_recipe_name : "SKF:20xFE-4xMFK=>1xTCB", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 4u32, }, StaticRecipeMaterial { ticker : "FE", amount :
        20u32, }], outputs : & [StaticRecipeMaterial { ticker : "TCB", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "50xZR 24xMFK 80xSRP=>1xVFT", standard_recipe_name :
        "SKF:24xMFK-80xSRP-50xZR=>1xVFT", duration :
        std::time::Duration::from_millis(259200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "ZR", amount : 50u32, }, StaticRecipeMaterial { ticker : "SRP", amount
        : 80u32, }, StaticRecipeMaterial { ticker : "MFK", amount : 24u32, }], outputs :
        & [StaticRecipeMaterial { ticker : "VFT", amount : 1u32, }], }, StaticRecipeInfo
        { building_ticker : "SKF", recipe_name : "20xTI 2xMFK=>1xSSL",
        standard_recipe_name : "SKF:2xMFK-20xTI=>1xSSL", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TI", amount : 20u32, }, StaticRecipeMaterial { ticker : "MFK", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "SSL", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "35xFE 8xMFK=>1xVSC", standard_recipe_name : "SKF:35xFE-8xMFK=>1xVSC", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 35u32, }, StaticRecipeMaterial { ticker : "MFK", amount :
        8u32, }], outputs : & [StaticRecipeMaterial { ticker : "VSC", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "10xZR 4xMFK=>1xMFL", standard_recipe_name : "SKF:4xMFK-10xZR=>1xMFL", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "MFK", amount : 4u32, }, StaticRecipeMaterial { ticker : "ZR", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "MFL", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "50xFAL 12xMFK=>1xSCB", standard_recipe_name : "SKF:50xFAL-12xMFK=>1xSCB",
        duration : std::time::Duration::from_millis(51840000u64), inputs : &
        [StaticRecipeMaterial { ticker : "MFK", amount : 12u32, }, StaticRecipeMaterial {
        ticker : "FAL", amount : 50u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "SCB", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SKF",
        recipe_name : "50xFET 4xMFK=>1xMSL", standard_recipe_name :
        "SKF:50xFET-4xMFK=>1xMSL", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FET", amount : 50u32, }, StaticRecipeMaterial { ticker : "MFK", amount
        : 4u32, }], outputs : & [StaticRecipeMaterial { ticker : "MSL", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "SKF", recipe_name :
        "20xZR 8xMFK=>1xLFL", standard_recipe_name : "SKF:8xMFK-20xZR=>1xLFL", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ZR", amount : 20u32, }, StaticRecipeMaterial { ticker : "MFK", amount :
        8u32, }], outputs : & [StaticRecipeMaterial { ticker : "LFL", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SL", recipe_name : "1xDA 1xDD=>1xIMM",
        standard_recipe_name : "SL:1xDA-1xDD=>1xIMM", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "DD", amount : 1u32, }, StaticRecipeMaterial { ticker : "DA", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "IMM", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "SL", recipe_name :
        "1xWAI 1xDV 1xIMM=>2xSNM", standard_recipe_name : "SL:1xDV-1xIMM-1xWAI=>2xSNM",
        duration : std::time::Duration::from_millis(172800000u64), inputs : &
        [StaticRecipeMaterial { ticker : "WAI", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "DV", amount : 1u32, }, StaticRecipeMaterial { ticker : "IMM", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "SNM", amount : 2u32, }],
        }, StaticRecipeInfo { building_ticker : "SL", recipe_name : "1xNN 1xROM=>1xWAI",
        standard_recipe_name : "SL:1xNN-1xROM=>1xWAI", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "NN", amount : 1u32, }, StaticRecipeMaterial { ticker : "ROM", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "WAI", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "SL", recipe_name : "=>1xIDC",
        standard_recipe_name : "SL:=>1xIDC", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [], outputs : &
        [StaticRecipeMaterial { ticker : "IDC", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "4xTS 1xO 1xAL=>1xSI",
        standard_recipe_name : "SME:1xAL-1xO-4xTS=>1xSI", duration :
        std::time::Duration::from_millis(10368000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }, StaticRecipeMaterial { ticker : "TS", amount :
        4u32, }, StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SI", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "3xSIO 1xAL=>1xSI", standard_recipe_name :
        "SME:1xAL-3xSIO=>1xSI", duration : std::time::Duration::from_millis(12960000u64),
        inputs : & [StaticRecipeMaterial { ticker : "SIO", amount : 3u32, },
        StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SI", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "3xSIO 1xC 1xO 1xFLX=>1xSI",
        standard_recipe_name : "SME:1xC-1xFLX-1xO-3xSIO=>1xSI", duration :
        std::time::Duration::from_millis(10368000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SIO", amount : 3u32, }, StaticRecipeMaterial { ticker : "FLX", amount :
        1u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, },
        StaticRecipeMaterial { ticker : "O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SI", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "8xREO 1xC 1xO 1xFLX=>5xRE",
        standard_recipe_name : "SME:1xC-1xFLX-1xO-8xREO=>5xRE", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "C", amount : 1u32, }, StaticRecipeMaterial { ticker : "FLX", amount :
        1u32, }, StaticRecipeMaterial { ticker : "REO", amount : 8u32, },
        StaticRecipeMaterial { ticker : "O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RE", amount : 5u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "3xSIO 1xC 1xO=>1xSI",
        standard_recipe_name : "SME:1xC-1xO-3xSIO=>1xSI", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "C", amount : 1u32, }, StaticRecipeMaterial { ticker : "O", amount :
        1u32, }, StaticRecipeMaterial { ticker : "SIO", amount : 3u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "SI", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "4xTIO 1xC 1xO=>2xTI",
        standard_recipe_name : "SME:1xC-1xO-4xTIO=>2xTI", duration :
        std::time::Duration::from_millis(64800000u64), inputs : & [StaticRecipeMaterial {
        ticker : "C", amount : 1u32, }, StaticRecipeMaterial { ticker : "TIO", amount :
        4u32, }, StaticRecipeMaterial { ticker : "O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "TI", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "8xREO 1xC 1xO=>4xRE",
        standard_recipe_name : "SME:1xC-1xO-8xREO=>4xRE", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "REO", amount : 8u32, }, StaticRecipeMaterial { ticker : "O", amount :
        1u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "RE", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "6xFEO 1xC 1xO 1xFLX=>4xFE",
        standard_recipe_name : "SME:1xC-6xFEO-1xFLX-1xO=>4xFE", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }, StaticRecipeMaterial { ticker : "FEO", amount :
        6u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, },
        StaticRecipeMaterial { ticker : "FLX", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FE", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "6xFEO 1xC 1xO=>3xFE",
        standard_recipe_name : "SME:1xC-6xFEO-1xO=>3xFE", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FEO", amount : 6u32, }, StaticRecipeMaterial { ticker : "C", amount :
        1u32, }, StaticRecipeMaterial { ticker : "O", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FE", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "1xSCR 1xO=>6xS", standard_recipe_name :
        "SME:1xO-1xSCR=>6xS", duration : std::time::Duration::from_millis(25920000u64),
        inputs : & [StaticRecipeMaterial { ticker : "O", amount : 1u32, },
        StaticRecipeMaterial { ticker : "SCR", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "S", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "2xFE 8xO=>2xSTL", standard_recipe_name :
        "SME:2xFE-8xO=>2xSTL", duration : std::time::Duration::from_millis(30240000u64),
        inputs : & [StaticRecipeMaterial { ticker : "O", amount : 8u32, },
        StaticRecipeMaterial { ticker : "FE", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "STL", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "1xC 3xAUO=>2xAU", standard_recipe_name :
        "SME:3xAUO-1xC=>2xAU", duration : std::time::Duration::from_millis(38880000u64),
        inputs : & [StaticRecipeMaterial { ticker : "AUO", amount : 3u32, },
        StaticRecipeMaterial { ticker : "C", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AU", amount : 2u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "4xAL 1xSI 2xO=>1xCF",
        standard_recipe_name : "SME:4xAL-2xO-1xSI=>1xCF", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 2u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        4u32, }, StaticRecipeMaterial { ticker : "SI", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CF", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "10xLIO 4xHAL=>4xLI", standard_recipe_name
        : "SME:4xHAL-10xLIO=>4xLI", duration :
        std::time::Duration::from_millis(57024000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HAL", amount : 4u32, }, StaticRecipeMaterial { ticker : "LIO", amount :
        10u32, }], outputs : & [StaticRecipeMaterial { ticker : "LI", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "SME", recipe_name :
        "5xCUO 10xO 1xSIO=>3xCU", standard_recipe_name : "SME:5xCUO-10xO-1xSIO=>3xCU",
        duration : std::time::Duration::from_millis(43200000u64), inputs : &
        [StaticRecipeMaterial { ticker : "O", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "SIO", amount : 1u32, }, StaticRecipeMaterial { ticker : "CUO", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "CU", amount : 3u32, }],
        }, StaticRecipeInfo { building_ticker : "SME", recipe_name :
        "6xALO 1xO 1xC 1xFLX=>4xAL", standard_recipe_name :
        "SME:6xALO-1xC-1xFLX-1xO=>4xAL", duration :
        std::time::Duration::from_millis(51840000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }, StaticRecipeMaterial { ticker : "FLX", amount :
        1u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, },
        StaticRecipeMaterial { ticker : "ALO", amount : 6u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AL", amount : 4u32, }], }, StaticRecipeInfo {
        building_ticker : "SME", recipe_name : "6xALO 1xC 1xO=>3xAL",
        standard_recipe_name : "SME:6xALO-1xC-1xO=>3xAL", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }, StaticRecipeMaterial { ticker : "ALO", amount :
        6u32, }, StaticRecipeMaterial { ticker : "C", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AL", amount : 3u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "1xRAG 10xAST 1xCBS=>1xRCT",
        standard_recipe_name : "SPF:10xAST-1xCBS-1xRAG=>1xRCT", duration :
        std::time::Duration::from_millis(224640000u64), inputs : & [StaticRecipeMaterial
        { ticker : "AST", amount : 10u32, }, StaticRecipeMaterial { ticker : "CBS",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "RAG", amount : 1u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "RCT", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SPF", recipe_name :
        "1xSFK 50xPG 10xATP=>1xAFP", standard_recipe_name :
        "SPF:10xATP-50xPG-1xSFK=>1xAFP", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ATP", amount : 10u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        50u32, }, StaticRecipeMaterial { ticker : "SFK", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AFP", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "4xBFP 4xNOZ 10xAST 1xACS 1xMFK=>1xENG",
        standard_recipe_name : "SPF:1xACS-10xAST-4xBFP-1xMFK-4xNOZ=>1xENG", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "ACS", amount : 1u32, }, StaticRecipeMaterial { ticker : "AST", amount :
        10u32, }, StaticRecipeMaterial { ticker : "NOZ", amount : 4u32, },
        StaticRecipeMaterial { ticker : "BFP", amount : 4u32, }, StaticRecipeMaterial {
        ticker : "MFK", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "ENG", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SPF",
        recipe_name : "6xLFP 6xNOZ 25xBRO 1xACS 1xMFK=>1xFSE", standard_recipe_name :
        "SPF:1xACS-25xBRO-6xLFP-1xMFK-6xNOZ=>1xFSE", duration :
        std::time::Duration::from_millis(86400000u64), inputs : & [StaticRecipeMaterial {
        ticker : "BRO", amount : 25u32, }, StaticRecipeMaterial { ticker : "LFP", amount
        : 6u32, }, StaticRecipeMaterial { ticker : "NOZ", amount : 6u32, },
        StaticRecipeMaterial { ticker : "ACS", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "MFK", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "FSE", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SPF",
        recipe_name : "4xAFP 4xANZ 20xFET 1xACS 1xMFK=>1xAEN", standard_recipe_name :
        "SPF:1xACS-4xAFP-4xANZ-20xFET-1xMFK=>1xAEN", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "ANZ", amount : 4u32, }, StaticRecipeMaterial { ticker : "AFP", amount
        : 4u32, }, StaticRecipeMaterial { ticker : "ACS", amount : 1u32, },
        StaticRecipeMaterial { ticker : "FET", amount : 20u32, }, StaticRecipeMaterial {
        ticker : "MFK", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "AEN", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SPF",
        recipe_name : "8xAFP 4xHNZ 20xWAL 1xACS 1xMFK=>1xHTE", standard_recipe_name :
        "SPF:1xACS-8xAFP-4xHNZ-1xMFK-20xWAL=>1xHTE", duration :
        std::time::Duration::from_millis(345600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "HNZ", amount : 4u32, }, StaticRecipeMaterial { ticker : "WAL", amount
        : 20u32, }, StaticRecipeMaterial { ticker : "ACS", amount : 1u32, },
        StaticRecipeMaterial { ticker : "MFK", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "AFP", amount : 8u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "HTE", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "SPF",
        recipe_name : "1xVOR 40xWAL 1xCBL=>1xVOE", standard_recipe_name :
        "SPF:1xCBL-1xVOR-40xWAL=>1xVOE", duration :
        std::time::Duration::from_millis(864000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "VOR", amount : 1u32, }, StaticRecipeMaterial { ticker : "WAL", amount
        : 40u32, }, StaticRecipeMaterial { ticker : "CBL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "VOE", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "2xFIR 20xWAL 1xCBL=>1xHYR",
        standard_recipe_name : "SPF:1xCBL-2xFIR-20xWAL=>1xHYR", duration :
        std::time::Duration::from_millis(604800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FIR", amount : 2u32, }, StaticRecipeMaterial { ticker : "CBL", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "WAL", amount : 20u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HYR", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "1xFIR 20xSTL 1xCBM=>1xHPR",
        standard_recipe_name : "SPF:1xCBM-1xFIR-20xSTL=>1xHPR", duration :
        std::time::Duration::from_millis(432000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FIR", amount : 1u32, }, StaticRecipeMaterial { ticker : "STL", amount
        : 20u32, }, StaticRecipeMaterial { ticker : "CBM", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HPR", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "10xFAL 2xMFK 1xCHA=>1xNOZ",
        standard_recipe_name : "SPF:1xCHA-10xFAL-2xMFK=>1xNOZ", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CHA", amount : 1u32, }, StaticRecipeMaterial { ticker : "FAL", amount :
        10u32, }, StaticRecipeMaterial { ticker : "MFK", amount : 2u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "NOZ", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "20xFET 4xMFK 1xCHA=>1xANZ",
        standard_recipe_name : "SPF:1xCHA-20xFET-4xMFK=>1xANZ", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "CHA", amount : 1u32, }, StaticRecipeMaterial { ticker : "FET", amount :
        20u32, }, StaticRecipeMaterial { ticker : "MFK", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ANZ", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "20xWAL 4xMFK 1xCHA=>1xHNZ",
        standard_recipe_name : "SPF:1xCHA-4xMFK-20xWAL=>1xHNZ", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "WAL", amount : 20u32, }, StaticRecipeMaterial { ticker : "CHA", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "MFK", amount : 4u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HNZ", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPF", recipe_name : "1xRAG 20xBGO 1xCBS=>1xQCR",
        standard_recipe_name : "SPF:20xBGO-1xCBS-1xRAG=>1xQCR", duration :
        std::time::Duration::from_millis(259200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "BGO", amount : 20u32, }, StaticRecipeMaterial { ticker : "RAG",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "CBS", amount : 1u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "QCR", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "SPF", recipe_name :
        "1xSFK 50xPG 5xTHP=>1xBFP", standard_recipe_name :
        "SPF:50xPG-1xSFK-5xTHP=>1xBFP", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "SFK", amount : 1u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        50u32, }, StaticRecipeMaterial { ticker : "THP", amount : 5u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BFP", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPP", recipe_name : "80xPG 14xFAL 4xKV=>1xBGS",
        standard_recipe_name : "SPP:14xFAL-4xKV-80xPG=>1xBGS", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "KV", amount : 4u32, }, StaticRecipeMaterial { ticker : "PG", amount :
        80u32, }, StaticRecipeMaterial { ticker : "FAL", amount : 14u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BGS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPP", recipe_name : "100xPG 20xAST 8xTK=>1xAGS",
        standard_recipe_name : "SPP:20xAST-100xPG-8xTK=>1xAGS", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "AST", amount : 20u32, }, StaticRecipeMaterial { ticker : "TK", amount
        : 8u32, }, StaticRecipeMaterial { ticker : "PG", amount : 100u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "AGS", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "SPP", recipe_name : "5xTHP 20xTHF=>12xBPT",
        standard_recipe_name : "SPP:20xTHF-5xTHP=>12xBPT", duration :
        std::time::Duration::from_millis(30240000u64), inputs : & [StaticRecipeMaterial {
        ticker : "THP", amount : 5u32, }, StaticRecipeMaterial { ticker : "THF", amount :
        20u32, }], outputs : & [StaticRecipeMaterial { ticker : "BPT", amount : 12u32,
        }], }, StaticRecipeInfo { building_ticker : "SPP", recipe_name :
        "5xLST 2xKV=>12xBRP", standard_recipe_name : "SPP:2xKV-5xLST=>12xBRP", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "KV", amount : 2u32, }, StaticRecipeMaterial { ticker : "LST", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "BRP", amount : 12u32, }],
        }, StaticRecipeInfo { building_ticker : "SPP", recipe_name :
        "5xATP 30xTHF=>12xAPT", standard_recipe_name : "SPP:5xATP-30xTHF=>12xAPT",
        duration : std::time::Duration::from_millis(43200000u64), inputs : &
        [StaticRecipeMaterial { ticker : "ATP", amount : 5u32, }, StaticRecipeMaterial {
        ticker : "THF", amount : 30u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "APT", amount : 12u32, }], }, StaticRecipeInfo { building_ticker : "SPP",
        recipe_name : "5xLST 2xTK=>10xARP", standard_recipe_name :
        "SPP:5xLST-2xTK=>10xARP", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "LST", amount : 5u32, }, StaticRecipeMaterial { ticker : "TK", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "ARP", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "SPP", recipe_name :
        "5xLST 5xW 5xTA=>10xSRP", standard_recipe_name : "SPP:5xLST-5xTA-5xW=>10xSRP",
        duration : std::time::Duration::from_millis(43200000u64), inputs : &
        [StaticRecipeMaterial { ticker : "LST", amount : 5u32, }, StaticRecipeMaterial {
        ticker : "TA", amount : 5u32, }, StaticRecipeMaterial { ticker : "W", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "SRP", amount : 10u32, }],
        }, StaticRecipeInfo { building_ticker : "TNP", recipe_name : "1xTC=>3xTCS",
        standard_recipe_name : "TNP:1xTC=>3xTCS", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TC", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "TCS", amount : 3u32, }], }, StaticRecipeInfo { building_ticker : "TNP",
        recipe_name : "1xTCO=>1xTC 1xO", standard_recipe_name : "TNP:1xTCO=>1xO-1xTC",
        duration : std::time::Duration::from_millis(58752000u64), inputs : &
        [StaticRecipeMaterial { ticker : "TCO", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "TC", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "O", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "TNP",
        recipe_name : "1xTC 4xREA 4xFLX=>1xETC", standard_recipe_name :
        "TNP:4xFLX-4xREA-1xTC=>1xETC", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FLX", amount : 4u32, }, StaticRecipeMaterial { ticker : "REA", amount :
        4u32, }, StaticRecipeMaterial { ticker : "TC", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "ETC", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "UPF", recipe_name :
        "10xPSL 20xPSM 10xTI 1xLIS 10xDEC 1xTCU=>1xCQM", standard_recipe_name :
        "UPF:10xDEC-1xLIS-10xPSL-20xPSM-1xTCU-10xTI=>1xCQM", duration :
        std::time::Duration::from_millis(216000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "DEC", amount : 10u32, }, StaticRecipeMaterial { ticker : "TCU",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "PSM", amount : 20u32, },
        StaticRecipeMaterial { ticker : "PSL", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "TI", amount : 10u32, }, StaticRecipeMaterial { ticker : "LIS", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CQM", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "10xPSL 20xPSM 10xTI 5xSAR 4xRAD=>1xBRS", standard_recipe_name :
        "UPF:10xPSL-20xPSM-4xRAD-5xSAR-10xTI=>1xBRS", duration :
        std::time::Duration::from_millis(259200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSM", amount : 20u32, }, StaticRecipeMaterial { ticker : "SAR",
        amount : 5u32, }, StaticRecipeMaterial { ticker : "PSL", amount : 10u32, },
        StaticRecipeMaterial { ticker : "TI", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "RAD", amount : 4u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "BRS", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "UPF",
        recipe_name : "12xPSL 8xPSM 1xADR=>1xTCU", standard_recipe_name :
        "UPF:1xADR-12xPSL-8xPSM=>1xTCU", duration :
        std::time::Duration::from_millis(138240000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSL", amount : 12u32, }, StaticRecipeMaterial { ticker : "PSM",
        amount : 8u32, }, StaticRecipeMaterial { ticker : "ADR", amount : 1u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "TCU", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "1xBWS 8xPSL 4xPCB 1xBID=>1xFUN", standard_recipe_name :
        "UPF:1xBID-1xBWS-4xPCB-8xPSL=>1xFUN", duration :
        std::time::Duration::from_millis(190080000u64), inputs : & [StaticRecipeMaterial
        { ticker : "BID", amount : 1u32, }, StaticRecipeMaterial { ticker : "PCB", amount
        : 4u32, }, StaticRecipeMaterial { ticker : "PSL", amount : 8u32, },
        StaticRecipeMaterial { ticker : "BWS", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FUN", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "UPF", recipe_name :
        "1xBSU 1000xPFE 1xCPU 1xWOR 1xFUN 1xSU 1xTCU=>1xHAM", standard_recipe_name :
        "UPF:1xBSU-1xCPU-1xFUN-1000xPFE-1xSU-1xTCU-1xWOR=>1xHAM", duration :
        std::time::Duration::from_millis(1296000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "CPU", amount : 1u32, }, StaticRecipeMaterial { ticker : "TCU", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "SU", amount : 1u32, },
        StaticRecipeMaterial { ticker : "BSU", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "FUN", amount : 1u32, }, StaticRecipeMaterial { ticker : "WOR", amount :
        1u32, }, StaticRecipeMaterial { ticker : "PFE", amount : 1000u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HAM", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "UPF", recipe_name :
        "10xPSL 20xPSM 10xFET 1xNV1 5xSAR 1xCOM=>1xBR1", standard_recipe_name :
        "UPF:1xCOM-10xFET-1xNV1-10xPSL-20xPSM-5xSAR=>1xBR1", duration :
        std::time::Duration::from_millis(345600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "SAR", amount : 5u32, }, StaticRecipeMaterial { ticker : "COM", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "PSL", amount : 10u32, },
        StaticRecipeMaterial { ticker : "PSM", amount : 20u32, }, StaticRecipeMaterial {
        ticker : "FET", amount : 10u32, }, StaticRecipeMaterial { ticker : "NV1", amount
        : 1u32, }], outputs : & [StaticRecipeMaterial { ticker : "BR1", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "10xPSL 20xPSM 10xFET 1xNV2 10xSAR 1xCOM=>1xBR2", standard_recipe_name :
        "UPF:1xCOM-10xFET-1xNV2-10xPSL-20xPSM-10xSAR=>1xBR2", duration :
        std::time::Duration::from_millis(432000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "FET", amount : 10u32, }, StaticRecipeMaterial { ticker : "NV2",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "COM", amount : 1u32, },
        StaticRecipeMaterial { ticker : "SAR", amount : 10u32, }, StaticRecipeMaterial {
        ticker : "PSM", amount : 20u32, }, StaticRecipeMaterial { ticker : "PSL", amount
        : 10u32, }], outputs : & [StaticRecipeMaterial { ticker : "BR2", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "10xPSL 4xSAR 1xCOM=>1xDOU", standard_recipe_name :
        "UPF:1xCOM-10xPSL-4xSAR=>1xDOU", duration :
        std::time::Duration::from_millis(155520000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSL", amount : 10u32, }, StaticRecipeMaterial { ticker : "SAR",
        amount : 4u32, }, StaticRecipeMaterial { ticker : "COM", amount : 1u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "DOU", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "100xFLO 1xCRU 1xLIS=>1xCPU", standard_recipe_name :
        "UPF:1xCRU-100xFLO-1xLIS=>1xCPU", duration :
        std::time::Duration::from_millis(518400000u64), inputs : & [StaticRecipeMaterial
        { ticker : "LIS", amount : 1u32, }, StaticRecipeMaterial { ticker : "CRU", amount
        : 1u32, }, StaticRecipeMaterial { ticker : "FLO", amount : 100u32, }], outputs :
        & [StaticRecipeMaterial { ticker : "CPU", amount : 1u32, }], }, StaticRecipeInfo
        { building_ticker : "UPF", recipe_name : "1xWS 1xDA 10xPSL 6xPSM=>1xLU",
        standard_recipe_name : "UPF:1xDA-10xPSL-6xPSM-1xWS=>1xLU", duration :
        std::time::Duration::from_millis(129600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "DA", amount : 1u32, }, StaticRecipeMaterial { ticker : "PSL", amount
        : 10u32, }, StaticRecipeMaterial { ticker : "PSM", amount : 6u32, },
        StaticRecipeMaterial { ticker : "WS", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "LU", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "UPF", recipe_name :
        "20xPSL 30xPSM 20xFET 1xLIS 20xDEC 1xTCU=>1xCQL", standard_recipe_name :
        "UPF:20xDEC-20xFET-1xLIS-20xPSL-30xPSM-1xTCU=>1xCQL", duration :
        std::time::Duration::from_millis(259200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSM", amount : 30u32, }, StaticRecipeMaterial { ticker : "TCU",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "DEC", amount : 20u32, },
        StaticRecipeMaterial { ticker : "LIS", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "FET", amount : 20u32, }, StaticRecipeMaterial { ticker : "PSL", amount
        : 20u32, }], outputs : & [StaticRecipeMaterial { ticker : "CQL", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "20xPSL 6xUTS 12xPSM=>1xWOR", standard_recipe_name :
        "UPF:20xPSL-12xPSM-6xUTS=>1xWOR", duration :
        std::time::Duration::from_millis(155520000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSL", amount : 20u32, }, StaticRecipeMaterial { ticker : "UTS",
        amount : 6u32, }, StaticRecipeMaterial { ticker : "PSM", amount : 12u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "WOR", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "20xPSL 30xPSM 4xSRD=>1xRDS", standard_recipe_name :
        "UPF:20xPSL-30xPSM-4xSRD=>1xRDS", duration :
        std::time::Duration::from_millis(259200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSM", amount : 30u32, }, StaticRecipeMaterial { ticker : "PSL",
        amount : 20u32, }, StaticRecipeMaterial { ticker : "SRD", amount : 4u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "RDS", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "4xSEQ 2xBSC 12xPSL 8xPSM 10xNG=>1xSU", standard_recipe_name :
        "UPF:2xBSC-10xNG-12xPSL-8xPSM-4xSEQ=>1xSU", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "BSC", amount : 2u32, }, StaticRecipeMaterial { ticker : "SEQ", amount
        : 4u32, }, StaticRecipeMaterial { ticker : "NG", amount : 10u32, },
        StaticRecipeMaterial { ticker : "PSM", amount : 8u32, }, StaticRecipeMaterial {
        ticker : "PSL", amount : 12u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "SU", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "UPF",
        recipe_name : "5xPSL 10xPSM 5xFAL 1xLIS 2xDEC 1xTCU=>1xCQS", standard_recipe_name
        : "UPF:2xDEC-5xFAL-1xLIS-5xPSL-10xPSM-1xTCU=>1xCQS", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSM", amount : 10u32, }, StaticRecipeMaterial { ticker : "LIS",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "FAL", amount : 5u32, },
        StaticRecipeMaterial { ticker : "PSL", amount : 5u32, }, StaticRecipeMaterial {
        ticker : "TCU", amount : 1u32, }, StaticRecipeMaterial { ticker : "DEC", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "CQS", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "30xPSL 50xPSM 8xSRD=>1xRDL", standard_recipe_name :
        "UPF:30xPSL-50xPSM-8xSRD=>1xRDL", duration :
        std::time::Duration::from_millis(432000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "PSM", amount : 50u32, }, StaticRecipeMaterial { ticker : "SRD",
        amount : 8u32, }, StaticRecipeMaterial { ticker : "PSL", amount : 30u32, }],
        outputs : & [StaticRecipeMaterial { ticker : "RDL", amount : 1u32, }], },
        StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "2xPSL 5xPSM 5xAL 1xDEC 1xTCU 1xBMF=>1xCQT", standard_recipe_name :
        "UPF:5xAL-1xBMF-1xDEC-2xPSL-5xPSM-1xTCU=>1xCQT", duration :
        std::time::Duration::from_millis(146880000u64), inputs : & [StaticRecipeMaterial
        { ticker : "AL", amount : 5u32, }, StaticRecipeMaterial { ticker : "PSM", amount
        : 5u32, }, StaticRecipeMaterial { ticker : "PSL", amount : 2u32, },
        StaticRecipeMaterial { ticker : "BMF", amount : 1u32, }, StaticRecipeMaterial {
        ticker : "TCU", amount : 1u32, }, StaticRecipeMaterial { ticker : "DEC", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "CQT", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "UPF", recipe_name :
        "1xLIS 1xHAB 60xDDT 200xPFE=>1xBSU", standard_recipe_name :
        "UPF:60xDDT-1xHAB-1xLIS-200xPFE=>1xBSU", duration :
        std::time::Duration::from_millis(432000000u64), inputs : & [StaticRecipeMaterial
        { ticker : "DDT", amount : 60u32, }, StaticRecipeMaterial { ticker : "HAB",
        amount : 1u32, }, StaticRecipeMaterial { ticker : "LIS", amount : 1u32, },
        StaticRecipeMaterial { ticker : "PFE", amount : 200u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "BSU", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "UPF", recipe_name : "6xBBH 10xBDE 10xBSE 50xSOI=>1xHAB",
        standard_recipe_name : "UPF:6xBBH-10xBDE-10xBSE-50xSOI=>1xHAB", duration :
        std::time::Duration::from_millis(172800000u64), inputs : & [StaticRecipeMaterial
        { ticker : "BDE", amount : 10u32, }, StaticRecipeMaterial { ticker : "SOI",
        amount : 50u32, }, StaticRecipeMaterial { ticker : "BBH", amount : 6u32, },
        StaticRecipeMaterial { ticker : "BSE", amount : 10u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "HAB", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "10xBOS 1xGV 1xHE=>1xCHA",
        standard_recipe_name : "WEL:10xBOS-1xGV-1xHE=>1xCHA", duration :
        std::time::Duration::from_millis(38880000u64), inputs : & [StaticRecipeMaterial {
        ticker : "GV", amount : 1u32, }, StaticRecipeMaterial { ticker : "BOS", amount :
        10u32, }, StaticRecipeMaterial { ticker : "HE", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "CHA", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "1xAL 1xBSE 1xHE=>1xFC",
        standard_recipe_name : "WEL:1xAL-1xBSE-1xHE=>1xFC", duration :
        std::time::Duration::from_millis(34560000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HE", amount : 1u32, }, StaticRecipeMaterial { ticker : "BSE", amount :
        1u32, }, StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FC", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "1xFE 1xAL 1xHE=>10xGC",
        standard_recipe_name : "WEL:1xAL-1xFE-1xHE=>10xGC", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HE", amount : 1u32, }, StaticRecipeMaterial { ticker : "FE", amount :
        1u32, }, StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GC", amount : 10u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "1xFE 1xAL 1xHE=>6xFLP",
        standard_recipe_name : "WEL:1xAL-1xFE-1xHE=>6xFLP", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }, StaticRecipeMaterial { ticker : "AL", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "FLP", amount : 6u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "1xAL 1xHE=>1xGV", standard_recipe_name :
        "WEL:1xAL-1xHE=>1xGV", duration : std::time::Duration::from_millis(21600000u64),
        inputs : & [StaticRecipeMaterial { ticker : "AL", amount : 1u32, },
        StaticRecipeMaterial { ticker : "HE", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "GV", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "1xI 1xFE 1xHE=>1xMHL",
        standard_recipe_name : "WEL:1xFE-1xHE-1xI=>1xMHL", duration :
        std::time::Duration::from_millis(60480000u64), inputs : & [StaticRecipeMaterial {
        ticker : "FE", amount : 1u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }, StaticRecipeMaterial { ticker : "I", amount : 1u32, }], outputs : &
        [StaticRecipeMaterial { ticker : "MHL", amount : 1u32, }], }, StaticRecipeInfo {
        building_ticker : "WEL", recipe_name : "200xALR 40xHE=>1xTOR",
        standard_recipe_name : "WEL:200xALR-40xHE=>1xTOR", duration :
        std::time::Duration::from_millis(345600000u64), inputs : & [StaticRecipeMaterial
        { ticker : "HE", amount : 40u32, }, StaticRecipeMaterial { ticker : "ALR", amount
        : 200u32, }], outputs : & [StaticRecipeMaterial { ticker : "TOR", amount : 1u32,
        }], }, StaticRecipeInfo { building_ticker : "WEL", recipe_name :
        "20xNFI 2xAL 1xHE=>6xSSC", standard_recipe_name : "WEL:2xAL-1xHE-20xNFI=>6xSSC",
        duration : std::time::Duration::from_millis(8640000u64), inputs : &
        [StaticRecipeMaterial { ticker : "AL", amount : 2u32, }, StaticRecipeMaterial {
        ticker : "NFI", amount : 20u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "SSC", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "WEL", recipe_name : "2xAL 1xHE=>6xTRU",
        standard_recipe_name : "WEL:2xAL-1xHE=>6xTRU", duration :
        std::time::Duration::from_millis(17280000u64), inputs : & [StaticRecipeMaterial {
        ticker : "HE", amount : 1u32, }, StaticRecipeMaterial { ticker : "AL", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "TRU", amount : 6u32, }],
        }, StaticRecipeInfo { building_ticker : "WEL", recipe_name :
        "2xBE 2xCF 1xHE=>4xTHP", standard_recipe_name : "WEL:2xBE-2xCF-1xHE=>4xTHP",
        duration : std::time::Duration::from_millis(15552000u64), inputs : &
        [StaticRecipeMaterial { ticker : "BE", amount : 2u32, }, StaticRecipeMaterial {
        ticker : "HE", amount : 1u32, }, StaticRecipeMaterial { ticker : "CF", amount :
        2u32, }], outputs : & [StaticRecipeMaterial { ticker : "THP", amount : 4u32, }],
        }, StaticRecipeInfo { building_ticker : "WEL", recipe_name : "6xAL 1xHE=>1xDRF",
        standard_recipe_name : "WEL:6xAL-1xHE=>1xDRF", duration :
        std::time::Duration::from_millis(8640000u64), inputs : & [StaticRecipeMaterial {
        ticker : "AL", amount : 6u32, }, StaticRecipeMaterial { ticker : "HE", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "DRF", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "WPL", recipe_name : "1xKV 1xTC=>1xTK",
        standard_recipe_name : "WPL:1xKV-1xTC=>1xTK", duration :
        std::time::Duration::from_millis(77760000u64), inputs : & [StaticRecipeMaterial {
        ticker : "TC", amount : 1u32, }, StaticRecipeMaterial { ticker : "KV", amount :
        1u32, }], outputs : & [StaticRecipeMaterial { ticker : "TK", amount : 1u32, }],
        }, StaticRecipeInfo { building_ticker : "WPL", recipe_name : "1xRCO=>1xCOT",
        standard_recipe_name : "WPL:1xRCO=>1xCOT", duration :
        std::time::Duration::from_millis(25920000u64), inputs : & [StaticRecipeMaterial {
        ticker : "RCO", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "COT", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "WPL",
        recipe_name : "1xRSI=>1xSIL", standard_recipe_name : "WPL:1xRSI=>1xSIL", duration
        : std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial
        { ticker : "RSI", amount : 1u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "SIL", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "WPL",
        recipe_name : "50xPG=>1xNL", standard_recipe_name : "WPL:50xPG=>1xNL", duration :
        std::time::Duration::from_millis(43200000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 50u32, }], outputs : & [StaticRecipeMaterial { ticker :
        "NL", amount : 1u32, }], }, StaticRecipeInfo { building_ticker : "WPL",
        recipe_name : "75xPG 5xTCL=>1xKV", standard_recipe_name :
        "WPL:75xPG-5xTCL=>1xKV", duration :
        std::time::Duration::from_millis(69120000u64), inputs : & [StaticRecipeMaterial {
        ticker : "PG", amount : 75u32, }, StaticRecipeMaterial { ticker : "TCL", amount :
        5u32, }], outputs : & [StaticRecipeMaterial { ticker : "KV", amount : 1u32, }], }
    ]
}

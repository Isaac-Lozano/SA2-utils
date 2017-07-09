use std::collections::HashMap;

lazy_static! {
    pub static ref OBJ_TABLE: HashMap<(u32, u16), &'static str> = {
        let mut obj_table = HashMap::new();

        // City Escape
        obj_table.insert((13, 0x00), "RING");
        obj_table.insert((13, 0x01), "RING_LINEAR");
        obj_table.insert((13, 0x02), "RING_CIRCLE");
        obj_table.insert((13, 0x03), "SPRA");
        obj_table.insert((13, 0x04), "SPRB");
        obj_table.insert((13, 0x05), "3SPRING");
        obj_table.insert((13, 0x06), "BIGJUMP");
        obj_table.insert((13, 0x07), "KASOKU");
        obj_table.insert((13, 0x08), "SAVEPOINT");
        obj_table.insert((13, 0x09), "SWITCH");
        obj_table.insert((13, 0x0a), "ITEMBOX");
        obj_table.insert((13, 0x0b), "ITEMBOXAIR");
        obj_table.insert((13, 0x0c), "ITEMBOXBALLOON");
        obj_table.insert((13, 0x0d), "LEVUPDAI");
        obj_table.insert((13, 0x0e), "GOALRING");
        obj_table.insert((13, 0x0f), "EMERALD");
        obj_table.insert((13, 0x10), "UDREEL");
        obj_table.insert((13, 0x11), "ORI");
        obj_table.insert((13, 0x12), "DYNAMITE");
        obj_table.insert((13, 0x13), "CONTWOOD");
        obj_table.insert((13, 0x14), "CONTIRON");
        obj_table.insert((13, 0x15), "ROCKET");
        obj_table.insert((13, 0x16), "ROCKETMISSSILE");
        obj_table.insert((13, 0x17), "SCHBOX");
        obj_table.insert((13, 0x18), "HINTBOX");
        obj_table.insert((13, 0x19), "MSGER");
        obj_table.insert((13, 0x1a), "SSS");
        obj_table.insert((13, 0x1b), "SOLIDBOX");
        obj_table.insert((13, 0x1c), "DMYOBJ");
        obj_table.insert((13, 0x1d), "SOAP SW");
        obj_table.insert((13, 0x1e), "SKULL");
        obj_table.insert((13, 0x1f), "PSKULL");
        obj_table.insert((13, 0x20), "CHAOPIPE");
        obj_table.insert((13, 0x21), "MINIMAL");
        obj_table.insert((13, 0x22), "WSMMLS");
        obj_table.insert((13, 0x23), "CONTCHAO");
        obj_table.insert((13, 0x24), "STOPLSD");
        obj_table.insert((13, 0x25), "KNUDAI");
        obj_table.insert((13, 0x26), "KDASIBA");
        obj_table.insert((13, 0x27), "KDWARPHOLE");
        obj_table.insert((13, 0x28), "KDDOOR");
        obj_table.insert((13, 0x29), "KDITEMBOX");
        obj_table.insert((13, 0x2a), "KDDRNGL");
        obj_table.insert((13, 0x2b), "KDDRNGC");
        obj_table.insert((13, 0x2c), "KDSPRING");
        obj_table.insert((13, 0x2d), "KDSPRINGB");
        obj_table.insert((13, 0x2e), "SPHERE");
        obj_table.insert((13, 0x2f), "CCYL");
        obj_table.insert((13, 0x30), "CCUBE");
        obj_table.insert((13, 0x31), "CWALL");
        obj_table.insert((13, 0x32), "CCIRCLE");
        obj_table.insert((13, 0x33), "MODMOD");
        obj_table.insert((13, 0x34), "EFFOBJ0");
        obj_table.insert((13, 0x35), "EFFLENSF");
        obj_table.insert((13, 0x36), "BUNCHIN");
        obj_table.insert((13, 0x37), "IRONBALL2");
        obj_table.insert((13, 0x38), "E KUMI");
        obj_table.insert((13, 0x39), "E AI");
        obj_table.insert((13, 0x3a), "LIGHT SW");
        obj_table.insert((13, 0x3b), "BOARDCOL");
        obj_table.insert((13, 0x3c), "CARMAN");
        obj_table.insert((13, 0x3d), "CARKAZ");
        obj_table.insert((13, 0x3e), "TJUMPDAI");
        obj_table.insert((13, 0x3f), "HAMMER");
        obj_table.insert((13, 0x40), "TRUCK");
        obj_table.insert((13, 0x41), "IRONBAR");
        obj_table.insert((13, 0x42), "TREEST");
        obj_table.insert((13, 0x43), "SWDRNGL");
        obj_table.insert((13, 0x44), "SWDRNGC");
        obj_table.insert((13, 0x45), "TREESHADOWS");
        obj_table.insert((13, 0x46), "LAMP");
        obj_table.insert((13, 0x47), "CARMANC");
        obj_table.insert((13, 0x48), "SIGNS");
        obj_table.insert((13, 0x49), "SIGNS_F");
        obj_table.insert((13, 0x4a), "SBLG");
        obj_table.insert((13, 0x4b), "ROADOBJ");
        obj_table.insert((13, 0x4c), "PALM");
        obj_table.insert((13, 0x4d), "BOARD");
        obj_table.insert((13, 0x4e), "CARKAZ_S");
        obj_table.insert((13, 0x4f), "SLIDER");
        obj_table.insert((13, 0x50), "GREEN_B");
        obj_table.insert((13, 0x51), "ADXCHG");
        obj_table.insert((13, 0x52), "BAR");
        obj_table.insert((13, 0x53), "FENCES");
        obj_table.insert((13, 0x54), "FENCEL");
        obj_table.insert((13, 0x55), "BIG THE CAT");
        obj_table.insert((13, 0x56), "SIGNBOARD");
        obj_table.insert((13, 0x57), "POSTER");
        obj_table.insert((13, 0x58), "TREESTNB");
        obj_table.insert((13, 0x59), "POSTER3");
        obj_table.insert((13, 0x5a), "LINKLINK");
        obj_table.insert((13, 0x5b), "E PATH");
        obj_table.insert((13, 0x5c), "GUIDANCE");
        obj_table.insert((13, 0x5d), "E GOLD");
        obj_table.insert((13, 0x5e), "SARROW");
        obj_table.insert((13, 0x5f), "TRBACK");
        obj_table.insert((13, 0x60), "CARMAN_NEAR");
        obj_table.insert((13, 0x61), "SE_PATCAR");
        obj_table.insert((13, 0x62), "SE_KAZE");
        obj_table.insert((13, 0x63), "POSTERM");
        obj_table.insert((13, 0x64), "NOINPCOL");
        obj_table.insert((13, 0x65), "PIC");

        obj_table
    };
}
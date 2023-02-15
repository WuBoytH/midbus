use {
    std::collections::HashSet,
    once_cell::sync::Lazy,
    walkdir::*
};

mod acmd;
mod agent_init;

pub static MIDBUS_SLOTS: Lazy<HashSet<i32>> = Lazy::new(|| {
    let mut vec = HashSet::new();
    for x in WalkDir::new("mods:/fighter/koopa/model/body").min_depth(1).into_iter().flatten() {
        if x.file_type().is_file() && x.path().ends_with("midbus.marker") {
            let str = x.path().to_str().unwrap().replace("/midbus.marker", "").replace("mods:/fighter/koopa/model/body/c", "");
            let num : i32 = str.parse().unwrap();
            vec.insert(num);
        }
    }
    vec
});

pub fn install() {
    acmd::install();
    agent_init::install();
}

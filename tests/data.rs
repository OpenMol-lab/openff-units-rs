#[path = "../src/data.rs"]
mod data;

#[test]
fn default_registry_contains_core_units() {
    let definitions = data::default_unit_definitions();
    assert!(
        definitions
            .iter()
            .any(|definition| { definition.name == "meter" && definition.matches_name("m") })
    );
    assert!(definitions.iter().any(|definition| {
        definition.name == "kilocalorie_per_mole" && definition.expression == "kilocalorie / mol"
    }));
}

#[test]
fn constants_are_available_to_registry() {
    let definitions = data::constant_definitions();
    assert!(definitions.iter().any(|definition| {
        definition.name == "planck_constant" && definition.matches_name("h")
    }));
}

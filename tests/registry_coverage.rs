use openff_units::{data, unit};

#[test]
fn every_named_definition_is_registered() {
    let registry = unit();
    let mut missing = Vec::new();
    for definition in data::parse_default_definitions()
        .into_iter()
        .chain(data::parse_constant_definitions())
    {
        if matches!(
            definition.kind,
            data::DefinitionKind::Prefix | data::DefinitionKind::Dimension
        ) {
            continue;
        }
        if registry.get(&definition.name).is_err() {
            missing.push(definition.name);
        }
        for alias in definition.aliases {
            if alias.chars().any(char::is_alphabetic) && registry.get(&alias).is_err() {
                missing.push(alias);
            }
        }
    }
    assert!(missing.is_empty(), "missing definitions: {missing:?}");
}

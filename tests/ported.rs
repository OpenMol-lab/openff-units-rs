use openff_units::{Magnitude, Quantity, elements, openmm, unit};

#[test]
fn chemical_aliases_and_nmr_dimensions() {
    let shorthand = Quantity::from_str("2000.0 * kilocalories_per_mole/angstrom**2").unwrap();
    let canonical = Quantity::from_str("2000.0 * kilocalories/mole/angstrom**2").unwrap();
    assert_eq!(shorthand, canonical);

    let one_ohm = Quantity::new(1.0, "ohm").unwrap();
    assert_eq!(one_ohm, Quantity::new(1.0, "henry / second").unwrap());
    assert_eq!(one_ohm, Quantity::new(1.0, "volt / ampere").unwrap());
    assert_eq!(
        Quantity::new(1.0, "watt").unwrap(),
        Quantity::new(1.0, "joule / second").unwrap()
    );
}

#[test]
fn arrays_and_serde_round_trip() {
    let q = Quantity::new(vec![2.0, 3.0], "amu").unwrap();
    assert!(matches!(q.m(), Magnitude::Array(_)));
    let json = serde_json::to_string(&q).unwrap();
    let decoded: Quantity = serde_json::from_str(&json).unwrap();
    assert_eq!(q, decoded);
}

#[test]
fn constants_and_openmm_round_trip() {
    let kb = Quantity::new(1.0, "k_B").unwrap();
    assert!(kb.to("kilogram * meter**2 / second**2 / kelvin").is_ok());
    let kb_openmm = openmm::to_openmm(Some(&kb)).unwrap();
    assert_eq!(kb_openmm.unit().dimension(), kb.u().dimension());
    assert!((kb_openmm.value().as_scalar().unwrap() - 1.380649e-23).abs() < 1e-35);
    let q = Quantity::new(4.0, "nanometer").unwrap();
    let omm = openmm::to_openmm(Some(&q)).unwrap();
    assert_eq!(openmm::from_openmm(Some(&omm)).unwrap(), q);
    assert_eq!(
        openmm::openmm_unit_to_string(Some(&unit().get("kilojoule_per_mole").unwrap())).unwrap(),
        "mole**-1 * kilojoule"
    );
    for expression in [
        "mole**-1 * kilojoule",
        "angstrom**-2 * mole**-1 * kilocalorie",
        "nanometer**-2 * mole**-1 * joule",
        "picosecond**-1",
        "dimensionless",
        "second",
        "angstrom",
    ] {
        let parsed = openmm::string_to_openmm_unit(expression).unwrap();
        assert_eq!(
            openmm::openmm_unit_to_string(Some(&parsed)).unwrap(),
            expression
        );
    }
}

#[test]
fn elements_match_python_tables() {
    assert_eq!(elements::SYMBOLS.get(&1).copied(), Some("H"));
    assert_eq!(elements::NUMBERS.get("Cl").copied(), Some(17));
    assert!((elements::MASSES_F64.get(&6).copied().unwrap() - 12.01078).abs() < 1e-8);
    assert_eq!(elements::MASSES.get(&1).unwrap().u().name(), "dalton");
}

#[test]
fn prefixes_offsets_and_measurements() {
    let q = Quantity::new(1.0, "meter").unwrap();
    assert!((q.to("cm").unwrap().value().unwrap() - 100.0).abs() < 1e-10);
    let mut in_place = Quantity::new(1.0, "meter").unwrap();
    in_place.ito("centimeter").unwrap();
    assert_eq!(in_place.value().unwrap(), 100.0);
    let celsius = Quantity::new(0.0, "degC").unwrap();
    assert!((celsius.to("kelvin").unwrap().value().unwrap() - 273.15).abs() < 1e-12);
    let measurement = Quantity::new(1.0, "kelvin")
        .unwrap()
        .plus_minus(0.05)
        .unwrap();
    assert_eq!(measurement.value().value().unwrap(), 1.0);
    assert_eq!(measurement.error().value().unwrap(), 0.05);
    assert_eq!(
        Quantity::from_str("1/meter").unwrap().u().dimension().0[0],
        -1
    );
    let reciprocal = 1.0 / unit().get("meter").unwrap();
    assert_eq!(reciprocal.unwrap().u().dimension().0[0], -1);
}

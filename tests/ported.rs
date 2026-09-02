use openff_units::{elements, openmm, unit, Magnitude, Quantity};

#[test]
fn chemical_aliases_and_nmr_dimensions() {
    let shorthand = Quantity::from_str("2000.0 * kilocalories_per_mole/angstrom**2").unwrap();
    let canonical = Quantity::from_str("2000.0 * kilocalories/mole/angstrom**2").unwrap();
    assert_eq!(shorthand, canonical);

    let one_ohm = Quantity::new(1.0, "ohm").unwrap();
    assert_eq!(one_ohm, Quantity::new(1.0, "henry / second").unwrap());
    assert_eq!(one_ohm, Quantity::new(1.0, "volt / ampere").unwrap());
    assert_eq!(Quantity::new(1.0, "watt").unwrap(), Quantity::new(1.0, "joule / second").unwrap());
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
    let q = Quantity::new(4.0, "nanometer").unwrap();
    let omm = openmm::to_openmm(Some(&q)).unwrap();
    assert_eq!(openmm::from_openmm(Some(&omm)).unwrap(), q);
    assert_eq!(openmm::openmm_unit_to_string(Some(&unit().get("kilojoule_per_mole").unwrap())).unwrap(), "mole**-1 * kilojoule");
}

#[test]
fn elements_match_python_tables() {
    assert_eq!(elements::SYMBOLS.get(&1).copied(), Some("H"));
    assert_eq!(elements::NUMBERS.get("Cl").copied(), Some(17));
    assert!((elements::MASSES_F64.get(&6).copied().unwrap() - 12.01078).abs() < 1e-8);
}


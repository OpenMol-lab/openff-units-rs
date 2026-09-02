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
        one_ohm,
        (1.0 / Quantity::new(1.0, "siemens").unwrap()).unwrap()
    );
    assert_eq!(
        Quantity::new(1.0, "watt").unwrap(),
        Quantity::new(1.0, "joule / second").unwrap()
    );
    for (name, symbol) in [
        ("tesla", "T"),
        ("ohm", "Ω"),
        ("henry", "H"),
        ("siemens", "S"),
        ("watt", "W"),
        ("weber", "Wb"),
    ] {
        assert_eq!(
            Quantity::new(2.5, name).unwrap(),
            Quantity::new(2.5, symbol).unwrap()
        );
    }
}

#[test]
fn arrays_and_serde_round_trip() {
    let q = Quantity::new(vec![2.0, 3.0], "amu").unwrap();
    assert!(matches!(q.m(), Magnitude::Array(_)));
    let json = serde_json::to_string(&q).unwrap();
    let decoded: Quantity = serde_json::from_str(&json).unwrap();
    assert_eq!(q, decoded);
    let from_text: Quantity = serde_json::from_str("\"1.0 angstrom\"").unwrap();
    assert_eq!(from_text, Quantity::new(1.0, "angstrom").unwrap());
    let from_object: Quantity =
        serde_json::from_str(r#"{"magnitude": 1.0, "units": "angstrom"}"#).unwrap();
    assert_eq!(from_object, Quantity::new(1.0, "angstrom").unwrap());
    let matrix = Quantity::new(ndarray::Array2::eye(3).into_dyn(), "angstrom").unwrap();
    let matrix_json = serde_json::to_string(&matrix).unwrap();
    let matrix_decoded: Quantity = serde_json::from_str(&matrix_json).unwrap();
    assert_eq!(matrix, matrix_decoded);
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
    let dalton_omm = openmm::OpenMMQuantity::new(0.5, "dalton").unwrap();
    assert_eq!(
        openmm::from_openmm(Some(&dalton_omm)).unwrap(),
        Quantity::new(0.5, "gram / mole").unwrap()
    );
    let molar_mass = Quantity::new(0.5, "gram / mole").unwrap();
    assert_eq!(
        openmm::to_openmm(Some(&molar_mass)).unwrap().unit(),
        &unit().get("dalton").unwrap()
    );
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
    let dalton = unit().get("dalton").unwrap();
    assert_eq!(
        openmm::openmm_unit_to_string(Some(&dalton)).unwrap(),
        "g/mol"
    );
    assert_eq!(openmm::string_to_openmm_unit("g/mol").unwrap(), dalton);
    assert!(
        openmm::to_openmm(None)
            .unwrap_err()
            .to_string()
            .contains("OpenFF")
    );
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
    assert_eq!(q.to_base_units().unwrap().u().name(), "meter");
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
    assert!(
        Quantity::new(1.0, "kelvin")
            .unwrap()
            .plus_minus("0.05 meter")
            .is_err()
    );
    assert_eq!(
        Quantity::from_str("1/meter").unwrap().u().dimension().0[0],
        -1
    );
    let reciprocal = 1.0 / unit().get("meter").unwrap();
    assert_eq!(reciprocal.unwrap().u().dimension().0[0], -1);
    assert_eq!(
        Quantity::new(1.0, "kg m / s**2").unwrap(),
        Quantity::new(1.0, "newton").unwrap()
    );
    assert!((unit().get("MPa").unwrap().scale() - 1.0e6).abs() < 1e-6);
    assert!((unit().get("KiB").unwrap().scale() - 8192.0).abs() < 1e-6);
    assert!((unit().get("hectometer").unwrap().scale() - 100.0).abs() < 1e-12);
    assert!((unit().get("decameter").unwrap().scale() - 10.0).abs() < 1e-12);
    assert!(unit().get("yobimeter").is_ok());
    assert!(
        Quantity::new(1.0, "byte")
            .unwrap()
            .is_compatible_with("dimensionless")
    );
}

#[test]
fn ensure_quantity_accepts_scalars_and_arrays() {
    use openff_units::openmm::{EnsuredQuantity, ensure_quantity};

    assert!(matches!(
        ensure_quantity(1, "openff").unwrap(),
        EnsuredQuantity::OpenFF(_)
    ));
    assert!(matches!(
        ensure_quantity(vec![1, 2], "openmm").unwrap(),
        EnsuredQuantity::OpenMM(_)
    ));
    let array = ndarray::Array2::from_elem((2, 2), 3.0).into_dyn();
    assert!(matches!(
        ensure_quantity(array, "openff").unwrap(),
        EnsuredQuantity::OpenFF(_)
    ));
    assert!(ensure_quantity(1.0, "pint").is_err());
}

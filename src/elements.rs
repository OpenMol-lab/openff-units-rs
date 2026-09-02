//! Symbols and standard atomic masses for the chemical elements.
//!
//! The values in this module are the same values used by the Python
//! `openff.units.elements` module.  They were seeded from OpenMM 7.7 and are
//! expressed in unified atomic mass units (daltons).  The maps are indexed by
//! atomic number, which starts at one; no entry for atomic number zero exists.

use std::collections::BTreeMap;
use std::sync::LazyLock;

/// Number of elements represented by the data table.
pub const ELEMENT_COUNT: usize = 116;

/// Largest atomic number represented by the data table.
pub const MAX_ATOMIC_NUMBER: u8 = ELEMENT_COUNT as u8;

/// Element symbols in atomic-number order.
///
/// This array is useful when a compact, allocation-free lookup is preferred.
/// Public map-style access is provided by [`SYMBOLS`].
pub const SYMBOL_DATA: [&str; ELEMENT_COUNT] = [
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk",
    "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Uub",
    "Uut", "Uuq", "Uup", "Uuh",
];

/// Standard atomic masses in atomic-number order, in daltons.
pub const MASS_DATA: [f64; ELEMENT_COUNT] = [
    1.007947,
    4.003,
    6.9412,
    9.0121823,
    10.8117,
    12.01078,
    14.00672,
    15.99943,
    18.99840325,
    20.17976,
    22.989769282,
    24.30506,
    26.98153868,
    28.08553,
    30.9737622,
    32.0655,
    35.4532,
    39.9481,
    39.09831,
    40.0784,
    44.9559126,
    47.8671,
    50.94151,
    51.99616,
    54.9380455,
    55.8452,
    58.9331955,
    58.69342,
    63.5463,
    65.4094,
    69.7231,
    72.641,
    74.921602,
    78.963,
    79.9041,
    83.7982,
    85.46783,
    87.621,
    88.905852,
    91.2242,
    92.906382,
    95.942,
    98.0,
    101.072,
    102.905502,
    106.421,
    107.86822,
    112.4118,
    114.8183,
    118.7107,
    121.7601,
    127.603,
    126.904473,
    131.2936,
    132.90545192,
    137.3277,
    138.905477,
    140.1161,
    140.907652,
    144.2423,
    145.0,
    150.362,
    151.9641,
    157.253,
    158.925352,
    162.5001,
    164.930322,
    167.2593,
    168.934212,
    173.043,
    174.9671,
    178.492,
    180.947882,
    183.841,
    186.2071,
    190.233,
    192.2173,
    195.0849,
    196.9665694,
    200.592,
    204.38332,
    207.21,
    208.980401,
    209.0,
    210.0,
    222.018,
    223.0,
    226.0,
    227.0,
    232.038062,
    231.035882,
    238.028913,
    237.0,
    244.0,
    243.0,
    247.0,
    247.0,
    251.0,
    252.0,
    257.0,
    258.0,
    259.0,
    262.0,
    261.0,
    262.0,
    266.0,
    264.0,
    269.0,
    268.0,
    281.0,
    272.0,
    285.0,
    284.0,
    289.0,
    288.0,
    292.0,
];

/// Mapping from atomic number to element symbol.
pub static SYMBOLS: LazyLock<BTreeMap<u8, &'static str>> = LazyLock::new(|| {
    SYMBOL_DATA
        .iter()
        .enumerate()
        .map(|(index, symbol)| ((index + 1) as u8, *symbol))
        .collect()
});

/// Mapping from atomic number to standard atomic mass in daltons.
///
/// This is the Rust equivalent of Python's ``MASSES`` mapping.  It is named
/// ``MASSES_F64`` to make the unit (dalton) explicit and to leave room for the
/// crate's unit-aware Quantity adapter.
pub static MASSES_F64: LazyLock<BTreeMap<u8, f64>> = LazyLock::new(|| {
    MASS_DATA
        .iter()
        .enumerate()
        .map(|(index, mass)| ((index + 1) as u8, *mass))
        .collect()
});

/// Unit-aware atomic masses, matching Python's ``MASSES`` mapping.
pub static MASSES: LazyLock<BTreeMap<u8, crate::Quantity>> = LazyLock::new(|| {
    MASS_DATA
        .iter()
        .enumerate()
        .map(|(index, mass)| {
            (
                (index + 1) as u8,
                crate::Quantity::new(*mass, "dalton").expect("dalton is registered"),
            )
        })
        .collect()
});

/// Mapping from element symbol to atomic number.
pub static NUMBERS: LazyLock<BTreeMap<&'static str, u8>> = LazyLock::new(|| {
    SYMBOL_DATA
        .iter()
        .enumerate()
        .map(|(index, symbol)| (*symbol, (index + 1) as u8))
        .collect()
});

/// A complete entry in the element table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    pub atomic_number: u8,
    pub symbol: &'static str,
    pub mass: f64,
}

impl Element {
    /// Construct an element from an atomic number.
    pub fn from_atomic_number(atomic_number: u8) -> Option<Self> {
        let index = usize::from(atomic_number).checked_sub(1)?;
        Some(Self {
            atomic_number,
            symbol: *SYMBOL_DATA.get(index)?,
            mass: *MASS_DATA.get(index)?,
        })
    }

    /// Construct an element from its case-sensitive symbol (for example,
    /// ``"Cl"``).
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        Self::from_atomic_number(atomic_number(symbol)?)
    }

    /// Return this element's atomic number.
    pub const fn atomic_number(self) -> u8 {
        self.atomic_number
    }

    /// Return this element's chemical symbol.
    pub const fn symbol(self) -> &'static str {
        self.symbol
    }

    /// Return this element's standard mass in daltons.
    pub const fn mass(self) -> f64 {
        self.mass
    }
}

/// Look up an element symbol by atomic number.
pub fn symbol(atomic_number: u8) -> Option<&'static str> {
    SYMBOLS.get(&atomic_number).copied()
}

/// Look up a standard atomic mass by atomic number, in daltons.
pub fn mass(atomic_number: u8) -> Option<f64> {
    MASSES_F64.get(&atomic_number).copied()
}

/// Look up an atomic number by its case-sensitive element symbol.
pub fn atomic_number(symbol: &str) -> Option<u8> {
    NUMBERS.get(symbol).copied()
}

/// Look up all data for an element by atomic number.
pub fn get(atomic_number: u8) -> Option<Element> {
    Element::from_atomic_number(atomic_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_contain_all_elements() {
        assert_eq!(SYMBOLS.len(), ELEMENT_COUNT);
        assert_eq!(MASSES_F64.len(), ELEMENT_COUNT);
        assert_eq!(NUMBERS.len(), ELEMENT_COUNT);
    }

    #[test]
    fn basic_lookups_match_python_data() {
        assert_eq!(symbol(1), Some("H"));
        assert_eq!(symbol(6), Some("C"));
        assert_eq!(atomic_number("Cl"), Some(17));
        assert_eq!(atomic_number("Cf"), Some(98));
        assert_eq!(mass(1), Some(1.007947));
        assert_eq!(mass(6), Some(12.01078));
    }

    #[test]
    fn symbols_round_trip() {
        for number in 1..=MAX_ATOMIC_NUMBER {
            let symbol = symbol(number).expect("table entry");
            assert_eq!(atomic_number(symbol), Some(number));
        }
    }

    #[test]
    fn element_entries_are_consistent() {
        for atomic_number in 1..=MAX_ATOMIC_NUMBER {
            let element = get(atomic_number).expect("table entry");
            assert_eq!(element.atomic_number(), atomic_number);
            assert_eq!(element.symbol(), symbol(atomic_number).unwrap());
            assert_eq!(element.mass(), mass(atomic_number).unwrap());
            assert_eq!(Element::from_symbol(element.symbol()), Some(element));
        }
    }

    #[test]
    fn invalid_lookups_return_none() {
        assert_eq!(symbol(0), None);
        assert_eq!(symbol(MAX_ATOMIC_NUMBER + 1), None);
        assert_eq!(mass(0), None);
        assert_eq!(atomic_number("cl"), None);
        assert_eq!(Element::from_symbol("not-an-element"), None);
    }
}

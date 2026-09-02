use crate::{Dimension, Result, Unit, UnitError};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Registry containing the OpenFF unit definitions.
#[derive(Clone, Debug)]
pub struct UnitRegistry {
    units: HashMap<String, Unit>,
}

pub static DEFAULT_UNIT_REGISTRY: Lazy<UnitRegistry> = Lazy::new(UnitRegistry::new);

/// Access the process-wide default registry.
pub fn unit() -> &'static UnitRegistry {
    &DEFAULT_UNIT_REGISTRY
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            units: HashMap::new(),
        };
        registry.define_base("meter", "m", 1.0, Dimension::with(0), &["metre"]);
        registry.define_base("second", "s", 1.0, Dimension::with(1), &["sec"]);
        registry.define_base("ampere", "A", 1.0, Dimension::with(3), &["amp"]);
        registry.define_base("gram", "g", 1e-3, Dimension::with(2), &[]);
        registry.define_base("mole", "mol", 1.0, Dimension::with(5), &[]);
        registry.define_base(
            "kelvin",
            "K",
            1.0,
            Dimension::with(4),
            &["degK", "degree_Kelvin", "degreeK", "Kelvin"],
        );
        registry.define_base("radian", "rad", 1.0, Dimension::NONE, &["rad"]);
        registry.define_base("bit", "bit", 1.0, Dimension::with(8), &[]);
        registry.define_base("count", "count", 1.0, Dimension::NONE, &[]);
        registry.define(
            "dimensionless",
            "dimensionless",
            1.0,
            Dimension::NONE,
            &[""],
        );

        // CODATA 2018 constants. Constants are represented as units with their
        // physical dimensions, which allows them to participate in expressions
        // such as `hartree = 2 * rydberg`.
        registry.define_expr("pi", "π", "3.141592653589793", &[]);
        registry.define_expr(
            "speed_of_light",
            "c",
            "299792458 * meter / second",
            &["c_0"],
        );
        registry.define_expr(
            "planck_constant",
            "h",
            "6.62607015e-34 * kilogram * meter ** 2 / second",
            &[],
        );
        registry.define_expr(
            "elementary_charge",
            "e",
            "1.602176634e-19 * ampere * second",
            &[],
        );
        registry.define_expr("avogadro_number", "N_A0", "6.02214076e23", &[]);
        registry.define_expr(
            "boltzmann_constant",
            "k_B",
            "1.380649e-23 * kilogram * meter ** 2 / second ** 2 / kelvin",
            &["k"],
        );
        registry.define_expr(
            "standard_atmosphere",
            "atm",
            "1.01325e5 * kilogram / meter / second ** 2",
            &["atmosphere"],
        );
        registry.define_expr("zeta", "ζ", "speed_of_light / (centimeter / second)", &[]);
        registry.define_expr(
            "dirac_constant",
            "hbar",
            "planck_constant / (2 * pi)",
            &["ħ", "atomic_unit_of_action", "a_u_action"],
        );
        registry.define_expr("avogadro_constant", "N_A", "avogadro_number / mole", &[]);
        registry.define_expr(
            "molar_gas_constant",
            "R",
            "boltzmann_constant * avogadro_constant",
            &[],
        );
        registry.define_expr(
            "rydberg_constant",
            "R_inf",
            "1.0973731568160e7 / meter",
            &["R_∞"],
        );
        registry.define_expr(
            "atomic_mass_constant",
            "m_u",
            "1.66053906660e-27 * kilogram",
            &[],
        );
        registry.define_expr(
            "electron_mass",
            "m_e",
            "9.1093837015e-31 * kilogram",
            &["atomic_unit_of_mass", "a_u_mass"],
        );
        registry.define(
            "fine_structure_constant",
            "alpha",
            0.0072973525693,
            Dimension::NONE,
            &["α"],
        );
        registry.define_expr("vacuum_permeability", "mu_0", "2 * fine_structure_constant * planck_constant / (elementary_charge ** 2 * speed_of_light)", &["µ_0", "mu0", "magnetic_constant"]);
        registry.define_expr("vacuum_permittivity", "epsilon_0", "elementary_charge ** 2 / (2 * fine_structure_constant * planck_constant * speed_of_light)", &["ε_0", "eps_0", "eps0", "electric_constant"]);

        registry.define(
            "degree",
            "deg",
            std::f64::consts::PI / 180.0,
            Dimension::NONE,
            &["arcdeg", "arcdegree", "angular_degree"],
        );
        registry.define("byte", "B", 8.0, Dimension::with(8), &["octet"]);
        registry.define_expr("angstrom", "Å", "1e-10 * meter", &["ångström", "Å"]);
        registry.define_expr("micron", "µ", "micrometer", &[]);
        registry.define_expr("micrometer", "µm", "1e-6 * meter", &["um"]);
        registry.define_expr("fermi", "fm", "1e-15 * meter", &[]);
        registry.define_expr("femtometer", "fm", "fermi", &[]);
        registry.define_expr(
            "bohr",
            "a0",
            "5.29177210903e-11 * meter",
            &["a_0", "bohr_radius", "atomic_unit_of_length", "a_u_length"],
        );

        registry.define_expr(
            "unified_atomic_mass_unit",
            "u",
            "1.66053906660e-27 * kilogram",
            &["atomic_mass_constant", "amu"],
        );
        registry.define_expr("dalton", "Da", "unified_atomic_mass_unit", &[]);

        registry.define_expr("minute", "min", "60 * second", &[]);
        registry.define_expr("hour", "hr", "60 * minute", &[]);
        registry.define_expr("day", "d", "24 * hour", &[]);
        registry.define_expr("week", "week", "7 * day", &[]);
        registry.define_expr("year", "yr", "365.25 * day", &["a", "julian_year"]);
        registry.define_expr("month", "month", "year / 12", &[]);
        registry.define(
            "timestep",
            "timestep",
            1.0,
            Dimension::with(7),
            &["_", "timesteps"],
        );
        registry.define(
            "degree_Celsius",
            "°C",
            1.0,
            Dimension::with(4),
            &["celsius", "degC", "degreeC"],
        );
        for celsius in registry
            .units
            .values_mut()
            .filter(|value| value.name == "degree_Celsius")
        {
            celsius.offset = 273.15;
        }

        registry.define_expr("liter", "L", "decimeter ** 3", &["l", "litre"]);
        registry.define_expr("hertz", "Hz", "1 / second", &["Hz"]);
        registry.define_expr(
            "reciprocal_centimeter",
            "cm^-1",
            "1 / centimeter",
            &["cm_1", "kayser"],
        );
        registry.define_expr("newton", "N", "kilogram * meter / second ** 2", &[]);
        registry.define_expr("dyne", "dyn", "gram * centimeter / second ** 2", &[]);
        registry.define_expr("joule", "J", "newton * meter", &[]);
        registry.define_expr("erg", "erg", "dyne * centimeter", &[]);
        registry.define_expr("rydberg", "Ry", "rydberg_constant * h * c", &[]);
        registry.define_expr(
            "hartree",
            "Eh",
            "2 * rydberg",
            &[
                "E_h",
                "hartree_energy",
                "atomic_unit_of_energy",
                "a_u_energy",
            ],
        );
        registry.define_expr(
            "calorie",
            "cal",
            "4.184 * joule",
            &["thermochemical_calorie", "cal_th"],
        );
        registry.define_expr(
            "calorie_per_mole",
            "cal/mol",
            "calorie / mole",
            &["calories_per_mole"],
        );
        registry.define_expr(
            "joule_per_mole",
            "J/mol",
            "joule / mole",
            &["joules_per_mole"],
        );
        registry.define_expr("kilocalorie", "kcal", "1000 * calorie", &["kilocalories"]);
        registry.define_expr("kilojoule", "kJ", "1000 * joule", &["kilojoules"]);
        registry.define_expr(
            "kilocalorie_per_mole",
            "kcal/mol",
            "kilocalorie / mole",
            &["kilocalories_per_mole"],
        );
        registry.define_expr(
            "kilojoule_per_mole",
            "kJ/mol",
            "kilojoule / mole",
            &["kilojoules_per_mole"],
        );
        registry.define_expr("watt", "W", "kilogram * meter ** 2 / second ** 3", &[]);
        registry.define_expr(
            "ohm",
            "Ω",
            "kilogram * meter ** 2 / second ** 3 / ampere ** 2",
            &["ohm"],
        );
        registry.define_expr("siemens", "S", "1 / ohm", &[]);
        registry.define_expr(
            "henry",
            "H",
            "kilogram * meter ** 2 / second ** 2 / ampere ** 2",
            &[],
        );
        registry.define_expr(
            "weber",
            "Wb",
            "kilogram * meter ** 2 / second ** 2 / ampere",
            &[],
        );
        registry.define_expr("tesla", "T", "kilogram / second ** 2 / ampere", &[]);
        registry.define_expr("pascal", "Pa", "newton / meter ** 2", &[]);
        registry.define_expr("bar", "bar", "1e5 * pascal", &[]);
        registry.define_expr("molar", "M", "mole / liter", &[]);
        registry.define_expr("coulomb", "C", "ampere * second", &[]);
        registry.define_expr(
            "faraday",
            "Fr",
            "elementary_charge * avogadro_constant * mole",
            &[],
        );
        registry.define_expr("volt", "V", "joule / coulomb", &[]);
        registry.define_expr("farad", "F", "coulomb / volt", &[]);
        registry.define_expr("debye", "D", "1e-9 / zeta * coulomb * angstrom", &[]);

        registry
    }

    fn define_base(
        &mut self,
        name: &str,
        symbol: &str,
        scale: f64,
        dimension: Dimension,
        aliases: &[&str],
    ) {
        self.define(name, symbol, scale, dimension, aliases);
    }

    fn define(
        &mut self,
        name: &str,
        symbol: &str,
        scale: f64,
        dimension: Dimension,
        aliases: &[&str],
    ) {
        let unit = Unit::with_symbol(name, symbol, scale, 0.0, dimension);
        self.units.insert(normalize(name), unit.clone());
        // Preserve case-sensitive SI symbols (for example `M` for molar and
        // `m` for metre); lower-case normalization must not overwrite an
        // earlier canonical spelling.
        self.units
            .entry(normalize(symbol))
            .or_insert_with(|| unit.clone());
        self.units.insert(symbol.to_owned(), unit.clone());
        for alias in aliases {
            self.units
                .entry(normalize(alias))
                .or_insert_with(|| unit.clone());
            self.units.insert((*alias).to_owned(), unit.clone());
        }
    }

    fn define_expr(&mut self, name: &str, symbol: &str, expression: &str, aliases: &[&str]) {
        if let Ok(mut value) = self.parse(expression) {
            value.name = name.to_owned();
            value.symbol = symbol.to_owned();
            self.units.insert(normalize(name), value.clone());
            self.units
                .entry(normalize(symbol))
                .or_insert_with(|| value.clone());
            self.units.insert(symbol.to_owned(), value.clone());
            for alias in aliases {
                self.units
                    .entry(normalize(alias))
                    .or_insert_with(|| value.clone());
                self.units.insert((*alias).to_owned(), value.clone());
            }
        }
    }

    pub fn get(&self, name: &str) -> Result<Unit> {
        if let Some(value) = self.units.get(name.trim()) {
            return Ok(value.clone());
        }
        let key = normalize(name);
        if let Some(value) = self.units.get(&key) {
            return Ok(value.clone());
        }
        if let Some(value) = self.prefixed(&key) {
            return Ok(value);
        }
        Err(UnitError::UnknownUnit(name.to_owned()))
    }

    /// Construct a quantity using this registry.
    pub fn quantity<M, U>(&self, magnitude: M, unit: U) -> Result<crate::Quantity>
    where
        M: Into<crate::Magnitude>,
        U: Into<crate::UnitInput>,
    {
        crate::Quantity::new(magnitude, unit)
    }

    fn prefixed(&self, key: &str) -> Option<Unit> {
        const PREFIXES: &[(&str, f64)] = &[
            ("deka", 1e1),
            ("deci", 1e-1),
            ("centi", 1e-2),
            ("milli", 1e-3),
            ("micro", 1e-6),
            ("nano", 1e-9),
            ("pico", 1e-12),
            ("femto", 1e-15),
            ("atto", 1e-18),
            ("zepto", 1e-21),
            ("yocto", 1e-24),
            ("kilo", 1e3),
            ("mega", 1e6),
            ("giga", 1e9),
            ("tera", 1e12),
            ("peta", 1e15),
            ("exa", 1e18),
            ("zetta", 1e21),
            ("yotta", 1e24),
            // Common SI symbols are accepted in addition to long prefixes.
            ("da", 1e1),
            ("d", 1e-1),
            ("c", 1e-2),
            ("m", 1e-3),
            ("u", 1e-6),
            ("n", 1e-9),
            ("p", 1e-12),
            ("f", 1e-15),
            ("a", 1e-18),
            ("z", 1e-21),
            ("y", 1e-24),
            ("k", 1e3),
            ("M", 1e6),
            ("G", 1e9),
            ("T", 1e12),
            ("P", 1e15),
            ("E", 1e18),
            ("Z", 1e21),
            ("Y", 1e24),
        ];
        for (prefix, factor) in PREFIXES {
            if let Some(base_name) = key.strip_prefix(prefix)
                && let Some(base) = self.units.get(base_name)
                && base.offset == 0.0
            {
                return Some(Unit::with_symbol(
                    key,
                    key,
                    base.scale * factor,
                    0.0,
                    base.dimension,
                ));
            }
        }
        None
    }

    pub fn parse(&self, expression: &str) -> Result<Unit> {
        let tokens = tokenize(expression)?;
        if tokens.is_empty() {
            return Err(UnitError::Parse("empty expression".to_owned()));
        }
        let mut parser = Parser {
            registry: self,
            tokens,
            position: 0,
        };
        let value = parser.parse_product()?;
        if parser.position != parser.tokens.len() {
            return Err(UnitError::Parse(format!(
                "unexpected token near position {}",
                parser.position
            )));
        }
        Ok(value)
    }

    pub(crate) fn base_unit(&self, dimension: Dimension) -> Result<Unit> {
        if dimension == Dimension::NONE {
            return self.get("dimensionless");
        }
        let names = [
            "meter", "second", "kilogram", "ampere", "kelvin", "mole", "count", "timestep", "bit",
        ];
        let mut result = self.get("dimensionless")?;
        for (index, exponent) in dimension.0.iter().enumerate() {
            if *exponent == 0 {
                continue;
            }
            let base = self.get(names[index])?.powi(*exponent)?;
            result = result.mul(&base)?;
        }
        Ok(result)
    }
}

fn normalize(value: &str) -> String {
    value
        .trim()
        .replace(['µ', 'μ'], "micro")
        .replace(['Å', 'Å'], "angstrom")
        .replace('Ω', "ohm")
        .replace('π', "pi")
        .replace('ζ', "zeta")
        .replace('∞', "inf")
        .replace('−', "-")
        .to_ascii_lowercase()
}

#[derive(Clone, Debug)]
enum Token {
    Name(String),
    Number(f64),
    Mul,
    Div,
    Pow,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.replace("**", "^").chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        match c {
            '*' => tokens.push(Token::Mul),
            '/' => tokens.push(Token::Div),
            '^' => tokens.push(Token::Pow),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '+' | '-' | '0'..='9' | '.' => {
                let start = i;
                i += 1;
                while i < chars.len()
                    && (chars[i].is_ascii_digit()
                        || matches!(chars[i], '.' | 'e' | 'E' | '+' | '-'))
                {
                    if (chars[i] == '+' || chars[i] == '-')
                        && chars[i - 1] != 'e'
                        && chars[i - 1] != 'E'
                    {
                        break;
                    }
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                let number = text
                    .parse::<f64>()
                    .map_err(|_| UnitError::Parse(format!("invalid number `{text}`")))?;
                tokens.push(Token::Number(number));
                continue;
            }
            _ => {
                let start = i;
                i += 1;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || !chars[i].is_ascii())
                {
                    i += 1;
                }
                tokens.push(Token::Name(chars[start..i].iter().collect()));
                continue;
            }
        }
        i += 1;
    }
    Ok(tokens)
}

struct Parser<'a> {
    registry: &'a UnitRegistry,
    tokens: Vec<Token>,
    position: usize,
}

impl Parser<'_> {
    fn parse_product(&mut self) -> Result<Unit> {
        let mut value = self.parse_power()?;
        loop {
            if self.position >= self.tokens.len() {
                break;
            }
            let divide = match self.tokens[self.position] {
                Token::Mul => false,
                Token::Div => true,
                _ => break,
            };
            self.position += 1;
            let rhs = self.parse_power()?;
            value = if divide {
                value.div(&rhs)?
            } else {
                value.mul(&rhs)?
            };
        }
        Ok(value)
    }

    fn parse_power(&mut self) -> Result<Unit> {
        let mut value = self.parse_atom()?;
        if self.position < self.tokens.len() && matches!(self.tokens[self.position], Token::Pow) {
            self.position += 1;
            let exponent = match self.tokens.get(self.position) {
                Some(Token::Number(number)) if number.fract() == 0.0 => *number as i32,
                _ => {
                    return Err(UnitError::Parse(
                        "unit exponents must be integers".to_owned(),
                    ));
                }
            };
            self.position += 1;
            value = value.powi(exponent)?;
        }
        Ok(value)
    }

    fn parse_atom(&mut self) -> Result<Unit> {
        match self.tokens.get(self.position).cloned() {
            Some(Token::Name(name)) => {
                self.position += 1;
                self.registry.get(&name)
            }
            Some(Token::Number(number)) => {
                self.position += 1;
                Ok(Unit::new(number.to_string(), number, Dimension::NONE))
            }
            Some(Token::LParen) => {
                self.position += 1;
                let value = self.parse_product()?;
                if !matches!(self.tokens.get(self.position), Some(Token::RParen)) {
                    return Err(UnitError::Parse("missing closing parenthesis".to_owned()));
                }
                self.position += 1;
                Ok(value)
            }
            _ => Err(UnitError::Parse(format!(
                "expected unit at position {}",
                self.position
            ))),
        }
    }
}

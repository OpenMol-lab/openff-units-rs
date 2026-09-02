//! Built-in unit and physical-constant definitions.
//!
//! The definitions are kept in the same Pint-compatible text format used by
//! the Python implementation.  [`parse_default_definitions`] and
//! [`parse_constant_definitions`] turn those files into small, owned records
//! that a unit registry can consume without depending on a parser crate.

/// The kind of a definition in one of the built-in data files.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionKind {
    /// A decimal or binary prefix such as `kilo- = 1e3 = k-`.
    Prefix,
    /// An SI base unit such as `meter = [length] = m`.
    BaseUnit,
    /// A named dimension declaration such as `[area] = [length] ** 2`.
    Dimension,
    /// A derived or convenience unit.
    Unit,
    /// A mathematical or physical constant.
    Constant,
}

/// One parsed entry from `defaults.txt` or `constants.txt`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Definition {
    /// Canonical name used on the left-hand side of the definition.
    pub name: String,
    /// Right-hand-side expression, before aliases and an optional offset.
    pub expression: String,
    /// Alternate spellings listed after the expression.
    pub aliases: Vec<String>,
    /// Additive offset, used by temperature units such as Celsius.
    pub offset: Option<String>,
    /// Category of this entry in the source file.
    pub kind: DefinitionKind,
}

impl Definition {
    /// Return whether `name` is the canonical spelling or one of its aliases.
    pub fn matches_name(&self, name: &str) -> bool {
        self.name == name || self.aliases.iter().any(|alias| alias == name)
    }
}

/// The unmodified Pint-compatible defaults file bundled with this crate.
pub const DEFAULTS_TEXT: &str = include_str!("../data/defaults.txt");

/// The unmodified Pint-compatible constants file bundled with this crate.
pub const CONSTANTS_TEXT: &str = include_str!("../data/constants.txt");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DefaultsSection {
    None,
    Prefix,
    BaseUnit,
    Unit,
}

/// Parse the built-in defaults file into prefixes, base units, dimensions, and
/// derived units.
pub fn parse_default_definitions() -> Vec<Definition> {
    let mut section = DefaultsSection::None;
    let mut in_system = false;
    let mut definitions = Vec::new();

    for line in DEFAULTS_TEXT.lines() {
        let line = line.trim();
        if line.starts_with("#### PREFIXES") {
            section = DefaultsSection::Prefix;
            continue;
        }
        if line.starts_with("#### BASE UNITS") {
            section = DefaultsSection::BaseUnit;
            continue;
        }
        if line.starts_with("#### UNITS") {
            section = DefaultsSection::Unit;
            continue;
        }
        if line.starts_with("#### ") {
            // CONSTANTS and SYSTEMS OF UNITS are metadata/import sections in
            // defaults.txt; their entries are not unit definitions here.
            section = DefaultsSection::None;
            continue;
        }
        if line.starts_with("@system ") {
            in_system = true;
            continue;
        }
        if line == "@end" {
            in_system = false;
            continue;
        }
        if in_system || line.is_empty() || line.starts_with('#') || line.starts_with("@") {
            continue;
        }

        let kind = match section {
            DefaultsSection::Prefix => DefinitionKind::Prefix,
            DefaultsSection::BaseUnit => DefinitionKind::BaseUnit,
            DefaultsSection::Unit if line.starts_with('[') => DefinitionKind::Dimension,
            DefaultsSection::Unit => DefinitionKind::Unit,
            DefaultsSection::None => continue,
        };
        if let Some(definition) = parse_definition_line(line, kind) {
            definitions.push(definition);
        }
    }
    definitions
}

/// Parse the built-in constants file into named constant definitions.
pub fn parse_constant_definitions() -> Vec<Definition> {
    CONSTANTS_TEXT
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('@') {
                None
            } else {
                parse_definition_line(line, DefinitionKind::Constant)
            }
        })
        .collect()
}

/// Alias for [`parse_default_definitions`], useful to callers that think in
/// terms of a registry's default unit set.
pub fn default_unit_definitions() -> Vec<Definition> {
    parse_default_definitions()
}

/// Alias for [`parse_constant_definitions`].
pub fn constant_definitions() -> Vec<Definition> {
    parse_constant_definitions()
}

/// Parse a single Pint definition line.
///
/// A line consists of `name = expression = alias = ...`; temperature entries
/// may additionally contain `; offset: value` between the expression and
/// aliases.  Comments are removed before parsing.
fn parse_definition_line(line: &str, kind: DefinitionKind) -> Option<Definition> {
    let line = line.split_once('#').map_or(line, |(body, _)| body).trim();
    if line.is_empty() {
        return None;
    }

    let (main, offset_and_aliases) = match line.split_once("; offset:") {
        Some((main, suffix)) => (main.trim(), Some(suffix.trim())),
        None => (line, None),
    };
    let mut fields = main
        .split('=')
        .map(str::trim)
        .filter(|field| !field.is_empty());
    let name = fields.next()?.to_owned();
    let expression = fields.next()?.to_owned();
    let mut aliases: Vec<String> = fields.map(str::to_owned).collect();

    let offset = offset_and_aliases.map(|suffix| {
        let mut suffix_fields = suffix
            .split('=')
            .map(str::trim)
            .filter(|field| !field.is_empty());
        let value = suffix_fields.next().unwrap_or_default().to_owned();
        aliases.extend(suffix_fields.map(str::to_owned));
        value
    });

    Some(Definition {
        name,
        expression,
        aliases,
        offset,
        kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_sources_are_present() {
        assert!(DEFAULTS_TEXT.contains("meter = [length]"));
        assert!(CONSTANTS_TEXT.contains("speed_of_light ="));
    }

    #[test]
    fn parses_prefixes_and_aliases() {
        let definitions = parse_default_definitions();
        let kilo = definitions
            .iter()
            .find(|definition| definition.name == "kilo-")
            .expect("kilo prefix");
        assert_eq!(kilo.kind, DefinitionKind::Prefix);
        assert_eq!(kilo.expression, "1e3");
        assert!(kilo.matches_name("k-"));
        assert_eq!(
            definitions
                .iter()
                .filter(|definition| definition.kind == DefinitionKind::Prefix)
                .count(),
            28
        );
    }

    #[test]
    fn parses_base_dimension_and_offset_entries() {
        let definitions = parse_default_definitions();
        let meter = definitions
            .iter()
            .find(|definition| definition.name == "meter")
            .expect("meter");
        assert_eq!(meter.kind, DefinitionKind::BaseUnit);
        assert_eq!(meter.expression, "[length]");
        assert!(meter.matches_name("metre"));

        let volume = definitions
            .iter()
            .find(|definition| definition.name == "[volume]")
            .expect("volume dimension");
        assert_eq!(volume.kind, DefinitionKind::Dimension);
        assert_eq!(volume.expression, "[length] ** 3");

        let celsius = definitions
            .iter()
            .find(|definition| definition.name == "degree_Celsius")
            .expect("Celsius");
        assert_eq!(celsius.offset.as_deref(), Some("273.15"));
        assert!(celsius.matches_name("degC"));
    }

    #[test]
    fn parses_constants_and_unicode_aliases() {
        let definitions = parse_constant_definitions();
        let speed = definitions
            .iter()
            .find(|definition| definition.name == "speed_of_light")
            .expect("speed of light");
        assert_eq!(speed.expression, "299792458 m/s");
        assert!(speed.matches_name("c"));

        let alpha = definitions
            .iter()
            .find(|definition| definition.name == "fine_structure_constant")
            .expect("fine-structure constant");
        assert!(alpha.matches_name("α"));
        assert!(alpha.matches_name("alpha"));
    }

    #[test]
    fn malformed_or_comment_lines_are_ignored() {
        assert!(parse_definition_line("# comment", DefinitionKind::Unit).is_none());
        assert!(parse_definition_line("without an equals sign", DefinitionKind::Unit).is_none());
    }
}

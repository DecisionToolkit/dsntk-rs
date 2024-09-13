//! # Input files for testing validation against XML Schema v1.3
//!
//! For each input file, there are validation errors provided,
//! these errors are reported by `xmllint` tool.

/// ```text
/// v13/_0001.dmn:2: element definitionen: Schemas validity error : Element 'definitionen': No matching global declaration available for the validation root.
/// ```
pub const _0001: &str = include_str!("_0001.dmn");

/// ```text
/// v13/_0002.dmn:2: element definitions: Schemas validity error : Element 'definitions': No matching global declaration available for the validation root.
/// ```
pub const _0002: &str = include_str!("_0002.dmn");

/// ```text
/// v13/_0003.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/invalid}definitions': No matching global declaration available for the validation root.
/// ```
pub const _0003: &str = include_str!("_0003.dmn");

/// ```text
/// v13/_0004.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'namespace' is required but missing.
/// v13/_0004.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'name' is required but missing.
/// ```
pub const _0004: &str = include_str!("_0004.dmn");

/// ```text
/// v13/_0005.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'namespace' is required but missing.
/// v13/_0005.dmn:2: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions': The attribute 'name' is required but missing.
/// ```
pub const _0005: &str = include_str!("_0005.dmn");

/// ```text
/// v13/_0006.dmn:5: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions', attribute 'revision': The attribute 'revision' is not allowed.
/// ```
pub const _0006: &str = include_str!("_0006.dmn");

/// ```text
/// v13/_0007.dmn:5: element definitions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}definitions', attribute 'revision': The attribute 'revision' is not allowed.
/// ```
pub const _0007: &str = include_str!("_0007.dmn");

/// ```text
/// v13/_0008.dmn:5: element revisions: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}revisions': This element is not expected.
/// Expected is one of (
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}description,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}extensionElements,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}import,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}itemDefinition,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}drgElement,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}decision,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}businessKnowledgeModel,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}inputData,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}knowledgeSource,
///  {https://www.omg.org/spec/DMN/20191111/MODEL/}decisionService
/// )
/// ```
pub const _0008: &str = include_str!("_0008.dmn");

pub const _0009: &str = include_str!("_0009.dmn");

/// ```text
/// v13/_0030.dmn:5: element inputData: Schemas validity error : Element '{https://www.omg.org/spec/DMN/20191111/MODEL/}inputData': The attribute 'name' is required but missing.
/// ```
pub const _0030: &str = include_str!("_0030.dmn");

pub const _0031: &str = include_str!("_0031.dmn");

pub const _0032: &str = include_str!("_0032.dmn");

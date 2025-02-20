use crate::impl_features::{get_impl_features_for_container, Feature};
use crate::ir_printer::{IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::if_statement::{Conditional, Equation, IfStatement};
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestUpdateMaskValue, TestValue};
use crate::parser::types::ty::Type;
use crate::parser::types::ContainerValue;
use crate::rust_printer::UpdateMaskType;
use crate::Objects;
use serde::Serialize;

pub(crate) fn containers_to_ir<'a>(
    containers: impl Iterator<Item = &'a Container>,
    o: &'a Objects,
) -> Vec<IrContainer> {
    containers.map(|a| container_to_ir(a, o)).collect()
}

fn container_to_ir(e: &Container, o: &Objects) -> IrContainer {
    let members = e.members().iter().map(|a| a.into()).collect();

    let tests = e.tests(o).iter().map(|a| a.into()).collect();

    let has_manual_size_field = e
        .all_definitions()
        .iter()
        .any(|a| a.size_of_fields_before_size().is_some());

    IrContainer {
        name: e.name().to_string(),
        object_type: e.container_type().into(),
        sizes: IrSizes::from_sizes(e.sizes()),
        members,
        tags: IrTags::from_tags(e.tags()),
        tests,
        file_info: IrFileInfo {
            file_name: e.file_info().name().to_string(),
            start_position: e.file_info().start_line() as u32,
            end_position: e.file_info().end_line() as u32,
        },
        only_has_io_error: e.only_has_io_errors(),
        has_manual_size_field,
        manual_size_subtraction: e.manual_size_field_subtraction(),
        features: get_impl_features_for_container(e).to_array(),
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "container_type_tag", content = "opcode")]
pub(crate) enum IrContainerType {
    Struct,
    CLogin(u16),
    SLogin(u16),
    Msg(u16),
    CMsg(u16),
    SMsg(u16),
}

impl From<ContainerType> for IrContainerType {
    fn from(v: ContainerType) -> Self {
        match v {
            ContainerType::Struct => Self::Struct,
            ContainerType::CLogin(o) => Self::CLogin(o),
            ContainerType::SLogin(o) => Self::SLogin(o),
            ContainerType::Msg(o) => Self::Msg(o),
            ContainerType::CMsg(o) => Self::CMsg(o),
            ContainerType::SMsg(o) => Self::SMsg(o),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrSizes {
    constant_sized: bool,
    minimum_size: u32,
    maximum_size: u32,
}

impl IrSizes {
    fn from_sizes(v: Sizes) -> Self {
        Self {
            constant_sized: v.is_constant().is_some(),
            minimum_size: v.minimum() as u32,
            maximum_size: v.maximum() as u32,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrContainer {
    name: String,
    object_type: IrContainerType,
    sizes: IrSizes,
    members: Vec<IrStructMember>,
    tags: IrTags,
    tests: Vec<IrTestCase>,
    file_info: IrFileInfo,
    only_has_io_error: bool,
    has_manual_size_field: bool,
    manual_size_subtraction: Option<u16>,
    features: Vec<Feature>,
}

impl IrContainer {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrOptionalStatement {
    name: String,
    members: Vec<IrStructMember>,
}

impl From<&OptionalStatement> for IrOptionalStatement {
    fn from(v: &OptionalStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            name: v.name().to_string(),
            members,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "struct_member_tag", content = "struct_member_content")]
pub(crate) enum IrStructMember {
    Definition(Box<IrStructMemberDefinition>),
    IfStatement(IrIfStatement),
    Optional(IrOptionalStatement),
}

impl From<&StructMember> for IrStructMember {
    fn from(v: &StructMember) -> Self {
        match v {
            StructMember::Definition(d) => Self::Definition(Box::new(d.into())),
            StructMember::IfStatement(statement) => Self::IfStatement(statement.into()),
            StructMember::OptionalStatement(optional) => Self::Optional(optional.into()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "equation_tag", content = "values")]
pub(crate) enum IrEquation {
    Equals { value: Vec<String> },
    NotEquals { value: String },
    BitwiseAnd { value: Vec<String> },
}

impl From<&Equation> for IrEquation {
    fn from(v: &Equation) -> Self {
        match v {
            Equation::Equals { values: value } => IrEquation::Equals {
                value: value.clone(),
            },
            Equation::NotEquals { value } => IrEquation::NotEquals {
                value: value.clone(),
            },
            Equation::BitwiseAnd { values: value } => IrEquation::BitwiseAnd {
                value: value.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrConditional {
    variable_name: String,
    equations: IrEquation,
}

impl From<Conditional> for IrConditional {
    fn from(v: Conditional) -> Self {
        let equations = v.equation().into();

        Self {
            variable_name: v.variable_name().to_string(),
            equations,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrIfStatement {
    pub conditional: IrConditional,
    members: Vec<IrStructMember>,
    else_if_statements: Vec<IrIfStatement>,
    else_members: Vec<IrStructMember>,
    original_type: IrType,
    part_of_separate_if_statement: bool,
    is_else_if_flag: bool,
}

impl From<&IfStatement> for IrIfStatement {
    fn from(v: &IfStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();
        let else_ifs = v.else_ifs().iter().map(|a| a.into()).collect();

        let else_statement_members = v.else_members().iter().map(|a| a.into()).collect();

        Self {
            conditional: v.conditional().clone().into(),
            members,
            else_if_statements: else_ifs,
            else_members: else_statement_members,
            original_type: v.original_ty().into(),
            part_of_separate_if_statement: v.part_of_separate_if_statement(),
            is_else_if_flag: v.is_elseif_flag(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrStructMemberDefinition {
    name: String,
    data_type: IrType,
    constant_value: Option<IrIntegerEnumValue>,
    used_as_size_in: Option<String>,
    size_of_fields_before_size: Option<u8>,
    used_in_if: bool,
    tags: IrTags,
}

impl From<&StructMemberDefinition> for IrStructMemberDefinition {
    fn from(v: &StructMemberDefinition) -> Self {
        Self {
            name: v.name().to_string(),
            data_type: v.ty().into(),
            constant_value: v.value().as_ref().map(|a| a.into()),
            used_as_size_in: v.used_as_size_in().clone(),
            size_of_fields_before_size: v.size_of_fields_before_size().map(|a| a as u8),
            used_in_if: v.used_in_if(),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "data_type_tag", content = "content")]
pub(crate) enum IrType {
    Integer(IrIntegerType),
    Bool(IrIntegerType),
    DateTime,
    PackedGuid,
    Guid,
    NamedGuid,
    FloatingPoint,
    CString,
    SizedCString,
    String,
    Array(IrArray),
    Enum {
        type_name: String,
        integer_type: IrIntegerType,
        upcast: bool,
    },
    Flag {
        type_name: String,
        integer_type: IrIntegerType,
        upcast: bool,
    },
    Struct {
        type_name: String,
        sizes: IrSizes,
    },
    UpdateMask,
    AuraMask,
    MonsterMoveSpline,
    AchievementDoneArray,
    AchievementInProgressArray,
    EnchantMask,
    InspectTalentGearMask,
    Gold,
    Level,
    Level16,
    Level32,
    VariableItemRandomProperty,
    AddonArray,
    IpAddress,
    Seconds,
    Milliseconds,
}

impl From<&Type> for IrType {
    fn from(v: &Type) -> Self {
        match v {
            Type::Integer(i) => Self::Integer(IrIntegerType::from_integer_type(i)),
            Type::PackedGuid => Self::PackedGuid,
            Type::Guid => Self::Guid,
            Type::NamedGuid => Self::NamedGuid,
            Type::FloatingPoint => Self::FloatingPoint,
            Type::CString => Self::CString,
            Type::String => Self::String,
            Type::UpdateMask { .. } => Self::UpdateMask,
            Type::AuraMask => Self::AuraMask,
            Type::Array(array) => Self::Array(array.into()),
            Type::Enum { e, upcast } => {
                let (upcast, integer_type) = if let Some(c) = upcast {
                    (true, IrIntegerType::from_integer_type(c))
                } else {
                    (false, IrIntegerType::from_integer_type(e.ty()))
                };

                Self::Enum {
                    type_name: e.name().to_string(),
                    integer_type,
                    upcast,
                }
            }
            Type::Flag { e, upcast } => {
                let (upcast, integer_type) = if let Some(c) = upcast {
                    (true, IrIntegerType::from_integer_type(c))
                } else {
                    (false, IrIntegerType::from_integer_type(e.ty()))
                };

                Self::Flag {
                    type_name: e.name().to_string(),
                    integer_type,
                    upcast,
                }
            }
            Type::Struct { e } => Self::Struct {
                type_name: e.name().to_string(),
                sizes: IrSizes::from_sizes(e.sizes()),
            },
            Type::SizedCString => Self::SizedCString,
            Type::Bool(i) => Self::Bool(IrIntegerType::from_integer_type(i)),
            Type::DateTime => Self::DateTime,
            Type::AchievementDoneArray => Self::AchievementDoneArray,
            Type::AchievementInProgressArray => Self::AchievementInProgressArray,
            Type::MonsterMoveSplines => Self::MonsterMoveSpline,
            Type::EnchantMask => Self::EnchantMask,
            Type::InspectTalentGearMask => Self::InspectTalentGearMask,
            Type::Gold => Self::Gold,
            Type::Level => Self::Level,
            Type::Level16 => Self::Level16,
            Type::Level32 => Self::Level32,
            Type::VariableItemRandomProperty => Self::VariableItemRandomProperty,
            Type::AddonArray => Self::AddonArray,
            Type::IpAddress => Self::IpAddress,
            Type::Seconds => Self::Seconds,
            Type::Milliseconds => Self::Milliseconds,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrArray {
    inner_type: IrArrayType,
    size: IrArraySize,
}

impl From<&Array> for IrArray {
    fn from(v: &Array) -> Self {
        Self {
            inner_type: v.ty().into(),
            size: v.size().into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "array_type_tag", content = "inner_type")]
pub(crate) enum IrArrayType {
    Integer(IrIntegerType),
    Struct(String),
    CString,
    Guid,
    PackedGuid,
}

impl From<&ArrayType> for IrArrayType {
    fn from(v: &ArrayType) -> Self {
        match v {
            ArrayType::Integer(i) => Self::Integer(IrIntegerType::from_integer_type(i)),
            ArrayType::Struct(f) => Self::Struct(f.name().into()),
            ArrayType::CString => Self::CString,
            ArrayType::Guid => Self::Guid,
            ArrayType::PackedGuid => Self::PackedGuid,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "array_size_tag", content = "size")]
pub(crate) enum IrArraySize {
    Fixed(String),
    Variable(String),
    Endless,
}

impl From<ArraySize> for IrArraySize {
    fn from(v: ArraySize) -> Self {
        match v {
            ArraySize::Fixed(s) => Self::Fixed(s.to_string()),
            ArraySize::Variable(s) => Self::Variable(s.name().into()),
            ArraySize::Endless => Self::Endless,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrIntegerEnumValue {
    pub value: String,
    pub original_string: String,
}

impl From<&ContainerValue> for IrIntegerEnumValue {
    fn from(v: &ContainerValue) -> Self {
        Self {
            value: v.value().to_string(),
            original_string: v.original_string().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrTestCase {
    subject: String,
    members: Vec<IrTestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: IrTags,
    file_info: IrFileInfo,
}

impl From<&&TestCase> for IrTestCase {
    fn from(v: &&TestCase) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            subject: v.subject().to_string(),
            members,
            raw_bytes: v.raw_bytes().to_vec(),
            tags: IrTags::from_tags(v.tags()),
            file_info: IrFileInfo {
                file_name: v.file_info().name().to_string(),
                start_position: v.file_info().start_line() as u32,
                end_position: v.file_info().end_line() as u32,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrTestCaseMember {
    variable_name: String,
    value: IrTestValue,
    tags: IrTags,
}

impl From<&TestCaseMember> for IrTestCaseMember {
    fn from(v: &TestCaseMember) -> Self {
        Self {
            variable_name: v.name().to_string(),
            value: v.value().into(),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrTestUpdateMaskValue {
    update_mask_type: IrUpdateMaskType,
    update_mask_name: String,
    update_mask_value: String,
}

impl From<&TestUpdateMaskValue> for IrTestUpdateMaskValue {
    fn from(e: &TestUpdateMaskValue) -> Self {
        Self {
            update_mask_type: e.ty().into(),
            update_mask_name: e.name().to_string(),
            update_mask_value: e.value().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) enum IrUpdateMaskType {
    Object,
    Item,
    Unit,
    Player,
    Container,
    GameObject,
    DynamicObject,
    Corpse,
}

impl From<UpdateMaskType> for IrUpdateMaskType {
    fn from(e: UpdateMaskType) -> Self {
        match e {
            UpdateMaskType::Object => Self::Object,
            UpdateMaskType::Item => Self::Item,
            UpdateMaskType::Unit => Self::Unit,
            UpdateMaskType::Player => Self::Player,
            UpdateMaskType::Container => Self::Container,
            UpdateMaskType::GameObject => Self::GameObject,
            UpdateMaskType::DynamicObject => Self::DynamicObject,
            UpdateMaskType::Corpse => Self::Corpse,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "test_value_tag", content = "content")]
pub(crate) enum IrTestValue {
    Integer(IrIntegerEnumValue),
    Bool(bool),
    DateTime(IrIntegerEnumValue),
    Guid(IrIntegerEnumValue),
    FloatingPoint {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<String>,
        size: IrArraySize,
    },
    String(String),
    Flag(Vec<String>),
    Enum(IrIntegerEnumValue),
    SubObject {
        type_name: String,
        members: Vec<IrTestCaseMember>,
    },
    ArrayOfSubObject {
        type_name: String,
        members: Vec<Vec<IrTestCaseMember>>,
    },
    UpdateMask(Vec<IrTestUpdateMaskValue>),
    IpAddress(IrIntegerEnumValue),
    Seconds(IrIntegerEnumValue),
    Milliseconds(IrIntegerEnumValue),
}

impl From<&TestValue> for IrTestValue {
    fn from(v: &TestValue) -> Self {
        match v {
            TestValue::Number(i) => Self::Integer(i.into()),
            TestValue::DateTime(v) => Self::DateTime(v.into()),
            TestValue::Guid(i) => Self::Guid(i.into()),
            TestValue::Bool(b) => Self::Bool(*b),
            TestValue::IpAddress(v) => IrTestValue::IpAddress(v.into()),
            TestValue::FloatingNumber {
                value,
                original_string,
            } => Self::FloatingPoint {
                value: *value,
                original_string: original_string.to_string(),
            },
            TestValue::Array { values, size } => Self::Array {
                values: values.iter().map(|a| a.to_string()).collect(),
                size: size.clone().into(),
            },
            TestValue::String(s) => Self::String(s.to_string()),
            TestValue::Flag(f) => Self::Flag(f.to_vec()),
            TestValue::Enum(e) => Self::Enum(e.into()),
            TestValue::SubObject { c, members } => Self::SubObject {
                type_name: c.name().to_string(),
                members: members.iter().map(|a| a.into()).collect(),
            },
            TestValue::ArrayOfSubObject(e, t) => Self::ArrayOfSubObject {
                type_name: e.name().to_string(),
                members: t
                    .iter()
                    .map(|a| a.iter().map(|a| a.into()).collect::<Vec<_>>())
                    .collect(),
            },
            TestValue::UpdateMask(v) => {
                IrTestValue::UpdateMask(v.iter().map(|a| a.into()).collect())
            }
            TestValue::Seconds(i) => Self::Seconds(i.into()),
            TestValue::Milliseconds(i) => Self::Milliseconds(i.into()),
        }
    }
}

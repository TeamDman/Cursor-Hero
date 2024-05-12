use bevy::reflect::Reflect;
use bevy::reflect::ReflectRef;
use bevy::reflect::TypeInfo;
use bevy::reflect::VariantType;

/// Returns `None` if the thing isn't an enum.
///
/// Example:
/// ```
/// let x = display_enum_qualified_variant_instance(CalculatorElementKind::DigitButton(4))
/// assert_eq!(x, Some("CalculatorElementKind::DigitButton(4)".to_owned()));
/// ```
pub fn display_enum_qualified_variant_instance(thing: &dyn Reflect) -> Option<String> {
    let ReflectRef::Enum(value) = thing.reflect_ref() else {
        return None;
    };
    Some(format!("{}::{:?}", value.reflect_short_type_path(), thing))
}

pub fn display_enum_unqualified_variant_definition(thing: &dyn Reflect) -> Option<String> {
    let ReflectRef::Enum(value) = thing.reflect_ref() else {
        return None;
    };

    let definition = match value.variant_type() {
        VariantType::Struct => todo!(),
        VariantType::Tuple => {
            let base = value.variant_name();
            let num_fields = value.field_len();
            let fields = (0..num_fields)
                .map(|i| {
                    value
                        .field_at(i)
                        .and_then(|field| field.get_represented_type_info())
                        .map(|info| {
                            match info {
                                TypeInfo::Struct(inner_info) => inner_info.type_path_table(),
                                TypeInfo::TupleStruct(inner_info) => inner_info.type_path_table(),
                                TypeInfo::Tuple(inner_info) => inner_info.type_path_table(),
                                TypeInfo::List(inner_info) => inner_info.type_path_table(),
                                TypeInfo::Array(inner_info) => inner_info.type_path_table(),
                                TypeInfo::Map(inner_info) => inner_info.type_path_table(),
                                TypeInfo::Enum(inner_info) => inner_info.type_path_table(),
                                TypeInfo::Value(inner_info) => inner_info.type_path_table(),
                            }
                            .short_path()
                            .to_owned()
                        })
                        .unwrap_or_else(|| format!("field_{}_unsupported", i))
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", base, fields)
        }
        VariantType::Unit => value.variant_name().to_owned(),
    };
    Some(definition)
}

#[cfg(test)]
mod test {
    #[derive(Reflect)]
    enum CalculatorElementKind {
        PlusButton,
        DigitButton(u8),
        Bruh((u8, bevy::math::Vec3)),
    }
    use super::*;

    #[test]
    fn variant_instance() {
        let elem_kind = CalculatorElementKind::PlusButton;
        let qualified_name = display_enum_qualified_variant_instance(&elem_kind).unwrap();
        assert_eq!(qualified_name, "CalculatorElementKind::PlusButton");
    }
    #[test]
    fn variant_instance_parameterized() {
        let elem_kind = CalculatorElementKind::DigitButton(4);
        let qualified_name = display_enum_qualified_variant_instance(&elem_kind).unwrap();
        assert_eq!(qualified_name, "CalculatorElementKind::DigitButton(4)");
    }

    #[test]
    fn variant_definition() {
        let elem_kind = CalculatorElementKind::PlusButton;
        let definition = display_enum_unqualified_variant_definition(&elem_kind).unwrap();
        assert_eq!(definition, "PlusButton");
    }
    #[test]
    fn variant_definition_parameterized() {
        let elem_kind = CalculatorElementKind::Bruh((4, bevy::math::Vec3::new(1.0, 2.0, 3.0)));
        let definition = display_enum_unqualified_variant_definition(&elem_kind).unwrap();
        assert_eq!(definition, "Bruh((u8, Vec3))");
    }
}

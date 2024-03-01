// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_iam_instance_profile_input_input_input(
    input: &crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssociateIamInstanceProfile", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("IamInstanceProfile");
    if let Some(var_2) = &input.iam_instance_profile {
        crate::protocol_serde::shape_iam_instance_profile_specification::ser_iam_instance_profile_specification(scope_1, var_2)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceId");
    if let Some(var_4) = &input.instance_id {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

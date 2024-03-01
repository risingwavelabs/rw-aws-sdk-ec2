// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AllocateHosts`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_placement(AutoPlacement)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::auto_placement) / [`set_auto_placement(Option<AutoPlacement>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_auto_placement):<br>required: **false**<br><p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>  <p>Default: <code>on</code> </p><br>
    ///   - [`availability_zone(impl Into<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::availability_zone) / [`set_availability_zone(Option<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_availability_zone):<br>required: **true**<br><p>The Availability Zone in which to allocate the Dedicated Host.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`instance_type(impl Into<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::instance_type) / [`set_instance_type(Option<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_instance_type):<br>required: **false**<br><p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>  <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p><br>
    ///   - [`instance_family(impl Into<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::instance_family) / [`set_instance_family(Option<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_instance_family):<br>required: **false**<br><p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>  <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p><br>
    ///   - [`quantity(i32)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::quantity) / [`set_quantity(Option<i32>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_quantity):<br>required: **false**<br><p>The number of Dedicated Hosts to allocate to your account with these parameters. If you are allocating the Dedicated Hosts on an Outpost, and you specify <b>AssetIds</b>, you can omit this parameter. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset. If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value that you specify for <b>Quantity</b> must be equal to the number of asset IDs specified.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the Dedicated Host during creation.</p><br>
    ///   - [`host_recovery(HostRecovery)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::host_recovery) / [`set_host_recovery(Option<HostRecovery>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_host_recovery):<br>required: **false**<br><p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>  <p>Default: <code>off</code> </p><br>
    ///   - [`outpost_arn(impl Into<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::outpost_arn) / [`set_outpost_arn(Option<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_outpost_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host. If you specify <b>OutpostArn</b>, you can optionally specify <b>AssetIds</b>.</p>  <p>If you are allocating the Dedicated Host in a Region, omit this parameter.</p><br>
    ///   - [`host_maintenance(HostMaintenance)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::host_maintenance) / [`set_host_maintenance(Option<HostMaintenance>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_host_maintenance):<br>required: **false**<br><p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p><br>
    ///   - [`asset_ids(impl Into<String>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::asset_ids) / [`set_asset_ids(Option<Vec::<String>>)`](crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::set_asset_ids):<br>required: **false**<br><p>The IDs of the Outpost hardware assets on which to allocate the Dedicated Hosts. Targeting specific hardware assets on an Outpost can help to minimize latency between your workloads. This parameter is supported only if you specify <b>OutpostArn</b>. If you are allocating the Dedicated Hosts in a Region, omit this parameter.</p>  <ul>   <li> <p>If you specify this parameter, you can omit <b>Quantity</b>. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset.</p> </li>   <li> <p>If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value for <b>Quantity</b> must be equal to the number of asset IDs specified.</p> </li>  </ul><br>
    /// - On success, responds with [`AllocateHostsOutput`](crate::operation::allocate_hosts::AllocateHostsOutput) with field(s):
    ///   - [`host_ids(Option<Vec::<String>>)`](crate::operation::allocate_hosts::AllocateHostsOutput::host_ids): <p>The ID of the allocated Dedicated Host. This is used to launch an instance onto a specific host.</p>
    /// - On failure, responds with [`SdkError<AllocateHostsError>`](crate::operation::allocate_hosts::AllocateHostsError)
    pub fn allocate_hosts(&self) -> crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder {
        crate::operation::allocate_hosts::builders::AllocateHostsFluentBuilder::new(self.handle.clone())
    }
}
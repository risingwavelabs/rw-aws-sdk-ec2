// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    #[allow(dead_code)] // unused when a service does not provide any operations
    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
}

/// Client for Amazon Elastic Compute Cloud
///
/// Client for invoking operations on Amazon Elastic Compute Cloud. Each operation on Amazon Elastic Compute Cloud is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
/// ## Constructing a `Client`
///
/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
/// crate should be used to automatically resolve this config using
/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
/// across multiple different AWS SDK clients. This config resolution process can be customized
/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
/// the [builder pattern] to customize the default config.
///
/// In the simplest case, creating a client looks as follows:
/// ```rust,no_run
/// # async fn wrapper() {
/// let config = aws_config::load_from_env().await;
/// let client = aws_sdk_ec2::Client::new(&config);
/// # }
/// ```
///
/// Occasionally, SDKs may have additional service-specific that can be set on the [`Config`] that
/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
/// The [`Config`] struct implements `From<&SdkConfig>`, so setting these specific settings can be
/// done as follows:
///
/// ```rust,no_run
/// # async fn wrapper() {
/// let sdk_config = ::aws_config::load_from_env().await;
/// let config = aws_sdk_ec2::config::Builder::from(&sdk_config)
/// # /*
///     .some_service_specific_setting("value")
/// # */
///     .build();
/// # }
/// ```
///
/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
///
/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
/// be done once at application start-up.
///
/// [`Config`]: crate::Config
/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
/// [`aws-config` docs]: https://docs.rs/aws-config/*
/// [`aws-config`]: https://crates.io/crates/aws-config
/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
/// # Using the `Client`
///
/// A client has a function for every operation that can be performed by the service.
/// For example, the [`AcceptAddressTransfer`](crate::operation::accept_address_transfer) operation has
/// a [`Client::accept_address_transfer`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
///
/// ```rust,ignore
/// let result = client.accept_address_transfer()
///     .address("example")
///     .send()
///     .await;
/// ```
///
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    handle: ::std::sync::Arc<Handle>,
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// This method will panic in the following cases:
    ///
    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
    /// - No `behavior_version` is provided.
    ///
    /// The panic message for each of these will have instructions on how to resolve them.
    #[track_caller]
    pub fn from_conf(conf: crate::Config) -> Self {
        let handle = Handle {
            conf: conf.clone(),
            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
        };
        if let Err(err) = Self::validate_config(&handle) {
            panic!("Invalid client configuration: {err}");
        }
        Self {
            handle: ::std::sync::Arc::new(handle),
        }
    }

    /// Returns the client's configuration.
    pub fn config(&self) -> &crate::Config {
        &self.handle.conf
    }

    fn validate_config(handle: &Handle) -> Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        handle
            .runtime_plugins
            .apply_client_configuration(&mut cfg)?
            .validate_base_client_config(&cfg)?;
        Ok(())
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.
    #[track_caller]
    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }
}

mod accept_address_transfer;

mod accept_reserved_instances_exchange_quote;

mod accept_transit_gateway_multicast_domain_associations;

mod accept_transit_gateway_peering_attachment;

mod accept_transit_gateway_vpc_attachment;

mod accept_vpc_endpoint_connections;

mod accept_vpc_peering_connection;

mod advertise_byoip_cidr;

mod allocate_address;

mod allocate_hosts;

mod allocate_ipam_pool_cidr;

mod apply_security_groups_to_client_vpn_target_network;

mod assign_ipv6_addresses;

mod assign_private_ip_addresses;

mod assign_private_nat_gateway_address;

mod associate_address;

mod associate_client_vpn_target_network;

mod associate_dhcp_options;

mod associate_enclave_certificate_iam_role;

mod associate_iam_instance_profile;

mod associate_instance_event_window;

mod associate_ipam_byoasn;

mod associate_ipam_resource_discovery;

mod associate_nat_gateway_address;

mod associate_route_table;

mod associate_subnet_cidr_block;

mod associate_transit_gateway_multicast_domain;

mod associate_transit_gateway_policy_table;

mod associate_transit_gateway_route_table;

mod associate_trunk_interface;

mod associate_vpc_cidr_block;

mod attach_classic_link_vpc;

mod attach_internet_gateway;

mod attach_network_interface;

mod attach_verified_access_trust_provider;

mod attach_volume;

mod attach_vpn_gateway;

mod authorize_client_vpn_ingress;

mod authorize_security_group_egress;

mod authorize_security_group_ingress;

mod bundle_instance;

mod cancel_bundle_task;

mod cancel_capacity_reservation;

mod cancel_capacity_reservation_fleets;

mod cancel_conversion_task;

mod cancel_export_task;

mod cancel_image_launch_permission;

mod cancel_import_task;

mod cancel_reserved_instances_listing;

mod cancel_spot_fleet_requests;

mod cancel_spot_instance_requests;

mod confirm_product_instance;

mod copy_fpga_image;

mod copy_image;

mod copy_snapshot;

mod create_capacity_reservation;

mod create_capacity_reservation_fleet;

mod create_carrier_gateway;

mod create_client_vpn_endpoint;

mod create_client_vpn_route;

mod create_coip_cidr;

mod create_coip_pool;

mod create_customer_gateway;

mod create_default_subnet;

mod create_default_vpc;

mod create_dhcp_options;

mod create_egress_only_internet_gateway;

mod create_fleet;

mod create_flow_logs;

mod create_fpga_image;

mod create_image;

mod create_instance_connect_endpoint;

mod create_instance_event_window;

mod create_instance_export_task;

mod create_internet_gateway;

mod create_ipam;

mod create_ipam_pool;

mod create_ipam_resource_discovery;

mod create_ipam_scope;

mod create_key_pair;

mod create_launch_template;

mod create_launch_template_version;

mod create_local_gateway_route;

mod create_local_gateway_route_table;

mod create_local_gateway_route_table_virtual_interface_group_association;

mod create_local_gateway_route_table_vpc_association;

mod create_managed_prefix_list;

mod create_nat_gateway;

mod create_network_acl;

mod create_network_acl_entry;

mod create_network_insights_access_scope;

mod create_network_insights_path;

mod create_network_interface;

mod create_network_interface_permission;

mod create_placement_group;

mod create_public_ipv4_pool;

mod create_replace_root_volume_task;

mod create_reserved_instances_listing;

mod create_restore_image_task;

mod create_route;

mod create_route_table;

mod create_security_group;

mod create_snapshot;

mod create_snapshots;

mod create_spot_datafeed_subscription;

mod create_store_image_task;

mod create_subnet;

mod create_subnet_cidr_reservation;

mod create_tags;

mod create_traffic_mirror_filter;

mod create_traffic_mirror_filter_rule;

mod create_traffic_mirror_session;

mod create_traffic_mirror_target;

mod create_transit_gateway;

mod create_transit_gateway_connect;

mod create_transit_gateway_connect_peer;

mod create_transit_gateway_multicast_domain;

mod create_transit_gateway_peering_attachment;

mod create_transit_gateway_policy_table;

mod create_transit_gateway_prefix_list_reference;

mod create_transit_gateway_route;

mod create_transit_gateway_route_table;

mod create_transit_gateway_route_table_announcement;

mod create_transit_gateway_vpc_attachment;

mod create_verified_access_endpoint;

mod create_verified_access_group;

mod create_verified_access_instance;

mod create_verified_access_trust_provider;

mod create_volume;

mod create_vpc;

mod create_vpc_endpoint;

mod create_vpc_endpoint_connection_notification;

mod create_vpc_endpoint_service_configuration;

mod create_vpc_peering_connection;

mod create_vpn_connection;

mod create_vpn_connection_route;

mod create_vpn_gateway;

/// Operation customization and supporting types.
///
/// The underlying HTTP requests made during an operation can be customized
/// by calling the `customize()` method on the builder returned from a client
/// operation call. For example, this can be used to add an additional HTTP header:
///
/// ```ignore
/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_ec2::Error> {
/// # let client: aws_sdk_ec2::Client = unimplemented!();
/// use ::http::header::{HeaderName, HeaderValue};
///
/// let result = client.accept_address_transfer()
///     .customize()
///     .mutate_request(|req| {
///         // Add `x-example-header` with value
///         req.headers_mut()
///             .insert(
///                 HeaderName::from_static("x-example-header"),
///                 HeaderValue::from_static("1"),
///             );
///     })
///     .send()
///     .await;
/// # }
/// ```
pub mod customize;

mod delete_carrier_gateway;

mod delete_client_vpn_endpoint;

mod delete_client_vpn_route;

mod delete_coip_cidr;

mod delete_coip_pool;

mod delete_customer_gateway;

mod delete_dhcp_options;

mod delete_egress_only_internet_gateway;

mod delete_fleets;

mod delete_flow_logs;

mod delete_fpga_image;

mod delete_instance_connect_endpoint;

mod delete_instance_event_window;

mod delete_internet_gateway;

mod delete_ipam;

mod delete_ipam_pool;

mod delete_ipam_resource_discovery;

mod delete_ipam_scope;

mod delete_key_pair;

mod delete_launch_template;

mod delete_launch_template_versions;

mod delete_local_gateway_route;

mod delete_local_gateway_route_table;

mod delete_local_gateway_route_table_virtual_interface_group_association;

mod delete_local_gateway_route_table_vpc_association;

mod delete_managed_prefix_list;

mod delete_nat_gateway;

mod delete_network_acl;

mod delete_network_acl_entry;

mod delete_network_insights_access_scope;

mod delete_network_insights_access_scope_analysis;

mod delete_network_insights_analysis;

mod delete_network_insights_path;

mod delete_network_interface;

mod delete_network_interface_permission;

mod delete_placement_group;

mod delete_public_ipv4_pool;

mod delete_queued_reserved_instances;

mod delete_route;

mod delete_route_table;

mod delete_security_group;

mod delete_snapshot;

mod delete_spot_datafeed_subscription;

mod delete_subnet;

mod delete_subnet_cidr_reservation;

mod delete_tags;

mod delete_traffic_mirror_filter;

mod delete_traffic_mirror_filter_rule;

mod delete_traffic_mirror_session;

mod delete_traffic_mirror_target;

mod delete_transit_gateway;

mod delete_transit_gateway_connect;

mod delete_transit_gateway_connect_peer;

mod delete_transit_gateway_multicast_domain;

mod delete_transit_gateway_peering_attachment;

mod delete_transit_gateway_policy_table;

mod delete_transit_gateway_prefix_list_reference;

mod delete_transit_gateway_route;

mod delete_transit_gateway_route_table;

mod delete_transit_gateway_route_table_announcement;

mod delete_transit_gateway_vpc_attachment;

mod delete_verified_access_endpoint;

mod delete_verified_access_group;

mod delete_verified_access_instance;

mod delete_verified_access_trust_provider;

mod delete_volume;

mod delete_vpc;

mod delete_vpc_endpoint_connection_notifications;

mod delete_vpc_endpoint_service_configurations;

mod delete_vpc_endpoints;

mod delete_vpc_peering_connection;

mod delete_vpn_connection;

mod delete_vpn_connection_route;

mod delete_vpn_gateway;

mod deprovision_byoip_cidr;

mod deprovision_ipam_byoasn;

mod deprovision_ipam_pool_cidr;

mod deprovision_public_ipv4_pool_cidr;

mod deregister_image;

mod deregister_instance_event_notification_attributes;

mod deregister_transit_gateway_multicast_group_members;

mod deregister_transit_gateway_multicast_group_sources;

mod describe_account_attributes;

mod describe_address_transfers;

mod describe_addresses;

mod describe_addresses_attribute;

mod describe_aggregate_id_format;

mod describe_availability_zones;

mod describe_aws_network_performance_metric_subscriptions;

mod describe_bundle_tasks;

mod describe_byoip_cidrs;

mod describe_capacity_block_offerings;

mod describe_capacity_reservation_fleets;

mod describe_capacity_reservations;

mod describe_carrier_gateways;

mod describe_classic_link_instances;

mod describe_client_vpn_authorization_rules;

mod describe_client_vpn_connections;

mod describe_client_vpn_endpoints;

mod describe_client_vpn_routes;

mod describe_client_vpn_target_networks;

mod describe_coip_pools;

mod describe_conversion_tasks;

mod describe_customer_gateways;

mod describe_dhcp_options;

mod describe_egress_only_internet_gateways;

mod describe_elastic_gpus;

mod describe_export_image_tasks;

mod describe_export_tasks;

mod describe_fast_launch_images;

mod describe_fast_snapshot_restores;

mod describe_fleet_history;

mod describe_fleet_instances;

mod describe_fleets;

mod describe_flow_logs;

mod describe_fpga_image_attribute;

mod describe_fpga_images;

mod describe_host_reservation_offerings;

mod describe_host_reservations;

mod describe_hosts;

mod describe_iam_instance_profile_associations;

mod describe_id_format;

mod describe_identity_id_format;

mod describe_image_attribute;

mod describe_images;

mod describe_import_image_tasks;

mod describe_import_snapshot_tasks;

mod describe_instance_attribute;

mod describe_instance_connect_endpoints;

mod describe_instance_credit_specifications;

mod describe_instance_event_notification_attributes;

mod describe_instance_event_windows;

mod describe_instance_status;

mod describe_instance_topology;

mod describe_instance_type_offerings;

mod describe_instance_types;

mod describe_instances;

mod describe_internet_gateways;

mod describe_ipam_byoasn;

mod describe_ipam_pools;

mod describe_ipam_resource_discoveries;

mod describe_ipam_resource_discovery_associations;

mod describe_ipam_scopes;

mod describe_ipams;

mod describe_ipv6_pools;

mod describe_key_pairs;

mod describe_launch_template_versions;

mod describe_launch_templates;

mod describe_local_gateway_route_table_virtual_interface_group_associations;

mod describe_local_gateway_route_table_vpc_associations;

mod describe_local_gateway_route_tables;

mod describe_local_gateway_virtual_interface_groups;

mod describe_local_gateway_virtual_interfaces;

mod describe_local_gateways;

mod describe_locked_snapshots;

mod describe_managed_prefix_lists;

mod describe_moving_addresses;

mod describe_nat_gateways;

mod describe_network_acls;

mod describe_network_insights_access_scope_analyses;

mod describe_network_insights_access_scopes;

mod describe_network_insights_analyses;

mod describe_network_insights_paths;

mod describe_network_interface_attribute;

mod describe_network_interface_permissions;

mod describe_network_interfaces;

mod describe_placement_groups;

mod describe_prefix_lists;

mod describe_principal_id_format;

mod describe_public_ipv4_pools;

mod describe_regions;

mod describe_replace_root_volume_tasks;

mod describe_reserved_instances;

mod describe_reserved_instances_listings;

mod describe_reserved_instances_modifications;

mod describe_reserved_instances_offerings;

mod describe_route_tables;

mod describe_scheduled_instance_availability;

mod describe_scheduled_instances;

mod describe_security_group_references;

mod describe_security_group_rules;

mod describe_security_groups;

mod describe_snapshot_attribute;

mod describe_snapshot_tier_status;

mod describe_snapshots;

mod describe_spot_datafeed_subscription;

mod describe_spot_fleet_instances;

mod describe_spot_fleet_request_history;

mod describe_spot_fleet_requests;

mod describe_spot_instance_requests;

mod describe_spot_price_history;

mod describe_stale_security_groups;

mod describe_store_image_tasks;

mod describe_subnets;

mod describe_tags;

mod describe_traffic_mirror_filters;

mod describe_traffic_mirror_sessions;

mod describe_traffic_mirror_targets;

mod describe_transit_gateway_attachments;

mod describe_transit_gateway_connect_peers;

mod describe_transit_gateway_connects;

mod describe_transit_gateway_multicast_domains;

mod describe_transit_gateway_peering_attachments;

mod describe_transit_gateway_policy_tables;

mod describe_transit_gateway_route_table_announcements;

mod describe_transit_gateway_route_tables;

mod describe_transit_gateway_vpc_attachments;

mod describe_transit_gateways;

mod describe_trunk_interface_associations;

mod describe_verified_access_endpoints;

mod describe_verified_access_groups;

mod describe_verified_access_instance_logging_configurations;

mod describe_verified_access_instances;

mod describe_verified_access_trust_providers;

mod describe_volume_attribute;

mod describe_volume_status;

mod describe_volumes;

mod describe_volumes_modifications;

mod describe_vpc_attribute;

mod describe_vpc_classic_link;

mod describe_vpc_classic_link_dns_support;

mod describe_vpc_endpoint_connection_notifications;

mod describe_vpc_endpoint_connections;

mod describe_vpc_endpoint_service_configurations;

mod describe_vpc_endpoint_service_permissions;

mod describe_vpc_endpoint_services;

mod describe_vpc_endpoints;

mod describe_vpc_peering_connections;

mod describe_vpcs;

mod describe_vpn_connections;

mod describe_vpn_gateways;

mod detach_classic_link_vpc;

mod detach_internet_gateway;

mod detach_network_interface;

mod detach_verified_access_trust_provider;

mod detach_volume;

mod detach_vpn_gateway;

mod disable_address_transfer;

mod disable_aws_network_performance_metric_subscription;

mod disable_ebs_encryption_by_default;

mod disable_fast_launch;

mod disable_fast_snapshot_restores;

mod disable_image;

mod disable_image_block_public_access;

mod disable_image_deprecation;

mod disable_ipam_organization_admin_account;

mod disable_serial_console_access;

mod disable_snapshot_block_public_access;

mod disable_transit_gateway_route_table_propagation;

mod disable_vgw_route_propagation;

mod disable_vpc_classic_link;

mod disable_vpc_classic_link_dns_support;

mod disassociate_address;

mod disassociate_client_vpn_target_network;

mod disassociate_enclave_certificate_iam_role;

mod disassociate_iam_instance_profile;

mod disassociate_instance_event_window;

mod disassociate_ipam_byoasn;

mod disassociate_ipam_resource_discovery;

mod disassociate_nat_gateway_address;

mod disassociate_route_table;

mod disassociate_subnet_cidr_block;

mod disassociate_transit_gateway_multicast_domain;

mod disassociate_transit_gateway_policy_table;

mod disassociate_transit_gateway_route_table;

mod disassociate_trunk_interface;

mod disassociate_vpc_cidr_block;

mod enable_address_transfer;

mod enable_aws_network_performance_metric_subscription;

mod enable_ebs_encryption_by_default;

mod enable_fast_launch;

mod enable_fast_snapshot_restores;

mod enable_image;

mod enable_image_block_public_access;

mod enable_image_deprecation;

mod enable_ipam_organization_admin_account;

mod enable_reachability_analyzer_organization_sharing;

mod enable_serial_console_access;

mod enable_snapshot_block_public_access;

mod enable_transit_gateway_route_table_propagation;

mod enable_vgw_route_propagation;

mod enable_volume_io;

mod enable_vpc_classic_link;

mod enable_vpc_classic_link_dns_support;

mod export_client_vpn_client_certificate_revocation_list;

mod export_client_vpn_client_configuration;

mod export_image;

mod export_transit_gateway_routes;

mod get_associated_enclave_certificate_iam_roles;

mod get_associated_ipv6_pool_cidrs;

mod get_aws_network_performance_data;

mod get_capacity_reservation_usage;

mod get_coip_pool_usage;

mod get_console_output;

mod get_console_screenshot;

mod get_default_credit_specification;

mod get_ebs_default_kms_key_id;

mod get_ebs_encryption_by_default;

mod get_flow_logs_integration_template;

mod get_groups_for_capacity_reservation;

mod get_host_reservation_purchase_preview;

mod get_image_block_public_access_state;

mod get_instance_types_from_instance_requirements;

mod get_instance_uefi_data;

mod get_ipam_address_history;

mod get_ipam_discovered_accounts;

mod get_ipam_discovered_public_addresses;

mod get_ipam_discovered_resource_cidrs;

mod get_ipam_pool_allocations;

mod get_ipam_pool_cidrs;

mod get_ipam_resource_cidrs;

mod get_launch_template_data;

mod get_managed_prefix_list_associations;

mod get_managed_prefix_list_entries;

mod get_network_insights_access_scope_analysis_findings;

mod get_network_insights_access_scope_content;

mod get_password_data;

mod get_reserved_instances_exchange_quote;

mod get_security_groups_for_vpc;

mod get_serial_console_access_status;

mod get_snapshot_block_public_access_state;

mod get_spot_placement_scores;

mod get_subnet_cidr_reservations;

mod get_transit_gateway_attachment_propagations;

mod get_transit_gateway_multicast_domain_associations;

mod get_transit_gateway_policy_table_associations;

mod get_transit_gateway_policy_table_entries;

mod get_transit_gateway_prefix_list_references;

mod get_transit_gateway_route_table_associations;

mod get_transit_gateway_route_table_propagations;

mod get_verified_access_endpoint_policy;

mod get_verified_access_group_policy;

mod get_vpn_connection_device_sample_configuration;

mod get_vpn_connection_device_types;

mod get_vpn_tunnel_replacement_status;

mod import_client_vpn_client_certificate_revocation_list;

mod import_image;

mod import_instance;

mod import_key_pair;

mod import_snapshot;

mod import_volume;

mod list_images_in_recycle_bin;

mod list_snapshots_in_recycle_bin;

mod lock_snapshot;

mod modify_address_attribute;

mod modify_availability_zone_group;

mod modify_capacity_reservation;

mod modify_capacity_reservation_fleet;

mod modify_client_vpn_endpoint;

mod modify_default_credit_specification;

mod modify_ebs_default_kms_key_id;

mod modify_fleet;

mod modify_fpga_image_attribute;

mod modify_hosts;

mod modify_id_format;

mod modify_identity_id_format;

mod modify_image_attribute;

mod modify_instance_attribute;

mod modify_instance_capacity_reservation_attributes;

mod modify_instance_credit_specification;

mod modify_instance_event_start_time;

mod modify_instance_event_window;

mod modify_instance_maintenance_options;

mod modify_instance_metadata_options;

mod modify_instance_placement;

mod modify_ipam;

mod modify_ipam_pool;

mod modify_ipam_resource_cidr;

mod modify_ipam_resource_discovery;

mod modify_ipam_scope;

mod modify_launch_template;

mod modify_local_gateway_route;

mod modify_managed_prefix_list;

mod modify_network_interface_attribute;

mod modify_private_dns_name_options;

mod modify_reserved_instances;

mod modify_security_group_rules;

mod modify_snapshot_attribute;

mod modify_snapshot_tier;

mod modify_spot_fleet_request;

mod modify_subnet_attribute;

mod modify_traffic_mirror_filter_network_services;

mod modify_traffic_mirror_filter_rule;

mod modify_traffic_mirror_session;

mod modify_transit_gateway;

mod modify_transit_gateway_prefix_list_reference;

mod modify_transit_gateway_vpc_attachment;

mod modify_verified_access_endpoint;

mod modify_verified_access_endpoint_policy;

mod modify_verified_access_group;

mod modify_verified_access_group_policy;

mod modify_verified_access_instance;

mod modify_verified_access_instance_logging_configuration;

mod modify_verified_access_trust_provider;

mod modify_volume;

mod modify_volume_attribute;

mod modify_vpc_attribute;

mod modify_vpc_endpoint;

mod modify_vpc_endpoint_connection_notification;

mod modify_vpc_endpoint_service_configuration;

mod modify_vpc_endpoint_service_payer_responsibility;

mod modify_vpc_endpoint_service_permissions;

mod modify_vpc_peering_connection_options;

mod modify_vpc_tenancy;

mod modify_vpn_connection;

mod modify_vpn_connection_options;

mod modify_vpn_tunnel_certificate;

mod modify_vpn_tunnel_options;

mod monitor_instances;

mod move_address_to_vpc;

mod move_byoip_cidr_to_ipam;

mod provision_byoip_cidr;

mod provision_ipam_byoasn;

mod provision_ipam_pool_cidr;

mod provision_public_ipv4_pool_cidr;

mod purchase_capacity_block;

mod purchase_host_reservation;

mod purchase_reserved_instances_offering;

mod purchase_scheduled_instances;

mod reboot_instances;

mod register_image;

mod register_instance_event_notification_attributes;

mod register_transit_gateway_multicast_group_members;

mod register_transit_gateway_multicast_group_sources;

mod reject_transit_gateway_multicast_domain_associations;

mod reject_transit_gateway_peering_attachment;

mod reject_transit_gateway_vpc_attachment;

mod reject_vpc_endpoint_connections;

mod reject_vpc_peering_connection;

mod release_address;

mod release_hosts;

mod release_ipam_pool_allocation;

mod replace_iam_instance_profile_association;

mod replace_network_acl_association;

mod replace_network_acl_entry;

mod replace_route;

mod replace_route_table_association;

mod replace_transit_gateway_route;

mod replace_vpn_tunnel;

mod report_instance_status;

mod request_spot_fleet;

mod request_spot_instances;

mod reset_address_attribute;

mod reset_ebs_default_kms_key_id;

mod reset_fpga_image_attribute;

mod reset_image_attribute;

mod reset_instance_attribute;

mod reset_network_interface_attribute;

mod reset_snapshot_attribute;

mod restore_address_to_classic;

mod restore_image_from_recycle_bin;

mod restore_managed_prefix_list_version;

mod restore_snapshot_from_recycle_bin;

mod restore_snapshot_tier;

mod revoke_client_vpn_ingress;

mod revoke_security_group_egress;

mod revoke_security_group_ingress;

mod run_instances;

mod run_scheduled_instances;

mod search_local_gateway_routes;

mod search_transit_gateway_multicast_groups;

mod search_transit_gateway_routes;

mod send_diagnostic_interrupt;

mod start_instances;

mod start_network_insights_access_scope_analysis;

mod start_network_insights_analysis;

mod start_vpc_endpoint_service_private_dns_verification;

mod stop_instances;

mod terminate_client_vpn_connections;

mod terminate_instances;

mod unassign_ipv6_addresses;

mod unassign_private_ip_addresses;

mod unassign_private_nat_gateway_address;

mod unlock_snapshot;

mod unmonitor_instances;

mod update_security_group_rule_descriptions_egress;

mod update_security_group_rule_descriptions_ingress;

mod withdraw_byoip_cidr;

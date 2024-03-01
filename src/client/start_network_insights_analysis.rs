// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartNetworkInsightsAnalysis`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_insights_path_id(impl Into<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::network_insights_path_id) / [`set_network_insights_path_id(Option<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_network_insights_path_id):<br>required: **true**<br><p>The ID of the path.</p><br>
    ///   - [`additional_accounts(impl Into<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::additional_accounts) / [`set_additional_accounts(Option<Vec::<String>>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_additional_accounts):<br>required: **false**<br><p>The member accounts that contain resources that the path can traverse.</p><br>
    ///   - [`filter_in_arns(impl Into<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::filter_in_arns) / [`set_filter_in_arns(Option<Vec::<String>>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_filter_in_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARN) of the resources that the path must traverse.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::set_client_token):<br>required: **true**<br><p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p><br>
    /// - On success, responds with [`StartNetworkInsightsAnalysisOutput`](crate::operation::start_network_insights_analysis::StartNetworkInsightsAnalysisOutput) with field(s):
    ///   - [`network_insights_analysis(Option<NetworkInsightsAnalysis>)`](crate::operation::start_network_insights_analysis::StartNetworkInsightsAnalysisOutput::network_insights_analysis): <p>Information about the network insights analysis.</p>
    /// - On failure, responds with [`SdkError<StartNetworkInsightsAnalysisError>`](crate::operation::start_network_insights_analysis::StartNetworkInsightsAnalysisError)
    pub fn start_network_insights_analysis(
        &self,
    ) -> crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder {
        crate::operation::start_network_insights_analysis::builders::StartNetworkInsightsAnalysisFluentBuilder::new(self.handle.clone())
    }
}
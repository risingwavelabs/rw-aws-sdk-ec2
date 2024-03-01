// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSpotDatafeedSubscription`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the Amazon S3 bucket in which to store the Spot Instance data feed. For more information about bucket names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html#bucketnamingrules">Rules for bucket naming</a> in the <i>Amazon S3 Developer Guide</i>.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`prefix(impl Into<String>)`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::prefix) / [`set_prefix(Option<String>)`](crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::set_prefix):<br>required: **false**<br><p>The prefix for the data feed file names.</p><br>
    /// - On success, responds with [`CreateSpotDatafeedSubscriptionOutput`](crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscriptionOutput) with field(s):
    ///   - [`spot_datafeed_subscription(Option<SpotDatafeedSubscription>)`](crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscriptionOutput::spot_datafeed_subscription): <p>The Spot Instance data feed subscription.</p>
    /// - On failure, responds with [`SdkError<CreateSpotDatafeedSubscriptionError>`](crate::operation::create_spot_datafeed_subscription::CreateSpotDatafeedSubscriptionError)
    pub fn create_spot_datafeed_subscription(
        &self,
    ) -> crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder {
        crate::operation::create_spot_datafeed_subscription::builders::CreateSpotDatafeedSubscriptionFluentBuilder::new(self.handle.clone())
    }
}
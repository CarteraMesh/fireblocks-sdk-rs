# PolicyRuleAuthorizationGroups

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**logic** | Option<**Logic**> | * AND - requires approval of all authorization groups * OR - requires approval of at least one of the authorization groups  (enum: AND, OR) | [optional]
**allow_operator_as_authorizer** | Option<**bool**> | Defines whether the user who initiates a transaction can approve their own transaction and count toward the approval threshold for their transaction | [optional]
**groups** | Option<[**Vec<models::PolicyRuleAuthorizationGroupsGroupsInner>**](PolicyRuleAuthorizationGroupsGroupsInner.md)> | Groups of entities which can approve the transaction | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



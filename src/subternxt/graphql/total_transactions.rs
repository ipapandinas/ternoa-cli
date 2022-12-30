use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/subternxt/graphql/schemas/mainnet_dictionary_schema.json",
    query_path = "src/subternxt/graphql/queries/total_transactions.graphql",
    response_derives = "Debug"
)]
pub struct TotalTransactions;

pub async fn get_total_transactions(network: String) -> Result<i64, Box<dyn Error>> {
    println!("Get total transactions: This feature is only available for the mainnet");
    let request_body = TotalTransactions::build_query(total_transactions::Variables);

    let client = reqwest::Client::new();
    let res = client.post(network).json(&request_body).send().await?;
    let response_body: Response<total_transactions::ResponseData> = res.json().await?;

    // response_body.data: Response { data: Some(ResponseData { extrinsics: Some(TotalTransactionsExtrinsics { total_count: nnnnn }) }), errors: None }
    // response_body.data.extrinsics: Some(TotalTransactionsExtrinsics { total_count: nnnn })
    let total_count = if let Some(ref response_data) = response_body.data {
        //println!("{:?}", response_data.extrinsics);

        if let Some(total_txns) = &response_data.extrinsics {
            total_txns.total_count
        } else {
            0
        }
    } else {
        0
    };

    Ok(total_count)
}

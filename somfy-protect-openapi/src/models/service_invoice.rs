/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInvoice {
    /// Invoice identifier.
    #[serde(rename = "invoice_id")]
    pub invoice_id: String,
    /// Paid date.
    #[serde(rename = "paid_on")]
    pub paid_on: String,
    /// Invoice subtotal.
    #[serde(rename = "sub_total")]
    pub sub_total: i32,
    /// Invoice tax.
    #[serde(rename = "tax")]
    pub tax: i32,
    /// Tax included?
    #[serde(rename = "tax_inclusive")]
    pub tax_inclusive: bool,
    /// Invoice total amount.
    #[serde(rename = "amount")]
    pub amount: i32,
    /// Invoice currency.
    #[serde(rename = "currency")]
    pub currency: String,
    /// Invoice Link to download PDF.
    #[serde(rename = "invoice_url")]
    pub invoice_url: String,
}

impl ServiceInvoice {
    pub fn new(
        invoice_id: String,
        paid_on: String,
        sub_total: i32,
        tax: i32,
        tax_inclusive: bool,
        amount: i32,
        currency: String,
        invoice_url: String,
    ) -> ServiceInvoice {
        ServiceInvoice {
            invoice_id,
            paid_on,
            sub_total,
            tax,
            tax_inclusive,
            amount,
            currency,
            invoice_url,
        }
    }
}

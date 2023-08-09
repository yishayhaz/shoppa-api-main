pub mod error_code;
use shoppa_core::{file_storage::StorageClient, email_sender::EmailClient, payments::PaymentClient, invoice_service::InvoiceClient};
use strum_macros::{Display, EnumString};

use axum::Extension;
use std::sync::Arc;

pub type AxumStorgeClientExtension = Extension<Arc<StorageClient>>;
pub type AxumEmailClientExtension = Extension<Arc<EmailClient>>;
pub type AxumPaymentClientExtension = Extension<Arc<PaymentClient>>;
pub type AxumInvoiceClientExtension = Extension<Arc<InvoiceClient>>;


#[derive(EnumString, Display)]
pub enum Cookeys {
    #[strum(to_string = "bribed_pigeon_here")]
    AccessToken,
    #[strum(to_string = "a_delicious_pigeon")]
    CsrfToken,
    #[strum(to_string = "familiar_pigeon")]
    VisitIndicator,
    #[strum(to_string = "lab_pigeon")]
    DebugingCookie,
    #[strum(to_string = "government_pigeon")]
    StoreUserAccessToken,
    #[strum(to_string = "pigeon_who_loves_to_eat_cookies")]
    CheckoutSession,
    #[strum(to_string = "number_of_pigeons")]
    OrderNumber,
}

#[derive(EnumString, Display)]
pub enum HeadKeys {
    #[strum(to_string = "x-top_secret_pigeon")]
    CsrfToken,
}

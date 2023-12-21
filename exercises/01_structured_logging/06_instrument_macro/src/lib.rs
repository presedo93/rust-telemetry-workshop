//! # Ergonomics
//!
//! Before I stated that we shouldn't treat each function as its own unit of work—it's way
//! too verbose and it leads to a lot of noise in the logs.
//! Nonetheless, it is often the case that a unit of work maps to **a certain** function. The work
//! starts when that function is invoked and ends when it returns. That was indeed the case in the
//! previous exercise.
//!
//! When it happens, we can use the `instrument` macro to avoid having to manually create a span
//! inside the function body.
//!
//! # Exercise
//!
//! Read the `instrument` macro documentation: https://docs.rs/tracing/latest/tracing/attr.instrument.html
//! Then use it to complete the same exercise as before, instead of manually creating and entering
//! spans.
//!
//! Tip: you can use `Span::current()` to get a reference to the current span from inside
//! the function body.
mod subscriber;

pub use subscriber::init_test_subscriber;
use tracing::{field::Empty, instrument, Span};

/// Given a list of order numbers, compute the total price.
#[instrument(name = "process total price", skip(order_numbers), fields( outcome = Empty ))]
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let mut total = 0;
    for order_number in order_numbers {
        let order_details = match get_order_details(*order_number) {
            Ok(od) => od,
            Err(e) => {
                Span::current().record("outcome", "failure");
                return Err(e);
            }
        };
        total += order_details.price;
    }
    
    Span::current().record("outcome", "success");
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
#[instrument(name = "retrieve order", skip(order_number), fields( outcome = Empty ))]
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    if order_number % 4 == 0 {
        Span::current().record("outcome", "failure");
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        Span::current().record("outcome", "success");
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}

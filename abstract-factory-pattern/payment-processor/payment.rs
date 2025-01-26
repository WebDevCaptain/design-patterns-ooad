// Abstract Products ----
trait PaymentProcessor {
    fn process_payment(&self, amount: f64) -> String;
}

trait RefundProcessor {
    fn process_refund(&self, amount: f64) -> String;
}

// Concrete Products ----

// Paypal
struct PaypalPaymentProcessor;
struct PaypalRefundProcessor;

impl PaymentProcessor for PaypalPaymentProcessor {
    fn process_payment(&self, amount: f64) -> String {
        format!("Processing payment of {} using Paypal", amount)
    }
}

impl RefundProcessor for PaypalRefundProcessor {
    fn process_refund(&self, amount: f64) -> String {
        format!("Processing refund of {} using Paypal", amount)
    }
}

// Stripe
struct StripePaymentProcessor;
struct StripeRefundProcessor;

impl PaymentProcessor for StripePaymentProcessor {
    fn process_payment(&self, amount: f64) -> String {
        format!("Processing payment of {} using Stripe", amount)
    }
}

impl RefundProcessor for StripeRefundProcessor {
    fn process_refund(&self, amount: f64) -> String {
        format!("Processing refund of {} using Stripe", amount)
    }
}

// Abstract Factory ----
trait PaymentFactory {
    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor>;
    fn create_refund_processor(&self) -> Box<dyn RefundProcessor>;
}

// Concrete Factory ----
struct PaypalFactory;
struct StripeFactory;

impl PaymentFactory for PaypalFactory {
    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(PaypalPaymentProcessor)
    }

    fn create_refund_processor(&self) -> Box<dyn RefundProcessor> {
        Box::new(PaypalRefundProcessor)
    }
}

impl PaymentFactory for StripeFactory {
    fn create_payment_processor(&self) -> Box<dyn PaymentProcessor> {
        Box::new(StripePaymentProcessor)
    }

    fn create_refund_processor(&self) -> Box<dyn RefundProcessor> {
        Box::new(StripeRefundProcessor)
    }
}

// Client side code
fn process_transactions(factory: &dyn PaymentFactory, payment_amt: f64, refund_amt: f64) {
    let payment_processor = factory.create_payment_processor();
    let refund_processor = factory.create_refund_processor();

    println!("{}", payment_processor.process_payment(payment_amt));
    println!("{}", refund_processor.process_refund(refund_amt));
}

fn main() {
    println!("Using Paypal for payment and refund:");

    let paypal_factory = PaypalFactory;
    process_transactions(&paypal_factory, 100.0, 50.0);

    println!("\nUsing Stripe for payment and refund:");

    let stripe_factory = StripeFactory;
    process_transactions(&stripe_factory, 200.0, 75.0);
}

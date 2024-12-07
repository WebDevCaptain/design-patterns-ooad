from abc import ABC, abstractmethod


# ======= Abstract Products =========
class PaymentProcessor(ABC):
    @abstractmethod
    def process_payment(self, amount: float) -> str:
        pass


class RefundProcessor(ABC):
    @abstractmethod
    def process_refund(self, amount: float) -> str:
        pass


# ==== Concrete Products =====
class PayPalPaymentProcessor(PaymentProcessor):
    def process_payment(self, amount: float) -> str:
        return f"Processed payment of ${amount} via PayPal."


class PayPalRefundProcessor(RefundProcessor):
    def process_refund(self, amount: float) -> str:
        return f"Processed refund of ${amount} via PayPal."


class StripePaymentProcessor(PaymentProcessor):
    def process_payment(self, amount: float) -> str:
        return f"Processed payment of ${amount} via Stripe."


class StripeRefundProcessor(RefundProcessor):
    def process_refund(self, amount: float) -> str:
        return f"Processed refund of ${amount} via Stripe."


# Abstract Factory
class PaymentFactory(ABC):
    @abstractmethod
    def create_payment_processor(self) -> PaymentProcessor:
        pass

    @abstractmethod
    def create_refund_processor(self) -> RefundProcessor:
        pass


# Concrete Factory: PayPalFactory
class PayPalFactory(PaymentFactory):
    def create_payment_processor(self) -> PaymentProcessor:
        return PayPalPaymentProcessor()

    def create_refund_processor(self) -> RefundProcessor:
        return PayPalRefundProcessor()


# Concrete Factory: StripeFactory
class StripeFactory(PaymentFactory):
    def create_payment_processor(self) -> PaymentProcessor:
        return StripePaymentProcessor()

    def create_refund_processor(self) -> RefundProcessor:
        return StripeRefundProcessor()


# Usage
def process_transactions(
    factory: PaymentFactory, payment_amt: float, refund_amt: float
):
    payment_processor = factory.create_payment_processor()
    refund_processor = factory.create_refund_processor()

    print(payment_processor.process_payment(payment_amt))
    print(refund_processor.process_refund(refund_amt))


if __name__ == "__main__":
    print("Using PayPal:")
    paypal_factory = PayPalFactory()
    process_transactions(paypal_factory, payment_amt=100.0, refund_amt=50.0)

    print("\nUsing Stripe:")
    stripe_factory = StripeFactory()
    process_transactions(stripe_factory, payment_amt=200.0, refund_amt=75.0)

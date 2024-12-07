interface PaymentProcessor {
  processPayment(amount: number): string;
}

interface RefundProcessor {
  processRefund(amount: number): string;
}

class PaypalPaymentProcessor implements PaymentProcessor {
  processPayment(amount: number): string {
    return `Processed payment of ${amount} via Paypal`;
  }
}

class PaypalRefundProcessor implements RefundProcessor {
  processRefund(amount: number): string {
    return `Processed refund of ${amount} via Paypal.`;
  }
}

interface PaymentFactory {
  createPaymentProcessor(): PaymentProcessor;
  createRefundProcessor(): RefundProcessor;
}

class PaypalFactory implements PaymentFactory {
  createPaymentProcessor(): PaymentProcessor {
    return new PaypalPaymentProcessor();
  }

  createRefundProcessor(): RefundProcessor {
    return new PaypalRefundProcessor();
  }
}

// Use
function processTransactions(
  fact: PaymentFactory,
  paymentAmount: number,
  refundAmount: number
) {
  const paymentProcessor = fact.createPaymentProcessor();
  const refundProcessor = fact.createRefundProcessor();

  console.log(paymentProcessor.processPayment(paymentAmount));
  console.log(refundProcessor.processRefund(refundAmount));
}

const paypalFactory = new PaypalFactory();
processTransactions(paypalFactory, 1200, 200);

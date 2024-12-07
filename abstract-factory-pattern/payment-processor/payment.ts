// ======= Abstract Products =========
interface PaymentProcessor {
  processPayment(amount: number): string;
}

interface RefundProcessor {
  processRefund(amount: number): string;
}

// ============== Concrete Products =======================
// Paypal
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

// Razorpay
class RazorpayPaymentProcessor implements PaymentProcessor {
  processPayment(amount: number): string {
    return `Processed payment of ${amount}$ via Razorpay.`;
  }
}

class RazorpayRefundProcessor implements RefundProcessor {
  processRefund(amount: number): string {
    return `Processed refund of ${amount}$ via Razorpay`;
  }
}

// ______________ Abstract Factory ______________
interface PaymentFactory {
  createPaymentProcessor(): PaymentProcessor;
  createRefundProcessor(): RefundProcessor;
}

// =============== Concrete Factory =========================
// Paypal factory
class PaypalFactory implements PaymentFactory {
  createPaymentProcessor(): PaymentProcessor {
    return new PaypalPaymentProcessor();
  }

  createRefundProcessor(): RefundProcessor {
    return new PaypalRefundProcessor();
  }
}

// Razorpay factory
class RazorpayFactory implements PaymentFactory {
  createPaymentProcessor(): PaymentProcessor {
    return new RazorpayPaymentProcessor();
  }

  createRefundProcessor(): RefundProcessor {
    return new RazorpayRefundProcessor();
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

const razorpayFactory = new RazorpayFactory();
processTransactions(razorpayFactory, 2100, 700);

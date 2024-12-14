interface Payload {
  [key: string]: number;
}

abstract class ReportGenerator {
  generateReport() {
    const data = this.fetchData();
    const formattedData = this.formatData(data);
    this.printReport(formattedData);
  }

  abstract fetchData(): Payload;

  abstract formatData(data: Payload): string;

  printReport(formattedData: string) {
    console.log("Report: ", formattedData);
  }
}

class SalesReport extends ReportGenerator {
  fetchData(): Payload {
    return { sales: 1200, profit: 800 };
  }

  formatData(data: Payload): string {
    return `Sales: ${data.sales}, Profit: ${data.profit}`;
  }
}

class UserActivityReport extends ReportGenerator {
  fetchData(): Payload {
    return { users: 200, active: 30 };
  }

  formatData(data: Payload): string {
    return `Total Users: ${data.users}, Active users: ${data.active}`;
  }
}

// Dry run
let report = new SalesReport();
report.generateReport();

report = new UserActivityReport();
report.generateReport();

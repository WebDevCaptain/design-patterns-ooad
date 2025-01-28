trait ReportGenerator {
    // Template method: defines the skeleton of the algorithm
    fn generate_report(&self) {
        let data = self.fetch_data();
        let formatted_data = self.format_data(&data);
        self.print_report(&formatted_data);
    }

    // Abstract method to fetch data
    fn fetch_data(&self) -> String;
    fn format_data(&self, data: &str) -> String;

    fn print_report(&self, formatted_data: &str) {
        println!("Report:\n{}", formatted_data);
    }
}

// Concrete implementations of ReportGenerator
struct SalesReport;

impl ReportGenerator for SalesReport {
    fn fetch_data(&self) -> String {
        "Sales data".to_string()
    }

    fn format_data(&self, data: &str) -> String {
        format!("Sales: 1000$, Profit: 200$, data = {}", data)
    }
}

struct UserActivityReport;

impl ReportGenerator for UserActivityReport {
    fn fetch_data(&self) -> String {
        "User activity data".to_string()
    }

    fn format_data(&self, data: &str) -> String {
        format!("Active users: 1000, Inactive users: 200, data = {}", data)
    }
}

fn main() {
    let sales_report = SalesReport;
    sales_report.generate_report();

    let user_activity_report = UserActivityReport;
    user_activity_report.generate_report();
}

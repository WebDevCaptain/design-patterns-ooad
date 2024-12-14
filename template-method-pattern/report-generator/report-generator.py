from abc import ABC, abstractmethod


# Abstract class defining the template
class ReportGenerator(ABC):
    def generate_report(self):
        """Template method: defines the skeleton of the algorithm."""
        data = self.fetch_data()
        formatted_data = self.format_data(data)
        self.print_report(formatted_data)

    @abstractmethod
    def fetch_data(self):
        """Abstract method to fetch data."""
        pass

    @abstractmethod
    def format_data(self, data):
        """Abstract method to format data."""
        pass

    def print_report(self, formatted_data):
        """Concrete method to print the report."""
        print("Report:")
        print(formatted_data, end="\n\n")


# Concrete implementation 1: Fetch and format sales data
class SalesReport(ReportGenerator):
    def fetch_data(self):
        return {"sales": 1000, "profit": 200}

    def format_data(self, data):
        return f"Sales: ${data['sales']}, Profit: ${data['profit']}"


# Concrete implementation 2: Fetch and format user activity data
class UserActivityReport(ReportGenerator):
    def fetch_data(self):
        return {"users": 150, "active": 50}

    def format_data(self, data):
        return f"Total Users: {data['users']}, Active Users: {data['active']}"


# Sample usage
if __name__ == "__main__":
    report = SalesReport()
    report.generate_report()

    report = UserActivityReport()
    report.generate_report()

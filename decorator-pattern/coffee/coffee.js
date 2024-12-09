var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
// Concrete component
var BasicCoffee = /** @class */ (function () {
    function BasicCoffee() {
    }
    BasicCoffee.prototype.cost = function () {
        return 5;
    };
    BasicCoffee.prototype.description = function () {
        return "Basic coffee";
    };
    return BasicCoffee;
}());
// Decorator (base)
var CoffeeDecorator = /** @class */ (function () {
    function CoffeeDecorator(c) {
        this.coffee = c;
    }
    return CoffeeDecorator;
}());
// --------- Concrete Decorators -------------
var MilkDecorator = /** @class */ (function (_super) {
    __extends(MilkDecorator, _super);
    function MilkDecorator() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    MilkDecorator.prototype.cost = function () {
        return this.coffee.cost() + 1.5;
    };
    MilkDecorator.prototype.description = function () {
        return this.coffee.description() + ", Milk";
    };
    return MilkDecorator;
}(CoffeeDecorator));
var SugarDecorator = /** @class */ (function (_super) {
    __extends(SugarDecorator, _super);
    function SugarDecorator() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    SugarDecorator.prototype.cost = function () {
        return this.coffee.cost() + 0.5;
    };
    SugarDecorator.prototype.description = function () {
        return this.coffee.description() + ", Sugar";
    };
    return SugarDecorator;
}(CoffeeDecorator));
// Usage
var coffee = new BasicCoffee();
console.log("".concat(coffee.description(), " -- ").concat(coffee.cost()));
var milkCoffee = new MilkDecorator(coffee);
console.log("".concat(milkCoffee.description(), " -- ").concat(milkCoffee.cost()));
var milkAndSugarCoffee = new SugarDecorator(milkCoffee);
console.log("".concat(milkAndSugarCoffee.description(), " -- ").concat(milkAndSugarCoffee.cost()));

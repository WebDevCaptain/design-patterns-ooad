/**
 * Concrete Flyweight class representing a circle.
 */
var Circle = /** @class */ (function () {
    function Circle(radius) {
        this.radius = radius;
    }
    Circle.prototype.draw = function () {
        console.log("Drawing a circle with a radius of ".concat(this.radius));
    };
    return Circle;
}());
/**
 * Flyweight Factory to manage and reuse shapes.
 */
var ShapeFactory = /** @class */ (function () {
    function ShapeFactory() {
        this.shapes = {};
    }
    ShapeFactory.prototype.getCircle = function (radius) {
        if (!this.shapes[radius]) {
            this.shapes[radius] = new Circle(radius);
            console.log("Creating a new circle with radius ".concat(radius));
        }
        return this.shapes[radius];
    };
    return ShapeFactory;
}());
// Usage
var shapeFactory = new ShapeFactory();
var c1 = shapeFactory.getCircle(12);
c1.draw();
var c2 = shapeFactory.getCircle(12);
c2.draw();
var c3 = shapeFactory.getCircle(32);
c3.draw();

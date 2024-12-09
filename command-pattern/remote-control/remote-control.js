var Light = /** @class */ (function () {
    function Light() {
    }
    Light.prototype.on = function () {
        console.log("Light turned ON.");
    };
    Light.prototype.off = function () {
        console.log("Light turned OFF.");
    };
    return Light;
}());
var LightOnCommand = /** @class */ (function () {
    function LightOnCommand(light) {
        this.light = light;
    }
    LightOnCommand.prototype.execute = function () {
        this.light.on();
    };
    return LightOnCommand;
}());
var LightOffCommand = /** @class */ (function () {
    function LightOffCommand(light) {
        this.light = light;
    }
    LightOffCommand.prototype.execute = function () {
        this.light.off();
    };
    return LightOffCommand;
}());
var RemoteControl = /** @class */ (function () {
    function RemoteControl() {
        this.command = null;
    }
    RemoteControl.prototype.setCommand = function (command) {
        this.command = command;
    };
    RemoteControl.prototype.pressButton = function () {
        if (this.command) {
            this.command.execute();
        }
    };
    return RemoteControl;
}());
// Usage
var light = new Light();
var lightOnCommand = new LightOnCommand(light);
var lightOffCommand = new LightOffCommand(light);
var remote = new RemoteControl();
// Turning on lights
remote.setCommand(lightOnCommand);
remote.pressButton();
// Enough, now turn it off
setTimeout(function () {
    remote.setCommand(lightOffCommand);
    remote.pressButton();
}, 2000);
